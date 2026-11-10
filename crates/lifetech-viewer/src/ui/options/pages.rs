//! The pages of the preferences, one module each, in the order of the
//! sidebar. A page declares its rows through `Cx` for one of its tabs.

pub mod accessibility;
pub mod appearance;
pub mod aurora;
pub mod avatars;
pub mod build;
pub mod camera;
pub mod chat;
pub mod commands;
pub mod confirmations;
pub mod controls;
pub mod fun;
pub mod graphics;
pub mod inventory;
pub mod joystick;
pub mod layout;
pub mod media;
pub mod network;
pub mod privacy;
pub mod protections;
pub mod rlva;
pub mod shortcuts;
pub mod sound;
pub mod spelling;
pub mod system;

use super::Page;
use super::cx::Cx;

/// The tabs of a page.
pub fn tabs(page: Page) -> Vec<&'static str> {
    match page {
        Page::Aurora => vec![
            lifetech_i18n::tr!("common-windows"),
            lifetech_i18n::tr!("ui-options-tab-world-and-interface"),
            lifetech_i18n::tr!("common-behavior"),
        ],
        Page::Graphics => vec![
            lifetech_i18n::tr!("ui-options-quality"),
            lifetech_i18n::tr!("common-light"),
            lifetech_i18n::tr!("ui-options-tab-sky-and-environment"),
            lifetech_i18n::tr!("ui-options-tab-reflections-and-water"),
            lifetech_i18n::tr!("common-image"),
            lifetech_i18n::tr!("ui-options-smoothness"),
            lifetech_i18n::tr!("region-parcels"),
        ],
        Page::Camera => vec![
            lifetech_i18n::tr!("common-viewpoint"),
            lifetech_i18n::tr!("common-behavior"),
            lifetech_i18n::tr!("common-mouselook"),
        ],
        Page::Controls => vec![lifetech_i18n::tr!("ui-options-page-controls"), lifetech_i18n::tr!("joystick-title")],
        Page::Avatars => vec![
            lifetech_i18n::tr!("ui-options-tab-name-tags"),
            lifetech_i18n::tr!("ui-options-names"),
            lifetech_i18n::tr!("ui-options-look-at-targets"),
            lifetech_i18n::tr!("ui-options-aurora-animations"),
        ],
        Page::Chat => vec![
            lifetech_i18n::tr!("common-nearby-chat"),
            lifetech_i18n::tr!("ui-options-aurora-notifications"),
            lifetech_i18n::tr!("ui-options-tab-keywords"),
            lifetech_i18n::tr!("ui-options-tab-history"),
            lifetech_i18n::tr!("ui-options-tab-away-and-replies"),
            lifetech_i18n::tr!("ui-options-groups-and-blocking"),
            lifetech_i18n::tr!("ui-options-tab-spelling"),
        ],
        Page::Sound => vec![
            lifetech_i18n::tr!("ui-options-volumes"),
            lifetech_i18n::tr!("ui-options-interface-sounds"),
            lifetech_i18n::tr!("common-voice"),
        ],
        Page::Layout => vec![
            lifetech_i18n::tr!("ui-options-tab-bars-and-windows"),
            lifetech_i18n::tr!("ui-options-hover-tips"),
            lifetech_i18n::tr!("ui-options-tab-startup"),
        ],
        Page::Appearance => vec![
            lifetech_i18n::tr!("ui-options-font"),
            lifetech_i18n::tr!("ui-options-colors"),
            lifetech_i18n::tr!("extra-skin"),
        ],
        Page::Shortcuts => shortcuts::tabs(),
        _ => vec![page.name()],
    }
}

/// The rows of one tab of a page.
pub fn draw(cx: &mut Cx, page: Page, tab: usize) {
    match page {
        Page::Aurora => aurora::page(cx, tab),
        Page::Graphics => graphics::page(cx, tab),
        Page::Camera => camera::page(cx, tab),
        Page::Avatars => avatars::page(cx, tab),
        Page::Build => build::page(cx),
        Page::Chat => chat::page(cx, tab),
        Page::Sound => sound::page(cx, tab),
        Page::Media => media::page(cx),
        Page::Privacy => privacy::page(cx),
        Page::Protections => protections::page(cx),
        Page::Rlva => rlva::page(cx),
        Page::Layout => layout::page(cx, tab),
        Page::Appearance => appearance::page(cx, tab),
        Page::Accessibility => accessibility::page(cx),
        Page::Confirmations => confirmations::page(cx),
        Page::Inventory => inventory::page(cx),
        Page::Controls => {
            if tab == 0 {
                controls::page(cx)
            } else {
                joystick::page(cx)
            }
        }
        Page::Shortcuts => shortcuts::page(cx, tab),
        Page::Commands => commands::page(cx),
        Page::System => system::page(cx),
        Page::Network => network::page(cx),
        Page::Fun => fun::page(cx),
    }
}
