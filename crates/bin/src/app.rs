use crate::action::Quit;
use crate::keybind::register_keybind;
use crate::menu::init_app_menu;
use crate::view::main_content::MainContent;
use assets::{Assets, load_fonts_asset};
use gpui::{
    App, Application, TitlebarOptions, WindowBackgroundAppearance, WindowKind, WindowOptions,
    point, prelude::*, px,
};
use gpui_component::{Root, theme};

pub struct AppRunner;

impl AppRunner {
    pub fn run() {
        Application::new().with_assets(Assets).run(|cx: &mut App| {
            // load asset
            load_fonts_asset(cx);
            register_keybind(cx);
            init_app_menu(cx);

            theme::init(cx);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let titlebar = TitlebarOptions {
                traffic_light_position: Some(point(px(12.), px(12.))),
                appears_transparent: true,
                title: None,
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
            cx.activate(true);
        });
    }
}
