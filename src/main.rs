#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod component;
mod platform;
mod view;

fn main() {
    app::AppRunner::run();
}
