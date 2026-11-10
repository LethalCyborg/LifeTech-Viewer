//! macOS readings of `sys_probe`: the process's memory, its threads with
//! their CPU time and names, and the machine's and each core's load, from
//! the Mach calls the system's Activity Monitor uses (`task_info`,
//! `task_threads`, `thread_info`, `host_statistics`,
//! `host_processor_info`). Declared here rather than through the `libc`
//! crate, which marks its Mach bindings deprecated. Times are nanoseconds,
//! as on Linux (`thread_cpu::cycles_per_ms`).

use super::{ProcessMemory, ThreadTime};

#[allow(non_camel_case_types)]
type mach_port_t = u32;
#[allow(non_camel_case_types)]
type kern_return_t = i32;

const KERN_SUCCESS: kern_return_t = 0;
const MACH_TASK_BASIC_INFO: i32 = 20;
const TASK_VM_INFO: i32 = 22;
const THREAD_IDENTIFIER_INFO: i32 = 4;
const THREAD_EXTENDED_INFO: i32 = 5;
const HOST_CPU_LOAD_INFO: i32 = 3;
const PROCESSOR_CPU_LOAD_INFO: i32 = 2;
/// Index of the idle ticks in a `cpu_ticks` array (user, system, idle, nice).
const CPU_STATE_IDLE: usize = 2;

unsafe extern "C" {
    static mach_task_self_: mach_port_t;
    fn mach_host_self() -> mach_port_t;
    fn task_info(task: mach_port_t, flavor: i32, info: *mut i32, count: *mut u32) -> kern_return_t;
    fn task_threads(task: mach_port_t, list: *mut *mut mach_port_t, count: *mut u32) -> kern_return_t;
    fn thread_info(thread: mach_port_t, flavor: i32, info: *mut i32, count: *mut u32) -> kern_return_t;
    fn host_statistics(host: mach_port_t, flavor: i32, info: *mut i32, count: *mut u32) -> kern_return_t;
    fn host_processor_info(host: mach_port_t, flavor: i32, cpus: *mut u32, info: *mut *mut i32, count: *mut u32) -> kern_return_t;
    fn mach_port_deallocate(task: mach_port_t, name: mach_port_t) -> kern_return_t;
    fn vm_deallocate(task: mach_port_t, address: usize, size: usize) -> kern_return_t;
}

fn task() -> mach_port_t {
    // SAFETY: the kernel fills this global at the process's start
    // (`mach_task_self()` is a macro reading it).
    unsafe { mach_task_self_ }
}

/// `struct mach_task_basic_info` (mach/task_info.h, under `#pragma pack(4)`).
#[repr(C, packed(4))]
#[derive(Default)]
struct TaskBasicInfo {
    virtual_size: u64,
    resident_size: u64,
    resident_size_max: u64,
    user_time: [i32; 2],
    system_time: [i32; 2],
    policy: i32,
    suspend_count: i32,
}

/// `struct task_vm_info` up to `phys_footprint` (TASK_VM_INFO_REV1_COUNT):
/// the memory Activity Monitor shows for the process.
#[repr(C, packed(4))]
#[derive(Default)]
struct TaskVmInfo {
    virtual_size: u64,
    region_count: i32,
    page_size: i32,
    resident_size: u64,
    resident_size_peak: u64,
    device: u64,
    device_peak: u64,
    internal: u64,
    internal_peak: u64,
    external: u64,
    external_peak: u64,
    reusable: u64,
    reusable_peak: u64,
    purgeable_volatile_pmap: u64,
    purgeable_volatile_resident: u64,
    purgeable_volatile_virtual: u64,
    compressed: u64,
    compressed_peak: u64,
    compressed_lifetime: u64,
    phys_footprint: u64,
}

/// `struct thread_identifier_info` (mach/thread_info.h).
#[repr(C)]
#[derive(Default)]
struct ThreadIdentifierInfo {
    thread_id: u64,
    thread_handle: u64,
    dispatch_qaddr: u64,
}

/// `struct thread_extended_info` (mach/thread_info.h): times in
/// nanoseconds and the thread's name.
#[repr(C)]
struct ThreadExtendedInfo {
    pth_user_time: u64,
    pth_system_time: u64,
    pth_cpu_usage: i32,
    pth_policy: i32,
    pth_run_state: i32,
    pth_flags: i32,
    pth_sleep_time: i32,
    pth_curpri: i32,
    pth_priority: i32,
    pth_maxpriority: i32,
    pth_name: [u8; 64],
}

/// The count `*_info` calls take: the structure's size in 32-bit words.
fn words<T>() -> u32 {
    (std::mem::size_of::<T>() / 4) as u32
}

pub fn current_thread_id() -> u32 {
    let mut id = 0u64;
    // SAFETY: a null thread is the calling thread; the call writes one u64.
    unsafe { libc::pthread_threadid_np(0, &mut id) };
    id as u32
}

