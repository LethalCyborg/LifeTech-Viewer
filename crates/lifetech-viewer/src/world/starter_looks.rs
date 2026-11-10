//! LifeTech: the starter characters offered on a resident's very first
//! login. The Second Life Library every account receives holds complete
//! avatars (a folder with a shape, a skin, hair, eyes and clothes); this
//! module finds them, so that the first-login picker (`ui/starter_picker.rs`)
//! can show one card per character and wear the chosen one.
//!
//! Nothing here is sent anywhere: the Library is read like any other part of
//! the inventory, and the chosen folder is worn through the existing
//! "copy a Library item, then wear the copy" path (`App::wear_selection`).

use super::inventory::Inventory;
use lifetech_net::login::LoginResponse;
use std::collections::HashSet;
use uuid::Uuid;

/// Folder type of an outfit (LLFolderType::FT_OUTFIT).
const FT_OUTFIT: i32 = 47;
/// Asset types that can be worn: clothing, object (attachment), body part.
const WEARABLE_TYPES: [i32; 3] = [5, 6, 13];
/// A body part whose wearable type is the shape (LLWearableType::WT_SHAPE).
const BODY_PART: i32 = 13;
const SHAPE: u32 = 0;

/// One character of the Library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarterLook {
    /// The Library folder that holds the whole character.
    pub folder: Uuid,
    pub name: String,
    /// Its picture: the folder's thumbnail, else a picture found inside
    /// (`appearance::outfit_image`); nil when it has none.
    pub picture: Uuid,
}

impl StarterLook {
    /// A mesh character: the Library names its modern avatars "Name (Mesh)";
    /// the others are the older classic (system-body) avatars.
    pub fn is_mesh(&self) -> bool {
        self.name.to_lowercase().contains("(mesh)")
    }

    /// The name without the Library's " (Mesh)" mark: every character
    /// offered is a mesh one.
    pub fn display_name(&self) -> String {
        let lower = self.name.to_lowercase();
        match lower.find("(mesh)") {
            Some(at) => format!("{}{}", &self.name[..at], &self.name[at + "(mesh)".len()..])
                .trim()
                .to_owned(),
            None => self.name.trim().to_owned(),
        }
    }

    /// A vampire, werewolf, zombie or other creature: listed apart from the
    /// everyday characters.
    pub fn is_fantasy(&self) -> bool {
        const CREATURES: [&str; 6] = ["vampire", "werewolf", "wolfman", "zombie", "beast", "monster"];
        let lower = self.name.to_lowercase();
        CREATURES.iter().any(|word| lower.contains(word))
    }
}

/// The characters a newcomer is offered: the mesh ones only, or all of them
/// when the Library has no mesh character.
pub fn offered(looks: Vec<StarterLook>) -> Vec<StarterLook> {
    if looks.iter().any(StarterLook::is_mesh) {
        looks.into_iter().filter(StarterLook::is_mesh).collect()
    } else {
        looks
    }
}

/// Whether this login is the account's first one ever, on any viewer: the
/// login answer's `login-flags` say `ever_logged_in` = "N" (LLStartUp,
/// `gAgent.setFirstLogin`). An answer without the flag is not a first login.
pub fn first_login_ever(login: Option<&LoginResponse>) -> bool {
    login.is_some_and(|l| l.raw["login-flags"].at(0)["ever_logged_in"].as_str() == "N")
}

/// The Library's characters, in the Library's order: every Library folder
/// that is an outfit folder or holds a shape directly, and is not itself
/// inside such a folder (a character's sub-folders are part of it). Only the
/// folders already loaded are seen: call it again as the Library arrives.
pub fn find(inv: &Inventory) -> Vec<StarterLook> {
    let mut out = Vec::new();
    if inv.lib_root.is_nil() {
        return out;
    }
    let mut todo = vec![inv.lib_root];
    let mut seen = HashSet::new();
    while let Some(id) = todo.pop() {
        if !seen.insert(id) {
            continue;
        }
        let Some(folder) = inv.folders.get(&id) else {
            continue;
        };
        if id != inv.lib_root && is_character(inv, id) {
            out.push(StarterLook {
                folder: id,
                name: folder.info.name.clone(),
                picture: super::appearance::outfit_image(inv, id),
            });
            continue;
        }
        // depth first, in the folder's own order
        todo.extend(folder.children.iter().rev().copied());
    }
    out
}

/// A folder that is a whole character: an outfit folder, or one with a
/// shape among its own items.
fn is_character(inv: &Inventory, id: Uuid) -> bool {
    let Some(folder) = inv.folders.get(&id) else {
        return false;
    };
    folder.info.type_default == FT_OUTFIT
        || folder
            .items
            .iter()
            .filter_map(|item| inv.items.get(item))
            .any(|item| item.asset_type == BODY_PART && item.flags & 0xff == SHAPE)
}

/// The items to wear for a character: every clothing, body part and
/// attachment in its folder and sub-folders, folders first in their order.
pub fn items(inv: &Inventory, folder: Uuid) -> Vec<Uuid> {
    let mut out = Vec::new();
    let mut todo = vec![folder];
    let mut seen = HashSet::new();
    while let Some(id) = todo.pop() {
        if !seen.insert(id) {
            continue;
        }
        let Some(f) = inv.folders.get(&id) else {
            continue;
        };
        out.extend(
            f.items
                .iter()
                .filter(|item| inv.items.get(item).is_some_and(|it| WEARABLE_TYPES.contains(&it.asset_type)))
                .copied(),
        );
        todo.extend(f.children.iter().rev().copied());
    }
    out
}

