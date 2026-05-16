use gpui::{
    App, Application, Bounds, Context, FocusHandle, Focusable, KeyBinding, SharedString,
    TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
    div, point, prelude::*, px, rgb, size,
};

struct MainContent {
    text: SharedString,
    sidebar_hidden: bool,
    focus_handle: FocusHandle,
}

impl MainContent {

}
