//! LifeTech: the viewer's own folder at the top of the inventory is
//! "#LifeTech", with "LifeTech AO" (the animation overrider's sets) and
//! "Build Chest" (the Build Chest's resources, ui/texture_library.rs) in it.
//! Aurora Viewer, which LifeTech Viewer is based on, named them "#Aurora",
//! "Aurora AO" and "Aurora Library", and LifeTech Viewer 1.0 named the chest
//! "LifeTech Library": an avatar that used either keeps its sets and its
//! chest, the folders renamed once, at login.

use super::inventory::{Inventory, actions};
use lifetech_llsd::llsd_map;
use lifetech_net::inventory::operations::Mutation;
use uuid::Uuid;

/// The viewer's folder at the top of the inventory.
pub const BRAND: &str = "#LifeTech";
/// The Build Chest's folder in it (`ui::texture_library::catalogue::LIBRARY`).
pub const CHEST: &str = "Build Chest";
/// Aurora Viewer's name of the top folder.
const LEGACY_BRAND: &str = "#Aurora";
/// Earlier names of the folders in it, and LifeTech's.
const LEGACY_CHILDREN: [(&str, &str); 3] = [("Aurora AO", "LifeTech AO"), ("Aurora Library", CHEST), ("LifeTech Library", CHEST)];

/// The child folder of `parent` named `name`, if any (not in the Library).
fn child(inv: &Inventory, parent: Uuid, name: &str) -> Option<Uuid> {
    inv.folders
        .get(&parent)?
        .children
        .iter()
        .copied()
        .find(|id| inv.folders.get(id).is_some_and(|f| !f.library && f.info.name == name))
}

/// The folders to rename and their new names: "#Aurora" when there is no
/// "#LifeTech" yet (two folders are never merged behind the resident's
/// back), then in the brand folder "Aurora AO" and the chest's earlier
/// names, each only while its new name is free.
fn renames(inv: &Inventory) -> Vec<(Uuid, &'static str)> {
    let root = inv.root;
    if root.is_nil() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let brand = match child(inv, root, BRAND) {
        Some(brand) => brand,
        None => match child(inv, root, LEGACY_BRAND) {
            Some(legacy) => {
                out.push((legacy, BRAND));
                legacy
            }
            None => return out,
        },
    };
    let mut taken: Vec<&str> = Vec::new();
    for (old, new) in LEGACY_CHILDREN {
        if child(inv, brand, new).is_none()
            && !taken.contains(&new)
            && let Some(id) = child(inv, brand, old)
        {
            out.push((id, new));
            taken.push(new);
        }
    }
    out
}

/// The renames that bring the folders to LifeTech's names (`renames`).
pub fn migration(inv: &Inventory) -> Vec<Mutation> {
    renames(inv)
        .into_iter()
        .filter_map(|(id, name)| match actions::patch(inv, id, llsd_map! { "name" => name }) {
            Ok(change) => Some(change),
            Err(reason) => {
                log::warn!("brand folder: {name} not renamed: {reason}");
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::inventory::{FetchState, Folder};
    use lifetech_net::inventory::InvFolder;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn add(inv: &mut Inventory, n: u128, parent: Uuid, name: &str) {
        inv.folders.insert(
            id(n),
            Folder {
                info: InvFolder {
                    id: id(n),
                    parent,
                    name: name.into(),
                    type_default: -1,
                    version: 1,
                    ..Default::default()
                },
                children: Vec::new(),
                items: Vec::new(),
                state: FetchState::Fetched,
                library: false,
            },
        );
        if let Some(p) = inv.folders.get_mut(&parent) {
            p.children.push(id(n));
        }
    }

    fn inventory() -> Inventory {
        let mut inv = Inventory {
            root: id(1),
            ..Default::default()
        };
        add(&mut inv, 1, Uuid::nil(), "My Inventory");
        inv
    }

    #[test]
    fn an_aurora_folder_and_its_ao_and_library_are_renamed() {
        let mut inv = inventory();
        add(&mut inv, 2, id(1), "#Aurora");
        add(&mut inv, 3, id(2), "Aurora AO");
        add(&mut inv, 4, id(2), "Aurora Library");
        add(&mut inv, 5, id(2), "Received items");
        assert_eq!(
            renames(&inv),
            [(id(2), "#LifeTech"), (id(3), "LifeTech AO"), (id(4), "Build Chest")],
            "#Aurora, its AO and its library; the rest stays"
        );
        assert_eq!(migration(&inv).len(), 3);
    }

    #[test]
    fn the_lifetech_library_of_1_0_becomes_the_build_chest() {
        let mut inv = inventory();
        add(&mut inv, 2, id(1), "#LifeTech");
        add(&mut inv, 3, id(2), "LifeTech AO");
        add(&mut inv, 4, id(2), "LifeTech Library");
        assert_eq!(renames(&inv), [(id(4), "Build Chest")]);
        // once renamed, nothing more
        inv.folders.get_mut(&id(4)).unwrap().info.name = "Build Chest".into();
        assert!(renames(&inv).is_empty());
    }

    #[test]
    fn an_existing_chest_is_never_renamed_over() {
        let mut inv = inventory();
        add(&mut inv, 2, id(1), "#LifeTech");
        add(&mut inv, 3, id(2), "Build Chest");
        add(&mut inv, 4, id(2), "LifeTech Library");
        assert!(renames(&inv).is_empty());
        // two earlier chests: only the first takes the name
        let mut inv = inventory();
        add(&mut inv, 2, id(1), "#LifeTech");
        add(&mut inv, 3, id(2), "Aurora Library");
        add(&mut inv, 4, id(2), "LifeTech Library");
        assert_eq!(renames(&inv), [(id(3), "Build Chest")]);
    }

    #[test]
    fn nothing_changes_without_an_aurora_folder_or_with_a_lifetech_one() {
        let mut inv = inventory();
        assert!(migration(&inv).is_empty(), "no #Aurora");
        add(&mut inv, 2, id(1), "#Aurora");
        add(&mut inv, 3, id(1), "#LifeTech");
        assert!(migration(&inv).is_empty(), "never merged");
        assert!(migration(&Inventory::default()).is_empty(), "no inventory yet");
    }
}
