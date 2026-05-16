use crate::action::{Quit, ToggleSidebar};
use crate::view::main_content::MainContent;
use gpui::{
    App, Application, KeyBinding, TitlebarOptions, WindowBackgroundAppearance, WindowKind,
    WindowOptions, prelude::*,
};
use gpui_component::{Root, theme};

pub struct AppRunner;

impl AppRunner {
    pub fn run() {
        Application::new().run(|cx: &mut App| {
            theme::init(cx);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([
                KeyBinding::new("cmd-q", Quit, None),
                KeyBinding::new("cmd-s", ToggleSidebar, Some("main_view")),
            ]);

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let titlebar = TitlebarOptions {
                appears_transparent: true,
                title: None,
                ..Default::default()
            };
            let window_option = WindowOptions {
                titlebar: Some(titlebar),
                kind: WindowKind::Normal,
                is_movable: true,
                window_background: WindowBackgroundAppearance::Opaque,
                ..Default::default()
            };

            cx.open_window(window_option, |window, cx| {
                let main_conten = cx.new(MainContent::new);
                cx.new(|cx| Root::new(main_conten, window, cx))
            })
            .unwrap();
        });
    }
}
