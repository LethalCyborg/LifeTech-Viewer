//! LifeTech: the viewer's own folder at the top of the inventory is
//! "#LifeTech", with "LifeTech AO" (the animation overrider's sets) and
//! "LifeTech Library" (the texture library) in it. Aurora Viewer, which
//! LifeTech Viewer is based on, named them "#Aurora", "Aurora AO" and
//! "Aurora Library": an avatar that used it keeps its sets and its library,
//! its folders renamed once, at the first login with LifeTech Viewer.

use super::inventory::{Inventory, actions};
use lifetech_llsd::llsd_map;
use lifetech_net::inventory::operations::Mutation;
use uuid::Uuid;

/// The viewer's folder at the top of the inventory.
pub const BRAND: &str = "#LifeTech";
/// Aurora Viewer's names, and LifeTech's for the same folders.
const LEGACY_BRAND: &str = "#Aurora";
const LEGACY_CHILDREN: [(&str, &str); 2] = [("Aurora AO", "LifeTech AO"), ("Aurora Library", "LifeTech Library")];

/// The child folder of `parent` named `name`, if any (not in the Library).
fn child(inv: &Inventory, parent: Uuid, name: &str) -> Option<Uuid> {
    inv.folders
        .get(&parent)?
        .children
        .iter()
        .copied()
        .find(|id| inv.folders.get(id).is_some_and(|f| !f.library && f.info.name == name))
}

/// The renames that bring an Aurora Viewer folder to LifeTech's names:
/// "#Aurora" and its "Aurora AO" and "Aurora Library". Nothing when there is
/// no "#Aurora", or when a "#LifeTech" already exists (two folders are never
/// merged behind the resident's back).
pub fn migration(inv: &Inventory) -> Vec<Mutation> {
    let root = inv.root;
    if root.is_nil() || child(inv, root, BRAND).is_some() {
        return Vec::new();
    }
    let Some(legacy) = child(inv, root, LEGACY_BRAND) else {
        return Vec::new();
    };
    let mut renames = vec![(legacy, BRAND)];
    for (old, new) in LEGACY_CHILDREN {
        if child(inv, legacy, new).is_none()
            && let Some(id) = child(inv, legacy, old)
        {
            renames.push((id, new));
        }
    }
    renames
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
        assert_eq!(migration(&inv).len(), 3, "#Aurora, its AO and its library; the rest stays");
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