/// Whether a character's folder and sub-folders are all loaded, so that
/// `items` lists everything there is to wear.
pub fn loaded(inv: &Inventory, folder: Uuid) -> bool {
    let mut todo = vec![folder];
    let mut seen = HashSet::new();
    while let Some(id) = todo.pop() {
        if !seen.insert(id) {
            continue;
        }
        let Some(f) = inv.folders.get(&id) else {
            return false;
        };
        if f.state != super::inventory::FetchState::Fetched {
            return false;
        }
        todo.extend(f.children.iter().copied());
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::inventory::{FetchState, Folder};
    use lifetech_net::inventory::{InvFolder, InvItem};

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn folder(inv: &mut Inventory, n: u128, parent: Uuid, name: &str, kind: i32) {
        inv.folders.insert(
            id(n),
            Folder {
                info: InvFolder {
                    id: id(n),
                    parent,
                    name: name.into(),
                    type_default: kind,
                    ..Default::default()
                },
                children: Vec::new(),
                items: Vec::new(),
                state: FetchState::Fetched,
                library: true,
            },
        );
        if let Some(p) = inv.folders.get_mut(&parent) {
            p.children.push(id(n));
        }
    }

    fn item(inv: &mut Inventory, n: u128, parent: u128, asset_type: i32, flags: u32) {
        inv.items.insert(
            id(n),
            InvItem {
                id: id(n),
                parent: id(parent),
                name: format!("item {n}"),
                asset_type,
                flags,
                ..Default::default()
            },
        );
        inv.folders.get_mut(&id(parent)).expect("parent").items.push(id(n));
    }

    /// Library ▸ Avatars ▸ {City (shape, skin, shirt, Extras ▸ hat),
    /// Fantasy (outfit folder)}, Library ▸ Textures (a texture).
    fn library() -> Inventory {
        let mut inv = Inventory {
            lib_root: id(1),
            ..Default::default()
        };
        inv.folders.insert(
            id(1),
            Folder {
                info: InvFolder {
                    id: id(1),
                    name: "Library".into(),
                    type_default: 8,
                    ..Default::default()
                },
                children: Vec::new(),
                items: Vec::new(),
                state: FetchState::Fetched,
                library: true,
            },
        );
        folder(&mut inv, 10, id(1), "Avatars", -1);
        folder(&mut inv, 11, id(10), "City", -1);
        item(&mut inv, 100, 11, 13, 0); // shape
        item(&mut inv, 101, 11, 13, 1); // skin
        item(&mut inv, 102, 11, 5, 4); // shirt
        folder(&mut inv, 12, id(11), "Extras", -1);
        item(&mut inv, 103, 12, 6, 2); // hat
        item(&mut inv, 104, 12, 0, 0); // a picture: not worn
        folder(&mut inv, 13, id(10), "Fantasy", FT_OUTFIT);
        folder(&mut inv, 20, id(1), "Textures", -1);
        item(&mut inv, 200, 20, 0, 0);
        inv
    }

    #[test]
    fn the_first_login_is_read_from_the_login_flags() {
        assert!(!first_login_ever(None));
    }

    #[test]
    fn characters_are_the_folders_with_a_shape_or_outfits() {
        let inv = library();
        let found = find(&inv);
        let names: Vec<_> = found.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, ["City", "Fantasy"]);
        // a character's picture is found inside it
        assert_eq!(found[0].picture, Uuid::nil(), "texture items carry no asset id here");
    }

    #[test]
    fn a_folder_with_a_skin_but_no_shape_is_not_a_character() {
        let mut inv = library();
        folder(&mut inv, 30, id(1), "Skins", -1);
        item(&mut inv, 300, 30, 13, 1);
        assert!(find(&inv).iter().all(|l| l.name != "Skins"));
    }

    #[test]
    fn wearing_a_character_takes_every_wearable_of_its_tree() {
        let inv = library();
        let mut worn = items(&inv, id(11));
        worn.sort();
        assert_eq!(worn, [id(100), id(101), id(102), id(103)]);
    }

    #[test]
    fn a_character_is_loaded_once_every_sub_folder_is() {
        let mut inv = library();
        assert!(loaded(&inv, id(11)));
        inv.folders.get_mut(&id(12)).expect("extras").state = FetchState::Unknown;
        assert!(!loaded(&inv, id(11)));
    }

    #[test]
    fn no_library_no_characters() {
        assert!(find(&Inventory::default()).is_empty());
    }

    fn look(name: &str) -> StarterLook {
        StarterLook {
            folder: Uuid::new_v4(),
            name: name.into(),
            picture: Uuid::nil(),
        }
    }

    #[test]
    fn newcomers_are_offered_the_mesh_characters_only() {
        let all = vec![look("Amy"), look("Alicia (Mesh)"), look("Barry (MESH)"), look("Bitsy")];
        let names: Vec<_> = offered(all).into_iter().map(|l| l.name).collect();
        assert_eq!(names, ["Alicia (Mesh)", "Barry (MESH)"]);
    }

    #[test]
    fn names_lose_the_mesh_mark_and_creatures_are_fantasy() {
        assert_eq!(look("Alicia (Mesh)").display_name(), "Alicia");
        assert_eq!(look("Vampire Hunter Alison (MESH)").display_name(), "Vampire Hunter Alison");
        assert_eq!(look("Amy").display_name(), "Amy");
        assert!(look("Zombie Sadie (Mesh)").is_fantasy());
        assert!(look("Wolfman Hugo (Mesh)").is_fantasy());
        assert!(!look("Johnny (Mesh)").is_fantasy());
    }

    #[test]
    fn a_library_without_mesh_characters_offers_them_all() {
        assert_eq!(offered(vec![look("Amy"), look("Bitsy")]).len(), 2);
    }
}
