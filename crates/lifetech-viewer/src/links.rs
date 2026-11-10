//! Project links shown on the login screen. Empty = not configured yet: the
//! button is shown disabled and nothing is fetched. The README header carries
//! the same Discord and website links: change them there too.
//!
//! LifeTech: the source and the issues are LifeTech Viewer's own repository.
//! Aurora Viewer's other community links (wiki, Discord, privacy policy,
//! news) are not LifeTech's, so they stay empty until LifeTech has its own.

/// Source code repository (GitHub page).
pub const GITHUB: &str = "https://github.com/LethalCyborg/LifeTech-Viewer";
/// Issue template chooser, opened in the browser by "Report a bug" (login
/// screen, top bar, Help menu) and by the crash window.
pub const NEW_ISSUE: &str = "https://github.com/LethalCyborg/LifeTech-Viewer/issues/new/choose";
/// The project's wiki, generated from the guides of `docs/`
/// (`scripts/build-wiki.ps1`): Help › Aurora Viewer wiki.
pub const WIKI: &str = "";
/// Community Discord invite.
pub const DISCORD: &str = "";
/// Project website.
pub const WEBSITE: &str = "https://lifetech.systems";
/// The privacy policy on the website (Preferences › Privacy › Your data).
pub const PRIVACY_POLICY: &str = "";
/// News feed: this folder of `main`, hand-edited Markdown files, one
/// `<language>.md` per UI language (`news/` at the repository root). The
/// viewer keeps what it read in its cache as a JSON array of
/// `{"date", "title", "summary", "url"}`.
pub const NEWS_FEED: &str = "";
/// Files of the source repository as GitHub serves them unrendered; a branch
/// name and a path follow.
pub const RAW_FILES: &str = "https://raw.githubusercontent.com/LethalCyborg/LifeTech-Viewer/";
