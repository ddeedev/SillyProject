#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod keybind;
mod menu;
mod view;

fn main() {
    app::AppRunner::run();
}
