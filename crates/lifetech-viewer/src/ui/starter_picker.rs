//! LifeTech: "Choose your look", shown once, on an account's very first
//! login (`world::starter_looks::first_login_ever`). A panel at the right of
//! the screen lists the Library's mesh characters (`world::starter_looks`):
//! clicking one tries it on (the app wears it as the whole outfit and turns
//! the camera to face the avatar), "Keep this look" ends the choice, and
//! "Cancel" puts the outfit worn before back. A Library object (a mesh body)
//! cannot be previewed without wearing it: the simulator only serves it
//! once worn or rezzed. Not in the Legacy Viewer, which leaves new residents
//! in the outfit the sign-up page gave them.

use super::icons;
use super::widgets::{self, ActionKind, action_button_if};
use crate::theme::Palette;
use crate::world::starter_looks::StarterLook;
use egui::{RichText, Vec2};
use lifetech_net::outfits::OutfitLink;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Width of the panel's content, height of one character's row.
const WIDTH: f32 = 320.0;
const ROW: f32 = 46.0;
const BUTTON: Vec2 = Vec2::new(152.0, 34.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Stage {
    /// Not decided yet: waiting for the login answer.
    #[default]
    Waiting,
    /// On screen.
    Open,
    /// Answered, or not a first login: never shown again this session.
    Done,
}

/// What the first try-on changed, to put it back on "Cancel".
pub struct Saved {
    /// The current outfit's links before the first try-on.
    pub outfit: Vec<OutfitLink>,
    /// The camera settings before it turned to face the avatar.
    pub camera: crate::camera::CameraSettings,
}

#[derive(Default)]
pub struct StarterPickerUi {
    pub stage: Stage,
    /// The character worn, or being put on.
    pub trying: Option<Uuid>,
    /// Set by the first try-on.
    pub saved: Option<Saved>,
    /// Character pictures to load (drained by the app with the others).
    pub wanted_images: HashSet<Uuid>,
    /// The characters found were written to the log once the Library loaded.
    pub logged: bool,
    /// `AURORA_DEMO_STARTER=try` made its one try-on.
    pub demo_tried: bool,
}

/// What the panel draws.
pub struct PickerEnv<'a> {
    pub looks: &'a [StarterLook],
    pub images: &'a HashMap<Uuid, egui::TextureHandle>,
    /// The whole Library has loaded: every character can be tried on.
    pub library_loaded: bool,
    /// An outfit change is under way: the next one waits.
    pub busy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerAction {
    /// Try this character's folder on.
    TryOn(Uuid),
    /// Keep the character worn now.
    Keep,
    /// Put the outfit worn before back (or, with nothing tried, just close).
    Cancel,
}

impl StarterPickerUi {
    /// Show the panel from now on.
    pub fn open(&mut self) {
        self.stage = Stage::Open;
        self.trying = None;
        self.saved = None;
    }

    pub fn is_open(&self) -> bool {
        self.stage == Stage::Open
    }

    pub fn show(&mut self, ctx: &egui::Context, p: &Palette, env: &PickerEnv<'_>) -> Option<PickerAction> {
        if self.stage != Stage::Open {
            return None;
        }
        let mut out = None;
        egui::Area::new(egui::Id::unique("starter_picker"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::RIGHT_CENTER, Vec2::new(-24.0, 0.0))
            .constrain(true)
            .show(ctx, |ui| {
                widgets::overlay(ui, p, egui::Margin::same(16), |ui| {
                    ui.set_width(WIDTH);
                    ui.label(
                        RichText::new(lifetech_i18n::tr!("starter-picker-title"))
                            .size(20.0)
                            .strong()
                            .color(p.ink),
                    );
                    ui.add_space(4.0);
                    ui.label(RichText::new(lifetech_i18n::tr!("starter-picker-intro")).size(13.0).color(p.muted));
                    ui.add_space(12.0);
                    if env.looks.is_empty() || !env.library_loaded {
                        if env.library_loaded {
                            ui.label(RichText::new(lifetech_i18n::tr!("starter-picker-none")).size(13.0).color(p.ink));
                        } else {
                            widgets::progress(
                                ui,
                                p,
                                "starter_picker_loading",
                                0.3,
                                WIDTH,
                                lifetech_i18n::tr!("starter-picker-loading"),
                                None,
                            );
                        }
                    } else {
                        egui::ScrollArea::vertical().max_height(ROW * 9.0).show(ui, |ui| {
                            let (everyday, fantasy): (Vec<_>, Vec<_>) = env.looks.iter().partition(|l| !l.is_fantasy());
                            for (title, list) in [
                                (lifetech_i18n::tr!("starter-picker-everyday"), everyday),
                                (lifetech_i18n::tr!("starter-picker-fantasy"), fantasy),
                            ] {
                                if list.is_empty() {
                                    continue;
                                }
                                ui.label(RichText::new(title).size(12.0).strong().color(p.muted));
                                ui.add_space(4.0);
                                for look in list {
                                    if self.row(ui, p, env, look) {
                                        out = Some(PickerAction::TryOn(look.folder));
                                    }
                                    ui.add_space(4.0);
                                }
                                ui.add_space(8.0);
                            }
                        });
                    }
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let tried = self.saved.is_some();
                        let (icon, label) = if tried {
                            ("arrow-back-up", lifetech_i18n::tr!("starter-picker-cancel"))
                        } else {
                            ("x", lifetech_i18n::tr!("starter-picker-not-now"))
                        };
                        let cancel = action_button_if(ui, p, icon, label, ActionKind::Flat, BUTTON, !env.busy);
                        if tried {
                            cancel.clone().on_hover_text(lifetech_i18n::tr!("starter-picker-cancel-hint"));
                        }
                        if cancel.clicked() {
                            out = Some(PickerAction::Cancel);
                        }
                        let keep = action_button_if(
                            ui,
                            p,
                            "check",
                            lifetech_i18n::tr!("starter-picker-keep-look"),
                            ActionKind::Primary,
                            BUTTON,
                            self.trying.is_some() && !env.busy,
                        );
                        if keep.clicked() {
                            out = Some(PickerAction::Keep);
                        }
                    });
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(lifetech_i18n::tr!("starter-picker-later"))
                            .size(11.5)
                            .color(p.muted_dim),
                    );
                });
            });
        if matches!(out, Some(PickerAction::Keep | PickerAction::Cancel)) {
            self.stage = Stage::Done;
        }
        out
    }

    /// One character's row; true when clicked to be tried on.
    fn row(&mut self, ui: &mut egui::Ui, p: &Palette, env: &PickerEnv<'_>, look: &StarterLook) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(WIDTH, ROW), egui::Sense::click());
        let worn = self.trying == Some(look.folder);
        let radius = ui.visuals().window_corner_radius;
        ui.painter().rect_filled(rect, radius, p.field);
        let stroke = if worn {
            egui::Stroke::new(2.0, p.violet)
        } else if response.hovered() && !env.busy {
            egui::Stroke::new(1.0, p.violet_light)
        } else {
            egui::Stroke::new(1.0, p.raised)
        };
        ui.painter().rect_stroke(rect, radius, stroke, egui::StrokeKind::Inside);
        // the picture, when the Library gives one, else the outfit icon
        let pic = egui::Rect::from_min_size(rect.min + Vec2::splat(5.0), Vec2::splat(ROW - 10.0));
        let full = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0));
        if !look.picture.is_nil() {
            self.wanted_images.insert(look.picture);
        }
        if let Some(t) = env.images.get(&look.picture) {
            ui.painter().image(t.id(), pic, full, widgets::PICTURE_TINT);
        } else if let Some(t) = icons::global("hanger-2-filled") {
            ui.painter().image(t.id(), pic.shrink(6.0), full, p.violet_light);
        }
        let name_color = if worn { p.violet_light } else { p.ink };
        ui.painter().text(
            egui::pos2(pic.right() + 10.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            look.display_name(),
            egui::FontId::proportional(14.0),
            name_color,
        );
        if worn {
            let status = if env.busy {
                (lifetech_i18n::tr!("starter-picker-putting-on"), p.muted)
            } else {
                (lifetech_i18n::tr!("starter-picker-wearing"), p.violet_light)
            };
            ui.painter().text(
                egui::pos2(rect.right() - 12.0, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                status.0,
                egui::FontId::proportional(12.0),
                status.1,
            );
        }
        let clicked = response.on_hover_cursor(egui::CursorIcon::PointingHand).clicked();
        clicked && may_try(self.trying, env.busy, look.folder)
    }
}

/// Whether a click may try a character on: not while an outfit change runs,
/// not the one already worn.
pub fn may_try(trying: Option<Uuid>, busy: bool, folder: Uuid) -> bool {
    !busy && trying != Some(folder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_picker_opens_once_and_closes_for_good() {
        let mut ui = StarterPickerUi::default();
        assert_eq!(ui.stage, Stage::Waiting);
        assert!(!ui.is_open());
        ui.open();
        assert!(ui.is_open());
        assert!(ui.trying.is_none() && ui.saved.is_none());
        ui.stage = Stage::Done;
        assert!(!ui.is_open());
    }

    #[test]
    fn a_character_is_tried_once_and_never_during_a_change() {
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        assert!(may_try(None, false, a));
        assert!(!may_try(None, true, a), "an outfit change is under way");
        assert!(!may_try(Some(a), false, a), "already worn");
        assert!(may_try(Some(a), false, b));
    }
}
