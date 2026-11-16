//! Find and replace in a notecard (LifeTech): the script editor's search
//! (LLFloaterSearchReplace, Firestorm, LGPL 2.1, `ui/scripts/dialogs.rs`) as
//! a bar over the notecard's text. Ctrl+F or the search button opens it;
//! Enter finds the next match, Shift+Enter the previous one, and F3 /
//! Shift+F3 do the same from the text itself. The match found is selected
//! in the text, which takes the keyboard (egui only shows a selection in
//! the focused field); Ctrl+F goes back to the search field. A replacement
//! goes through the editor's own undo history, so Ctrl+Z takes it back, and
//! never removes or adds an embedded item: a match that covers one is
//! skipped, and such characters typed in the replacement are dropped.
//!
//! In the Markdown view the text shown is the rendered one, read-only: Find
//! searches and selects it there, and a replacement changes the notecard's
//! source, the n-th match shown standing for the n-th match of the source.

use crate::{
    theme::Palette,
    ui::{
        scripts::editing,
        widgets::{flat_button, icon_button},
    },
};
use egui::{Id, RichText};
use lifetech_net::inventory::notecard::{FIRST_EMBEDDED_CHAR, Notecard};
use std::ops::Range;

#[derive(Default)]
pub(super) struct NoteFind {
    pub open: bool,
    query: String,
    replacement: String,
    match_case: bool,
    status: String,
    focus: bool,
}

/// The bytes of a character range of `text`.
fn char_bytes(text: &str, chars: Range<usize>) -> Range<usize> {
    let at = |n: usize| text.char_indices().nth(n).map_or(text.len(), |(i, _)| i);
    at(chars.start)..at(chars.end)
}

fn embedded(c: char) -> bool {
    c as u32 >= FIRST_EMBEDDED_CHAR
}

/// The matches of `query` in `text` (character ranges) that hold no
/// embedded item.
fn matches(text: &str, query: &str, match_case: bool) -> Vec<Range<usize>> {
    let chars: Vec<char> = text.chars().collect();
    editing::matches(text, query, !match_case)
        .into_iter()
        .filter(|r| !chars[r.clone()].iter().copied().any(embedded))
        .collect()
}

/// `text` with every match replaced, and how many were. `None` when the
/// result would not fit in a notecard.
fn replaced_all(text: &str, query: &str, replacement: &str, match_case: bool) -> Option<(String, usize)> {
    let ranges = matches(text, query, match_case);
    let boundaries: Vec<usize> = text.char_indices().map(|(i, _)| i).chain([text.len()]).collect();
    let mut output = String::with_capacity(text.len());
    let mut previous = 0;
    for range in &ranges {
        output.push_str(&text[previous..boundaries[range.start]]);
        output.push_str(replacement);
        previous = boundaries[range.end];
    }
    output.push_str(&text[previous..]);
    Notecard::validate_text(&output).ok().map(|()| (output, ranges.len()))
}

/// Where Find searches and selects: the source editor, or the Markdown
/// view's rendered text (`preview`) in its own read-only editor.
fn target(editor: Id, text: &str, preview: Option<&str>) -> (Id, String) {
    match preview {
        Some(shown) => (editor.with("markdown"), shown.to_owned()),
        None => (editor, text.to_owned()),
    }
}

impl NoteFind {
    fn field_ids(editor: Id) -> [Id; 2] {
        [editor.with("find_query"), editor.with("find_replacement")]
    }

    /// Opens the bar with the selected text as the query, its field focused.
    pub fn begin(&mut self, ctx: &egui::Context, editor: Id, text: &str, preview: Option<&str>) {
        let (shown_id, shown) = target(editor, text, preview);
        let selected = editing::bytes(&shown, editing::cursor(ctx, shown_id));
        let selected = &shown[selected];
        if !selected.is_empty() && !selected.contains('\n') {
            self.query = selected.chars().filter(|c| !embedded(*c)).collect();
        }
        self.open = true;
        self.focus = true;
        self.status.clear();
    }