pub fn process_memory() -> Option<ProcessMemory> {
    let mut basic = TaskBasicInfo::default();
    let mut count = words::<TaskBasicInfo>();
    // SAFETY: the structure is sized as `count` says; the call writes at
    // most that many words.
    if unsafe { task_info(task(), MACH_TASK_BASIC_INFO, (&raw mut basic).cast(), &mut count) } != KERN_SUCCESS {
        return None;
    }
    let mut vm = TaskVmInfo::default();
    let mut count = words::<TaskVmInfo>();
    // SAFETY: as above.
    let footprint = if unsafe { task_info(task(), TASK_VM_INFO, (&raw mut vm).cast(), &mut count) } == KERN_SUCCESS {
        vm.phys_footprint
    } else {
        basic.resident_size
    };
    // SAFETY: plain-old-data structure, all zeroes is a valid value.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    // SAFETY: the call writes one rusage for the calling process.
    let faults = if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } == 0 {
        (usage.ru_minflt as u64).wrapping_add(usage.ru_majflt as u64)
    } else {
        0
    };
    Some(ProcessMemory {
        working_set: basic.resident_size,
        peak_working_set: basic.resident_size_max.max(basic.resident_size),
        // the footprint: what the process holds, compressed or not
        private: footprint,
        page_faults: faults as u32,
    })
}

/// Every thread with its CPU time (ns), and the whole process's.
pub fn read_threads() -> (Vec<ThreadTime>, u64) {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: the call writes one timespec; the process's clock exists.
    let process = if unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut t) } == 0 {
        (t.tv_sec as u64).wrapping_mul(1_000_000_000).wrapping_add(t.tv_nsec as u64)
    } else {
        0
    };
    let mut out = Vec::new();
    let mut list: *mut mach_port_t = std::ptr::null_mut();
    let mut count = 0u32;
    // SAFETY: the kernel allocates `list` (count ports); each port is
    // released and the list freed below.
    unsafe {
        if task_threads(task(), &mut list, &mut count) != KERN_SUCCESS || list.is_null() {
            return (out, process);
        }
        for i in 0..count as usize {
            let port = *list.add(i);
            let mut ident = ThreadIdentifierInfo::default();
            let mut n = words::<ThreadIdentifierInfo>();
            let id_ok = thread_info(port, THREAD_IDENTIFIER_INFO, (&raw mut ident).cast(), &mut n) == KERN_SUCCESS;
            let mut ext: ThreadExtendedInfo = std::mem::zeroed();
            let mut n = words::<ThreadExtendedInfo>();
            let ext_ok = thread_info(port, THREAD_EXTENDED_INFO, (&raw mut ext).cast(), &mut n) == KERN_SUCCESS;
            mach_port_deallocate(task(), port);
            if !(id_ok && ext_ok) {
                continue;
            }
            let len = ext.pth_name.iter().position(|&b| b == 0).unwrap_or(ext.pth_name.len());
            out.push(ThreadTime {
                id: ident.thread_id as u32,
                name: String::from_utf8_lossy(&ext.pth_name[..len]).into_owned(),
                cycles: ext.pth_user_time.wrapping_add(ext.pth_system_time),
            });
        }
        vm_deallocate(task(), list as usize, count as usize * std::mem::size_of::<mach_port_t>());
    }
    (out, process)
}

/// The machine's (idle, total) ticks.
pub fn system_times() -> Option<(u64, u64)> {
    let mut ticks = [0u32; 4];
    let mut count = 4u32;
    // SAFETY: HOST_CPU_LOAD_INFO writes four tick counters.
    let ok = unsafe { host_statistics(mach_host_self(), HOST_CPU_LOAD_INFO, ticks.as_mut_ptr().cast(), &mut count) } == KERN_SUCCESS;
    ok.then(|| idle_total(&ticks))
}

/// Each core's (idle, total) ticks.
pub fn core_times() -> Vec<(u64, u64)> {
    let mut cpus = 0u32;
    let mut info: *mut i32 = std::ptr::null_mut();
    let mut count = 0u32;
    // SAFETY: the kernel allocates `info` (count words: four tick counters
    // per core), read here then freed.
    unsafe {
        if host_processor_info(mach_host_self(), PROCESSOR_CPU_LOAD_INFO, &mut cpus, &mut info, &mut count) != KERN_SUCCESS
            || info.is_null()
        {
            return Vec::new();
        }
        let words = std::slice::from_raw_parts(info.cast::<u32>(), count as usize);
        let out = words.chunks_exact(4).take(cpus as usize).map(idle_total).collect();
        vm_deallocate(task(), info as usize, count as usize * 4);
        out
    }
}

fn idle_total(ticks: &[u32]) -> (u64, u64) {
    let total = ticks.iter().map(|&t| u64::from(t)).sum();
    (u64::from(ticks[CPU_STATE_IDLE]), total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_structures_have_the_kernels_sizes() {
        // MACH_TASK_BASIC_INFO_COUNT, THREAD_IDENTIFIER_INFO_COUNT,
        // THREAD_EXTENDED_INFO_COUNT; TASK_VM_INFO_REV1_COUNT
        assert_eq!(words::<TaskBasicInfo>(), 12);
        assert_eq!(words::<ThreadIdentifierInfo>(), 6);
        assert_eq!(words::<ThreadExtendedInfo>(), 28);
        assert_eq!(words::<TaskVmInfo>(), 38);
    }

    #[test]
    fn the_process_and_its_threads_are_read() {
        let m = process_memory().expect("memory");
        assert!(m.working_set > 0 && m.private > 0);
        let (threads, process) = read_threads();
        assert!(process > 0);
        assert!(threads.iter().any(|t| t.id == current_thread_id()));
        assert!(system_times().is_some_and(|(idle, total)| total > idle));
        assert!(!core_times().is_empty());
    }
}
