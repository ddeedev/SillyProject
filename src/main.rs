#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod component;
mod platform;
mod view;
mod assets;

fn main() {
    app::AppRunner::run();
}
