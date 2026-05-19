#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod assets;
mod component;
mod platform;
mod ui;
mod view;

fn main() {
    app::AppRunner::run();
}
