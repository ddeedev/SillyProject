#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod assets;
mod components;
mod keybind;
mod menu;
mod platform;
mod ui;
mod view;

fn main() {
    app::AppRunner::run();
}
