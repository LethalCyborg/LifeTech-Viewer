//! LifeTech: the first-login character picker in the app. Decides once per
//! session whether to offer it (the login answer's `ever_logged_in`,
//! `AURORA_STARTER_FORCE`, or `AURORA_DEMO_STARTER`), keeps the Library
//! loading while it is open, and runs the try-on: the first character tried
//! saves the outfit and the camera, each one is copied from the Library and
//! worn as the whole outfit (`wear_as_outfit`, after
//! LLAppearanceMgr::wearItemsOnAvatar's copy), the camera faces the avatar,
//! and "Cancel" puts the saved outfit and camera back.

use super::App;
use crate::camera::{CameraPreset, CameraSettings};
use crate::ui::starter_picker::{PickerAction, PickerEnv, Saved, Stage};
use crate::world::{appearance, inventory::actions, starter_looks};
use uuid::Uuid;

impl App {
    /// One frame of the picker. `allowed`: nothing may cover the view now
    /// (loading screen, mouselook, the photo's full frame, do not disturb);
    /// the picker then waits and shows after.
    pub(super) fn starter_picker_frame(&mut self, ctx: &egui::Context, p: &crate::theme::Palette, allowed: bool) {
        // the bottom bar's button shows pressed while the panel is open
        self.panels.starter_looks = self.starter_picker.is_open();
        if self.starter_picker.stage == Stage::Waiting && !self.decide_starter_picker() {
            return;
        }
        if !self.starter_picker.is_open() || !allowed {
            return;
        }
        let inv = &mut self.world.inventory;
        // the Library's folders arrive a few at a time; asking again each
        // frame is a walk of its folders, with at most 16 fetches queued
        let library = inv.lib_root;
        let library_loaded = !library.is_nil() && actions::request_tree(inv, library);
        let all = starter_looks::find(inv);
        let total = all.len();
        let looks = starter_looks::offered(all);
        if library_loaded && !self.starter_picker.logged {
            self.starter_picker.logged = true;
            let names: Vec<&str> = looks.iter().map(|l| l.name.as_str()).collect();
            log::info!(
                "starter picker: Library loaded, {total} character(s), {} offered: {names:?}",
                looks.len()
            );
        }
        // AURORA_DEMO_STARTER=try: the first character tried on by itself
        if self.demo
            && library_loaded
            && !self.starter_picker.demo_tried
            && crate::demo::starter::try_at().is_some_and(|at| self.frame_count >= at)
            && let Some(first) = looks.first()
        {
            self.starter_picker.demo_tried = true;
            self.try_on_starter(first.folder);
        }
        // one outfit change at a time: a try-on waits for the last one
        let busy = !self.inventory_ui.wear_copy_groups.is_empty() || self.appearance_ui.pending.is_some();
        let env = PickerEnv {
            looks: &looks,
            images: &self.ui_images,
            library_loaded,
            busy,
        };
        match self.starter_picker.show(ctx, p, &env) {
            Some(PickerAction::TryOn(folder)) => self.try_on_starter(folder),
            Some(PickerAction::Keep) => {
                log::info!("starter picker: the character tried on is kept");
                if let Some(saved) = self.starter_picker.saved.take() {
                    self.restore_starter_camera(saved.camera);
                }
            }
            Some(PickerAction::Cancel) => match self.starter_picker.saved.take() {
                Some(saved) => {
                    log::info!("starter picker: cancelled, the outfit worn before is put back");
                    self.apply_appearance_action(appearance::Action::SetOutfit(saved.outfit));
                    self.restore_starter_camera(saved.camera);
                }
                None => log::info!("starter picker: closed without trying a character"),
            },
            None => {}
        }
    }

