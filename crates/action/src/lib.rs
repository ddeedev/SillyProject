use gpui::{Action, App, KeyBinding, actions};

actions!(app, [ToggleSidebar, Quit, CloseTab, ResetSidebar]);

#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = app, no_json)]
pub struct SwitchSpace(pub usize);

pub fn register_keybind(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-w", CloseTab, Some("main_view")),
        KeyBinding::new("cmd-s", ToggleSidebar, Some("main_view")),
        KeyBinding::new("ctrl-0", ResetSidebar, None),
        // switch space globally
        KeyBinding::new("ctrl-1", SwitchSpace(1), None),
        KeyBinding::new("ctrl-2", SwitchSpace(2), None),
        KeyBinding::new("ctrl-3", SwitchSpace(3), None),
    ]);
}

// Action	macOS Shortcut	Windows Shortcut
// New tab	Command-T	Control-T
// New window	Command-N	Control-N
// New incognito window	Command-Shift-N	Control-Shift-N
// Close current tab or window	Command-W	Control-W
// Open Little Arc	Command-Option-N	n/a
// Re-open last closed tab	Command-Shift-T	Control-Shift-T
// Pin/Unpin current tab	Command-D	Control-D
// Copy current tab URL	Command-Shift-C	Control-Shift-C
// Copy current tab URL as Markdown	Command-Shift-Option-C	Control-Shift-Alt-C
// Change current tab URL	Command-L	Control-L or Alt-D
// Show/Hide the Sidebar	Command-S	Control-S
// Clear unpinned tabs	Command-Shift-K	Control-Shift-K
// Go directly to tab N	Command-1 , Command-2, Command-3 ...	Control-1 , Control-2, Control-3 ...
// Focus on Space N	Control-1, Control-2, Control-3 …	Alt-1, Alt-2, Alt-3 …
// Toggle between recent tabs	Control-Tab	Control-Tab
// Switch between tabs	Command-Option-Up Arrow (↑) or Option-Command-Down Arrow (↓)	Control-Alt-Up Arrow (↑) or Alt-Control-Down Arrow (↓)
// Switch between Spaces	Command-Option-Left Arrow (←) or Option-Command-Right Arrow (→)	Control-Alt-Left Arrow (←) or Alt-Control-Right Arrow (→)
// Go forward on tab history	Command-Right Arrow (→) or Command-Right Bracket (])	Alt-Right Arrow (→)
// Go back on tab history	Command-Left Arrow (←) or Command-Left Bracket ([)	Alt-Left Arrow (←)
// Add Split View	Control-Shift-Plus Sign (+)	Control-Shift-Plus Sign (+)
// Close Split View	Control-Shift-Minus Sign (-)	Control-Shift-Minus Sign (-)
// Switch Split View focus	Control-Shift-1, Control-Shift-2, ...	Control-Shift-1, Control-Shift-2, ...
// View History	Command-Y	Control-H
// Zoom in webpage	Command-Plus Sign (+)	Control-Plus Sign (+)
// Zoom out webpage	Command-Minus Sign (-)	Control-Minus Sign (-)
// Reset webpage zoom	Command-Zero	Control-Zero
// Reload webpage	Command-R	Control-R
// Find in webpage	Command-F	Control-F/
//
// ┌────────────┬────────────┬────────────┐
// │ Mac symbol │    Key     │ GPUI token │
// ├────────────┼────────────┼────────────┤
// │ ⌃ ^        │ Control    │ ctrl       │
// ├────────────┼────────────┼────────────┤
// │ ⌘          │ Command    │ cmd        │
// ├────────────┼────────────┼────────────┤
// │ ⌥          │ Option/Alt │ alt        │
// ├────────────┼────────────┼────────────┤
// │ ⇧          │ Shift      │ shift      │
// └────────────┴────────────┴────────────┘