    /// Ctrl+F for this notecard window: its editor or the bar's fields
    /// have the keyboard, or the pointer is over the window.
    pub fn shortcut(&mut self, ui: &egui::Ui, editor: Id, text: &str, preview: Option<&str>) {
        let [query, replacement] = Self::field_ids(editor);
        if crate::ui::find_shortcut::take(ui, &[editor, editor.with("markdown"), query, replacement]) {
            self.begin(ui.ctx(), editor, text, preview);
        }
    }

    /// The bar, when open. True when the text changed.
    pub fn bar(&mut self, ui: &mut egui::Ui, p: &Palette, editor: Id, text: &mut String, preview: Option<&str>, editable: bool) -> bool {
        if !self.open {
            return false;
        }
        let ctx = ui.ctx().clone();
        let [query_id, replacement_id] = Self::field_ids(editor);
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            let query = crate::ui::widgets::search_text_edit(ui, p, &mut self.query, |edit| {
                edit.id(query_id)
                    .hint_text(lifetech_i18n::tr!("script-search-query"))
                    .desired_width(160.0)
            });
            if !ui.is_sizing_pass() && std::mem::take(&mut self.focus) {
                query.request_focus();
                editing::select(&ctx, query_id, 0..self.query.chars().count(), true);
            }
            if (query.has_focus() || query.lost_focus()) && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let up = ui.input(|i| i.modifiers.shift);
                ui.input_mut(|i| {
                    i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
                    i.consume_key(egui::Modifiers::SHIFT, egui::Key::Enter);
                });
                self.find(&ctx, editor, text, preview, up);
            }
            // F3 / Shift+F3: the next or previous match, with the keyboard
            // in this notecard's text or in the bar
            let ours = ctx.memory(|m| {
                m.focused()
                    .is_some_and(|f| [editor, editor.with("markdown"), query_id, replacement_id].contains(&f))
            });
            if ours && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F3)) {
                self.find(&ctx, editor, text, preview, false);
            } else if ours && ui.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, egui::Key::F3)) {
                self.find(&ctx, editor, text, preview, true);
            }
            if editable {
                crate::ui::widgets::text_field(ui, true, &mut self.replacement, |b, _| {
                    egui::TextEdit::singleline(b)
                        .id(replacement_id)
                        .hint_text(lifetech_i18n::tr!("script-search-replacement"))
                        .desired_width(160.0)
                });
            }
            crate::ui::widgets::checkbox(ui, p, &mut self.match_case, lifetech_i18n::tr!("notecard-find-match-case"));
            if flat_button(ui, p, lifetech_i18n::tr!("script-search-find")).clicked() {
                self.find(&ctx, editor, text, preview, false);
            }
            if editable {
                ui.add_enabled_ui(!self.query.is_empty(), |ui| {
                    if flat_button(ui, p, lifetech_i18n::tr!("script-search-replace")).clicked() {
                        changed |= self.replace(&ctx, editor, text, preview);
                    }
                    if flat_button(ui, p, lifetech_i18n::tr!("script-search-replace-all")).clicked() {
                        changed |= self.replace_all(&ctx, editor, text);
                    }
                });
            }
            if !self.status.is_empty() {
                ui.add(egui::Label::new(RichText::new(&self.status).color(p.muted)).wrap_mode(egui::TextWrapMode::Extend));
            }
            if icon_button(ui, p, "x", lifetech_i18n::tr!("common-close"), true)
                || (query.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)))
            {
                self.open = false;
                ctx.memory_mut(|m| m.request_focus(target(editor, "", preview).0));
            }
        });
        changed
    }

    pub fn find(&mut self, ctx: &egui::Context, editor: Id, text: &str, preview: Option<&str>, up: bool) {
        let (shown_id, shown) = target(editor, text, preview);
        let found = matches(&shown, &self.query, self.match_case);
        let range = editing::cursor(ctx, shown_id).sorted_cursors();
        if let Some(next) = editing::next_match(&found, range[0].index.0..range[1].index.0, up) {
            editing::select(ctx, shown_id, next, true);
            self.status.clear();
        } else {
            self.status = lifetech_i18n::tr!("script-search-not-found").into();
        }
    }

    /// The selected match takes the replacement, then the next one is found.
    pub fn replace(&mut self, ctx: &egui::Context, editor: Id, text: &mut String, preview: Option<&str>) -> bool {
        let (shown_id, shown) = target(editor, text, preview);
        let range = editing::cursor(ctx, shown_id).sorted_cursors();
        let selected = range[0].index.0..range[1].index.0;
        let replacement = self.clean_replacement();
        let mut changed = false;
        // the selected match, as the match of the source it stands for
        let source_match = matches(&shown, &self.query, self.match_case)
            .iter()
            .position(|r| *r == selected)
            .and_then(|n| matches(text, &self.query, self.match_case).into_iter().nth(n));
        if let Some(selected) = source_match {
            let mut after = text.clone();
            let bytes = char_bytes(text, selected.clone());
            after.replace_range(bytes, &replacement);
            if Notecard::validate_text(&after).is_err() {
                self.status = lifetech_i18n::tr!("notecard-text-too-large").into();
                return false;
            }
            editing::replace_any(ctx, editor, text, selected.clone(), &replacement);
            changed = true;
            if preview.is_some() {
                // the rendered text follows a frame later: go on from there
                let at = editing::cursor(ctx, shown_id).sorted_cursors()[1].index.0;
                editing::select(ctx, shown_id, at..at, false);
            }
        }
        if preview.is_none() {
            self.find(ctx, editor, text, None, false);
        }
        changed
    }

    /// Every match at once, as one step of the undo history.
    pub fn replace_all(&mut self, ctx: &egui::Context, editor: Id, text: &mut String) -> bool {
        let replacement = self.clean_replacement();
        match replaced_all(text, &self.query, &replacement, self.match_case) {
            Some((_, 0)) => {
                self.status = lifetech_i18n::tr!("script-search-not-found").into();
                false
            }
            Some((output, count)) => {
                let all = 0..text.chars().count();
                editing::replace_any(ctx, editor, text, all, &output);
                self.status = lifetech_i18n::tr!("script-search-replaced", count = count as i64);
                true
            }
            None => {
                self.status = lifetech_i18n::tr!("notecard-text-too-large").into();
                false
            }
        }
    }

    /// Whether the query field has the keyboard (demo log).
    pub fn query_focused(&self, ctx: &egui::Context, editor: Id) -> bool {
        ctx.memory(|m| m.has_focus(Self::field_ids(editor)[0]))
    }

    /// The demo's Replace all (`AURORA_DEMO_INVENTORY=note-find`).
    pub fn demo_replace_all(&mut self, ctx: &egui::Context, editor: Id, text: &mut String, query: &str, replacement: &str) -> bool {
        self.query = query.into();
        self.replacement = replacement.into();
        self.replace_all(ctx, editor, text)
    }

    fn clean_replacement(&self) -> String {
        self.replacement.chars().filter(|c| !embedded(*c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn marker(n: u32) -> char {
        char::from_u32(FIRST_EMBEDDED_CHAR + n).expect("an embedded marker")
    }

    #[test]
    fn replace_all_counts_and_keeps_the_rest() {
        let (out, n) = replaced_all("Hello hello HELLO", "hello", "bye", false).expect("fits");
        assert_eq!((out.as_str(), n), ("bye bye bye", 3));
        let (out, n) = replaced_all("Hello hello HELLO", "hello", "bye", true).expect("fits");
        assert_eq!((out.as_str(), n), ("Hello bye HELLO", 1));
        let (out, n) = replaced_all("née née", "é", "e", false).expect("fits");
        assert_eq!((out.as_str(), n), ("nee nee", 2));
        assert_eq!(replaced_all("abc", "x", "y", false).map(|(_, n)| n), Some(0));
    }

    #[test]
    fn a_match_over_an_embedded_item_is_left_alone() {
        let text = format!("a{}b ab", marker(0));
        let found = matches(&text, &format!("a{}b", marker(0)), false);
        assert!(found.is_empty(), "the item is never replaced away");
        let (out, n) = replaced_all(&text, "ab", "x", false).expect("fits");
        assert_eq!((out, n), (format!("a{}b x", marker(0)), 1));
    }

    #[test]
    fn replace_all_refuses_a_text_too_large_for_a_notecard() {
        let text = "a".repeat(40_000);
        assert!(replaced_all(&text, "a", "bb", false).is_none());
        assert!(replaced_all(&text, "a", "b", false).is_some());
    }
}
