//! LifeTech: the frame loop's pace in the log, every 30 s ("frame pacing"):
//! what a frame waited for, so that a low frame rate with an idle processor
//! can be told apart: the GPU (the wait for the frame before), the frame
//! limiter (the cap, the background cap), or a system that wakes the
//! limiter late (macOS slows the timers of an application it takes for
//! idle). The Resource monitor shows these waits as one.

use std::time::{Duration, Instant};

const EVERY: Duration = Duration::from_secs(30);

#[derive(Default)]
pub struct FrameDiag {
    since: Option<Instant>,
    frames: u32,
    focused: u32,
    gpu: Duration,
    limiter: Duration,
    late: Duration,
    worst_late: Duration,
    cap: f32,
}

/// What one frame waited for.
pub struct Waits {
    pub focused: bool,
    /// The frame limiter's rate (0: none).
    pub cap: f32,
    pub gpu: Duration,
    pub limiter: Duration,
    /// How long after its time the limiter's wait ended.
    pub late: Duration,
}

impl FrameDiag {
    /// Count a frame; the line once every 30 s (`extra`: vsync, screen…).
    pub fn frame(&mut self, w: Waits, extra: impl FnOnce() -> String) {
        let now = Instant::now();
        let since = *self.since.get_or_insert(now);
        self.frames += 1;
        self.focused += u32::from(w.focused);
        self.gpu += w.gpu;
        self.limiter += w.limiter;
        self.late += w.late;
        self.worst_late = self.worst_late.max(w.late);
        self.cap = w.cap;
        let span = now.duration_since(since);
        if span >= EVERY {
            log::info!("frame pacing: {}", self.line(span, &extra()));
            *self = FrameDiag::default();
        }
    }

    fn line(&self, span: Duration, extra: &str) -> String {
        let n = self.frames.max(1) as f64;
        let per = |d: Duration| d.as_secs_f64() * 1000.0 / n;
        format!(
            "{:.1} fps over {:.0} s, in front {:.0} % of frames, cap {} | per frame: GPU wait {:.1} ms, limiter {:.1} ms, limiter late {:.2} ms (worst {:.1} ms) | {extra}",
            self.frames as f64 / span.as_secs_f64(),
            span.as_secs_f64(),
            self.focused as f64 * 100.0 / n,
            if self.cap >= 1.0 {
                format!("{:.0} fps", self.cap)
            } else {
                "none".into()
            },
            per(self.gpu),
            per(self.limiter),
            per(self.late),
            self.worst_late.as_secs_f64() * 1000.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_line_says_what_a_frame_waited_for() {
        let mut d = FrameDiag::default();
        for _ in 0..24 {
            d.frame(
                Waits {
                    focused: true,
                    cap: 60.0,
                    gpu: Duration::from_millis(30),
                    limiter: Duration::from_millis(5),
                    late: Duration::from_micros(500),
                },
                String::new,
            );
        }
        let line = d.line(Duration::from_secs(1), "vsync off");
        assert!(
            line.starts_with("24.0 fps over 1 s, in front 100 % of frames, cap 60 fps"),
            "{line}"
        );
        assert!(line.contains("GPU wait 30.0 ms, limiter 5.0 ms, limiter late 0.50 ms"), "{line}");
    }
}
