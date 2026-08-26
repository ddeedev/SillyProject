#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod components;
mod keybind;
mod menu;
mod platform;
mod ui;
mod view;
use settings;

fn main() {
    app::AppRunner::run();
}
