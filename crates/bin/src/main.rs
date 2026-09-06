#![cfg_attr(target_family = "wasm", no_main)]
mod app;
mod menu;
mod view;

fn main() {
    app::AppRunner::run();
}
