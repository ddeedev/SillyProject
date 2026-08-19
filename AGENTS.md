# AGENTS.md

Guide for AI agents working in the Silly repository.

## Project overview

Silly is a GPU-accelerated desktop application (intended to become a REST client) written in Rust using the [GPUI](https://www.gpui.rs/) UI framework. It is currently in early development (v0.1.0). The repository also contains a small static marketing/documentation site built with [Zola](https://www.getzola.org/) under `page/`.

The primary codebase is a single Cargo binary crate. The author uses a hobby-style workflow with `main`, `develop`, and feature branches (e.g. `vibe`).

## Repository structure

```text
/
├── Cargo.toml              # Rust package manifest
├── Cargo.lock
├── src/                    # Main Rust application
│   ├── main.rs             # Entrypoint
│   ├── app.rs              # AppRunner: application setup, window, key bindings
│   ├── action.rs           # GPUI action definitions (Quit, ToggleSidebar)
│   ├── assets.rs           # Embedded asset loading (SVG icons, fonts)
│   ├── platform.rs         # macOS-specific native helpers (traffic lights)
│   ├── components/         # Reusable UI components
│   │   └── sidebar.rs      # Sidebar component with docked/floating/resize behavior
│   ├── view/               # Top-level view controllers
│   │   └── main_content.rs # Root view composing Sidebar + main area
│   └── ui/                 # Reserved module, currently empty
├── assets/                 # Embedded assets referenced from src/assets.rs
│   ├── icons/              # SVG icons loaded via gpui::svg().path("icons/...")
│   └── fonts/              # Bundled TTF fonts (e.g. Pacifico)
├── page/                   # Zola static site
│   ├── config.toml
│   ├── content/
│   ├── templates/
│   ├── static/
│   └── sass/
├── .github/workflows/      # CI/CD
│   ├── deploy.yml          # Build macOS release binary and GitHub Release
│   └── page.yml            # Build & deploy Zola site to gh-pages
├── README.md               # Basic run instructions
├── NOTE.md                 # Author note about future monorepo plans
└── ROADMAP.md              # Feature roadmap and status
```

## Essential commands

### Rust application

```bash
# Run the desktop app locally (requires GPUI platform dependencies; see README.md)
cargo run

# Build release binary (macOS-only CI target)
cargo build --release --locked
```

There are no configured `cargo test` tests at the time of writing, and no Makefile or custom build scripts.

### Zola site

The site is built and deployed by GitHub Actions. To build locally, install Zola and run from the `page/` directory:

```bash
zola build
# or for local preview
zola serve
```

## Architecture and control flow

### Entrypoint

`src/main.rs` delegates everything to `app::AppRunner::run()`.

### Application setup (`src/app.rs`)

`AppRunner::run()` performs the following:

1. Registers the embedded asset source (`crate::assets::Assets`).
2. Loads bundled fonts (`load_fonts_asset(cx)`).
3. Initializes the default `gpui-component` theme (`theme::init(cx)`).
4. Registers global actions (`Quit`, `ToggleSidebar`) and key bindings:
   - `cmd-q` → `Quit`
   - `cmd-s` → `ToggleSidebar` scoped to the `main_view` key context
5. Opens a normal GPUI window wrapped in `gpui_component::Root`.
6. Instantiates `MainContent` as the window content.

### View hierarchy

```
Application
 └─ Window
     └─ Root (from gpui-component)
         └─ MainContent (src/view/main_content.rs)
             ├─ Sidebar (src/components/sidebar.rs)
             │   ├─ docked sidebar area
             │   ├─ resize handle
             │   ├─ hover hot-zone (when hidden)
             │   └─ floating sidebar overlay
             └─ main content area
```

### GPUI patterns used

- **Entities**: Top-level stateful views are GPUI `Entity<T>` values. `MainContent` owns `Entity<Sidebar>` and observes it for changes.
- **Actions**: User commands are declared with the `actions!` macro in `src/action.rs` and dispatched via `cx.bind_keys`, `on_action`, and `cx.listener`.
- **Render traits**: Components implement `gpui::Render` or `gpui::RenderOnce` and return `impl IntoElement` built with the fluent GPUI element API (`div()`, `.child(...)`, etc.).
- **Key contexts**: `MainContent` sets `.key_context("main_view")` so the `cmd-s` shortcut only fires when that view has focus.
- **Focus**: `MainContent` implements `Focusable` and calls `.track_focus(&self.focus_handle)`.
- **Async animations**: Animation loops are spawned with `cx.spawn(async move |this, cx| { ... }).detach()` and use `cx.background_executor().timer(...)` to drive frame updates.

### Sidebar state machine

The sidebar in `src/components/sidebar.rs` has two related states:

- **Docked**: open or closed with animated `width_anim`.
- **Floating**: appears when the window is in hidden-docked mode and the user hovers the left edge. Driven by `floating_visible` and `floating_progress`.

Important transition rule (in `Sidebar::toggle`):

- If the floating sidebar is currently visible while the docked sidebar is hidden, toggling restores the docked sidebar immediately without playing the floating slide-out.
- Otherwise it toggles the docked hidden state and disables the floating sidebar.

The main content area listens to `MouseMoveEvent` to hide the floating sidebar when the cursor is over the main area.

### macOS platform integration (`src/platform.rs`)

GPUI does not expose hiding the window traffic-light buttons, so `src/platform.rs` uses Objective-C runtime calls via the `objc`/`cocoa`/`raw-window-handle` crates to find the `NSWindow` and set the standard close/minimize/zoom buttons hidden. This is a no-op on non-macOS platforms.

The sidebar calls this during render to hide traffic lights when the sidebar is fully collapsed and no floating sidebar is visible:

```rust
set_traffic_lights_hidden(window, self.width_anim < 1.0 && self.floating_progress <= 0.0);
```

## Asset handling

Assets are embedded at compile time with `rust_embed`:

- `Assets` struct in `src/assets.rs` embeds `./assets/icons/*.svg` and exposes them through GPUI's `AssetSource` trait.
- SVGs are referenced by path, e.g. `svg().path("icons/sidebar-left.svg")`.
- Fonts are included with `include_bytes!` and loaded into the GPUI text system at startup via `load_fonts_asset`.
- `FONT_SUPPORT` is a hard-coded array; adding a new bundled font means adding an entry there.

## Code conventions

- Rust edition 2024.
- Import style: `use gpui::{..., prelude::*};` followed by crate-relative imports (`crate::action::...`).
- GPUI element chains are heavily indented and chained; keep this style.
- Hex colors are written as `rgb(0x636080)` and sizes with `px(...)`.
- Component builders follow the `new()` + consuming setter pattern (`width(...)`, `toggle_sidebar(...)`), then implement `RenderOnce`.
- State IDs are sometimes attached to elements for debugging (`#sidebar-hover-zone`, `#floating-sidebar`, `#sidebar-resize-handle`).

## Testing approach

There are currently no unit tests, integration tests, or UI tests in the repository. When adding testable logic (e.g. request handling, profile switching, sidebar state transitions), prefer plain Rust modules/functions and add tests under `src/` or in a `tests/` directory as appropriate.

## CI/CD

### Release binary (`deploy.yml`)

- Triggers on tags `v*` or manually.
- Runs on `macos-latest` (`aarch64-apple-darwin`).
- Builds with `cargo build --release --locked`.
- Packages `target/release/silly` into `dist/silly-aarch64-apple-darwin.tar.gz`.
- For tag pushes, attaches the archive to a GitHub Release.
- For manual runs, uploads it as an Actions artifact.
- Note: the produced binary is unsigned; Gatekeeper will block it until the user right-clicks → Open.

### Static site (`page.yml`)

- Triggers on pushes to `develop` that touch `page/**` or `.github/workflows/page.yml`, and manually.
- Uses `shalzz/zola-deploy-action@v0.19.2` to build `page/` and deploy to `gh-pages`.
- Site base URL is configured as `https://ddeedev.github.io/SillyProject` in `page/config.toml`.

## Branches

- `main`: default branch on remote.
- `develop`: integration branch; Zola site deploys from here.
- Feature branches may exist locally (e.g. `vibe`).

## Important gotchas

1. **GPUI dependency has native platform requirements.** The app will not build/run without the GPUI prerequisites. See README.md for the link to gpui.rs installation instructions.
2. **macOS-only native code.** `src/platform.rs` is the only place that reaches into AppKit. Keep it isolated; changes there should not leak into GPUI view code.
3. **Release CI only targets Apple Silicon macOS.** Do not expect Linux/Windows release binaries from the current workflows.
4. **No tests exist yet.** Verify changes by running `cargo run` and manually exercising the UI.
5. **Unused imports currently generate warnings.** The codebase currently has compiler warnings from unused imports; do not treat a clean warning-free build as required unless asked.
6. **Future monorepo planned.** `NOTE.md` mentions moving the "Folder" feature into a new crate and making this a Rust monorepo. If you add substantial new functionality, consider whether it should live in a new crate under `crates/` (currently empty).
7. **Roadmap is the source of truth for features.** Check `ROADMAP.md` before starting major work; many core features (REST requests, profiles, auth, history, cache) are still "Plan".
