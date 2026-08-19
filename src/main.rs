#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod assets;
mod components;
mod platform;
mod ui;
mod view;
mod keybind;
mod menu;

fn main() {
    app::AppRunner::run();
}
