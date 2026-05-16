#![cfg_attr(target_family = "wasm", no_main)]
mod action;
mod app;
mod component;

fn main() {
    app::AppRunner::run();
}
