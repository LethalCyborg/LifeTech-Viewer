//! AURORA_DEMO_STARTER=1|try: the first-login "Choose your look" panel
//! (`ui/starter_picker.rs`) on the demo, as if the account had never logged
//! in. The demo Library gets an "Avatars" folder like Second Life's: three
//! mesh characters (one a vampire, listed under Fantasy, with a hat in an
//! "Extras" sub-folder) and a classic avatar that is not offered, beside a
//! "Textures" folder that is no character. Clicking one tries it on: its
//! items are copied into the inventory and worn as the whole outfit, the
//! camera faces the avatar; "Cancel" puts the outfit back. With `try`, the
//! first character is tried on by itself (`try_at`), for a capture.

use crate::world::World;
use crate::world::inventory::{FetchState, Folder};
use lifetech_net::inventory::{InvFolder, InvItem};
use uuid::Uuid;

/// The scenario is on for this run.
pub fn enabled() -> bool {
    std::env::var("AURORA_DEMO_STARTER").is_ok_and(|m| !m.is_empty() && m != "0")
}

/// `AURORA_DEMO_STARTER=try`: the first character is tried on by itself at
/// this frame, for a capture of the try-on.
pub fn try_at() -> Option<u64> {
    (std::env::var("AURORA_DEMO_STARTER").as_deref() == Ok("try")).then_some(700)
}

/// Ids of the demo Library: folders from 8_100, items from 8_200.
const FOLDERS: u128 = 8_100;
const ITEMS: u128 = 8_200;

/// The characters: name, and whether it has a hat in an "Extras" folder.
const CHARACTERS: [(&str, bool); 4] = [
    ("City (Mesh)", false),
    ("Sporty (Mesh)", false),
    ("Vampire Fabian (Mesh)", true),
    ("Classic", false),
];

/// Wearables of a character: name, asset type, inventory type, flags (the
/// wearable type of LLWearableType: shape 0, skin 1, hair 2, eyes 3,
/// shirt 4, pants 5).
const PARTS: [(&str, i32, i32, u32); 6] = [
    ("Shape", 13, 18, 0),
    ("Skin", 13, 18, 1),
    ("Hair", 13, 18, 2),
    ("Eyes", 13, 18, 3),
    ("Shirt", 5, 18, 4),
    ("Pants", 5, 18, 5),
];

fn folder(id: Uuid, parent: Uuid, name: &str) -> Folder {
    Folder {
        info: InvFolder {
            id,
            parent,
            name: name.into(),
            type_default: -1,
            version: 1,
            ..Default::default()
        },
        children: Vec::new(),
        items: Vec::new(),
        state: FetchState::Fetched,
        library: true,
    }
}

fn item(id: Uuid, parent: Uuid, owner: Uuid, name: String, asset_type: i32, inv_type: i32, flags: u32) -> InvItem {
    InvItem {
        id,
        parent,
        name,
        asset_type,
        inv_type,
        flags,
        creator: owner,
        owner,
        base_mask: 0x7fff_ffff,
        owner_mask: 0x7fff_ffff,
        next_owner_mask: 0x7fff_ffff,
        // what a Library item allows everyone is what the agent holds on it
        everyone_mask: 0x7fff_ffff,
        ..Default::default()
    }
}

/// Adds the characters to the demo Library (created if missing).
pub fn seed(world: &mut World) {
    // an outfit to try characters over, and to put back on "Cancel"
    if crate::world::appearance::cof(&world.inventory).is_none() {
        crate::world::appearance::seed_demo(&mut world.inventory, world.agent_id);
    }
    let inv = &mut world.inventory;
    if inv.lib_root.is_nil() {
        inv.lib_root = Uuid::from_u128(8003);
        inv.lib_owner = Uuid::from_u128(8004);
    }
    let root = inv.lib_root;
    let owner = inv.lib_owner;
    inv.folders.entry(root).or_insert_with(|| {
        let mut f = folder(root, Uuid::nil(), lifetech_i18n::tr!("common-library"));
        f.info.type_default = 8;
        f
    });
    let mut next_folder = FOLDERS;
    let mut new_folder = |inv: &mut crate::world::inventory::Inventory, parent: Uuid, name: &str| {
        let id = Uuid::from_u128(next_folder);
        next_folder += 1;
        inv.folders.insert(id, folder(id, parent, name));
        if let Some(p) = inv.folders.get_mut(&parent) {
            p.children.push(id);
        }
        id
    };
    let avatars = new_folder(inv, root, "Avatars");
    let mut items = Vec::new();
    let mut next_item = ITEMS;
    let mut add = |items: &mut Vec<InvItem>, parent: Uuid, name: String, at: i32, it: i32, flags: u32| {
        items.push(item(Uuid::from_u128(next_item), parent, owner, name, at, it, flags));
        next_item += 1;
    };
    for (name, hat) in CHARACTERS {
        let character = new_folder(inv, avatars, name);
        for (part, at, it, flags) in PARTS {
            add(&mut items, character, format!("{name} {part}"), at, it, flags);
        }
        if hat {
            let extras = new_folder(inv, character, "Extras");
            // attached to the skull (attachment point 2)
            add(&mut items, extras, format!("{name} Hat"), 6, 6, 2);
        }
    }
    let textures = new_folder(inv, root, "Textures");
    add(&mut items, textures, "Wood".into(), 0, 0, 0);
    inv.add_items(items);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_demo_library_holds_four_characters() {
        let mut world = World::new(std::sync::Arc::new(crate::scene::avatar::AvatarLibrary::load()));
        seed(&mut world);
        let looks = crate::world::starter_looks::find(&world.inventory);
        let names: Vec<_> = looks.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, ["City (Mesh)", "Sporty (Mesh)", "Vampire Fabian (Mesh)", "Classic"]);
        // the classic avatar is not offered, the vampire is listed apart
        let offered = crate::world::starter_looks::offered(looks.clone());
        assert_eq!(offered.len(), 3);
        assert!(offered[2].is_fantasy());
        // the hat of the Extras folder is worn with the rest
        let fantasy = looks[2].folder;
        assert_eq!(crate::world::starter_looks::items(&world.inventory, fantasy).len(), 7);
        assert!(crate::world::starter_looks::loaded(&world.inventory, fantasy));
    }
}