    /// Wears a character as the whole outfit; the first one saves the outfit
    /// and the camera for "Cancel", and turns the camera to the avatar.
    fn try_on_starter(&mut self, folder: Uuid) {
        let inv = &self.world.inventory;
        if !starter_looks::loaded(inv, folder) {
            log::warn!("starter picker: the character's folder is not loaded yet");
            return;
        }
        let items = starter_looks::items(inv, folder);
        if items.is_empty() {
            log::warn!("starter picker: the character holds nothing to wear");
            return;
        }
        if self.starter_picker.saved.is_none() {
            let Some(cof) = appearance::cof(inv) else {
                log::warn!("starter picker: no Current Outfit folder, nothing tried on");
                return;
            };
            if !appearance::complete(inv, cof) {
                // the outfit could not be put back: wait until it is known
                log::info!("starter picker: the current outfit is still loading, try again in a moment");
                self.world.inventory.request(cof);
                return;
            }
            let outfit = appearance::usable_links(inv, cof);
            log::info!("starter picker: outfit saved before the first try-on ({} link(s))", outfit.len());
            self.starter_picker.saved = Some(Saved {
                outfit,
                camera: self.settings.camera.clone(),
            });
            self.switch_camera_view(CameraPreset::Front);
        }
        log::info!("starter picker: trying on a character ({} item(s))", items.len());
        self.starter_picker.trying = Some(folder);
        self.wear_as_outfit(items);
    }

    /// The camera as it was before the first try-on.
    fn restore_starter_camera(&mut self, camera: CameraSettings) {
        self.settings.camera = camera;
        self.camera.reset_to_defaults(&mut self.world.agent, &self.settings.camera);
        self.settings.save();
    }

    /// LifeTech: once per session, after login and with the inventory's
    /// folders known, Aurora Viewer's "#Aurora" folder (with its AO and
    /// texture library) is renamed to LifeTech's names
    /// (`world::brand_folder`).
    pub(super) fn migrate_brand_folder(&mut self) {
        let inv = &self.world.inventory;
        if self.brand_folder_checked || self.demo || self.world.login.is_none() || !inv.folders.contains_key(&inv.root) {
            return;
        }
        self.brand_folder_checked = true;
        let changes = crate::world::brand_folder::migration(inv);
        if !changes.is_empty() {
            log::info!(
                "brand folder: renaming Aurora Viewer's #Aurora folders to LifeTech's ({})",
                changes.len()
            );
        }
        for change in changes {
            self.apply_inventory_action(crate::ui::inventory::InvAction::Edit(change));
        }
    }

    /// The bottom bar's "Choose your look" button: opens the panel at any
    /// time (not only on a first login); pressed again while it is open, it
    /// closes it like "Not now" or "Cancel" would.
    pub(super) fn toggle_starter_picker(&mut self) {
        if self.starter_picker.is_open() {
            if self.inventory_ui.wear_copy_groups.is_empty() && self.appearance_ui.pending.is_none() {
                self.starter_picker.stage = Stage::Done;
                if let Some(saved) = self.starter_picker.saved.take() {
                    log::info!("starter picker: closed from the bar, the outfit worn before is put back");
                    self.apply_appearance_action(appearance::Action::SetOutfit(saved.outfit));
                    self.restore_starter_camera(saved.camera);
                }
            }
            return;
        }
        log::info!("starter picker: opened from the bar");
        self.starter_picker.open();
    }

    /// Whether the picker's fate is known: opened on a first login ever (or
    /// the demo scenario), done otherwise. False while the login answer has
    /// not arrived.
    fn decide_starter_picker(&mut self) -> bool {
        if self.demo && crate::demo::starter::enabled() {
            crate::demo::starter::seed(&mut self.world);
            self.starter_picker.open();
            return true;
        }
        let Some(login) = self.world.login.as_deref() else {
            return false;
        };
        let first = starter_looks::first_login_ever(Some(login));
        // AURORA_STARTER_FORCE: the picker on an account that has logged in
        // before, to test it on the grid with an existing avatar
        let forced = std::env::var("AURORA_STARTER_FORCE").is_ok_and(|v| !v.is_empty() && v != "0");
        if first || forced {
            log::info!("starter picker: offering the Library's characters (first login ever: {first}, forced: {forced})");
            self.starter_picker.open();
        } else {
            self.starter_picker.stage = Stage::Done;
        }
        true
    }
}
