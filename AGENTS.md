# AGENTS.md

Guide for AI agents working in the Silly repository.

## Project overview

Silly is a GPU-accelerated desktop application (intended to become a REST client) written in Rust using the [GPUI](https://www.gpui.rs/) UI framework. It is currently in early development (v0.1.0). The repository also contains a small static marketing/documentation site built with [Zola](https://www.getzola.org/) under `page/`.

The codebase is organized as a Rust workspace. The previous single-crate layout has been restructured into a small monorepo under `crates/`. The author uses a hobby-style workflow with `main`, `develop`, and feature branches (e.g. `vibe`).

## AI agent policy: mentor mode by default

This repository is a deliberate learning project. Contributors write their own code — agents working here must act as **mentors, not implementers**. Do not vibe code this project.

Unless the human explicitly opts out in the current session, an agent MUST:

1. **Never write or edit source code** (`crates/`, `page/`, `Cargo.toml`, CI). Reading is always allowed.
2. **Never paste drop-in implementations** of what the human is building. Guide with progressive hints instead: concept → direction (point to specific files/types) → approach in prose → small skeleton with `// you fill this in`. Escalate only when they remain stuck.
3. **Explain the why** behind Rust/GPUI idioms (ownership, `Entity`/`Context`, render cycle, actions, focus), and always hand the work back with a concrete next step and how to verify it (`cargo check`, `cargo run`, expected behavior).

The full protocol lives in the mentor skill: [`.claude/skills/mentor/SKILL.md`](.claude/skills/mentor/SKILL.md) (symlinked into `.agents/skills/mentor` and `.opencode/skills/mentor`). Load and follow it.

<!--**Opt-out:** only if the human explicitly confirms they want implementation (e.g. "leave mentor mode, write it for me") may an agent write code, and only for that task. Non-source meta files (docs, notes, agent configuration) may be written only when the human explicitly requests it. -->

## Repository structure

```text
/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock
├── crates/                 # Rust workspace members
│   ├── bin/                # Main desktop application binary (`silly`)
│   │   └── src/
│   │       ├── main.rs     # Entrypoint
│   │       ├── app.rs      # AppRunner: application setup, window, key bindings
│   │       ├── action.rs   # GPUI action definitions (Quit, ToggleSidebar)
│   │       ├── keybind.rs  # Global and scoped key bindings
│   │       ├── menu.rs     # macOS application menus
│   │       └── view/
│   │           ├── mod.rs  # View module + mock sidebar data
│   │           └── main_content.rs # Root view composing Sidebar + main area
│   ├── assets/             # Embedded asset loading (SVG icons, fonts, mock data)
│   │   └── src/assets.rs
│   ├── context/            # Domain model: spaces, profiles, sidebar nodes, tabs, config
│   │   └── src/
│   │       ├── context.rs  # Crate re-exports
│   │       ├── space.rs    # SpaceContext, ProfileContext, SidebarContext
│   │       ├── node.rs     # Node / NodeId tree for folders and tabs
│   │       ├── tab_data.rs # TabData (Browser, ApiRequest)
│   │       └── config/     # TOML config loading trait + SpaceContext impl
│   ├── settings/           # User settings crate (placeholder)
│   │   └── src/settings.rs
│   └── ui/                 # Reusable GPUI components
│       └── src/
│           ├── ui.rs       # Crate root
│           ├── sidebar.rs  # SidebarView with docked/floating/resize behavior
│           └── platform.rs # macOS-specific native helpers (traffic lights)
├── assets/                 # Embedded assets referenced from crates/assets
│   ├── icons/              # SVG icons loaded via gpui::svg().path("icons/...")
│   ├── fonts/              # Bundled TTF fonts (e.g. Pacifico)
│   └── mock/               # Mock data embedded by rust_embed
├── page/                   # Zola static site
│   ├── config.toml
│   ├── content/
│   ├── templates/
│   ├── static/
│   └── sass/
├── .github/workflows/      # CI/CD
│   ├── deploy.yml          # Build macOS release binary and GitHub Release
│   └── page.yml            # Build & deploy Zola site to gh-pages
├── .claude/skills/mentor/  # Mentor skill (canonical source; symlinked into .agents/skills/ and .opencode/skills/)
├── .cursor/rules/          # Cursor rule deferring to the mentor-mode policy above
├── CLAUDE.md               # Imports this file for Claude Code
├── README.md               # Basic run instructions
├── NOTE.md                 # Author note about future monorepo plans
└── ROADMAP.md              # Feature roadmap and status
```

## Essential commands

### Rust application

The workspace default member is `crates/bin`, so `cargo run` and `cargo build` target the desktop app by default.

```bash
# Run the desktop app locally (requires GPUI platform dependencies; see README.md)
cargo run

# Build release binary (macOS-only CI target)
cargo build --release --locked
```

There are very few configured tests at the time of writing, and no Makefile or custom build scripts.

### Zola site

The site is built and deployed by GitHub Actions. To build locally, install Zola and run from the `page/` directory:

```bash
zola build
# or for local preview
zola serve
```

## Architecture and control flow

### Entrypoint

`crates/bin/src/main.rs` delegates everything to `app::AppRunner::run()`.

### Application setup (`crates/bin/src/app.rs`)

`AppRunner::run()` performs the following:

1. Registers the embedded asset source (`assets::Assets`).
2. Loads bundled fonts (`assets::load_fonts_asset(cx)`).
3. Registers global key bindings (`crate::keybind::register_keybind(cx)`).
4. Initializes the macOS application menu (`crate::menu::init_app_menu(cx)`).
5. Initializes the default `gpui-component` theme (`theme::init(cx)`).
6. Registers the `Quit` action handler.
7. Opens a normal GPUI window wrapped in `gpui_component::Root`.
8. Instantiates `MainContent` as the window content.

### View hierarchy

```text
Application
 └─ Window
     └─ Root (from gpui-component)
         └─ MainContent (crates/bin/src/view/main_content.rs)
             ├─ SidebarView (crates/ui/src/sidebar.rs)
             │   ├─ docked sidebar area
             │   ├─ resize handle
             │   ├─ hover hot-zone (when hidden)
             │   └─ floating sidebar overlay
             └─ main content area
```

`MainContent` owns `gpui::Entity<ui::sidebar::SidebarView>` and `gpui::Entity<context::space::SpaceContext>`, and observes both for changes. The sidebar is driven by a `context::space::SidebarContext` entity that contains the node tree, favorites, and folders.

### Crate responsibilities

- **`crates/bin`**: Application entrypoint, window setup, actions, key bindings, menus, top-level views, and mock data.
- **`crates/assets`**: Compile-time embedded assets (`rust_embed`) and font loading helpers. Exposes `Assets` as a GPUI `AssetSource`.
- **`crates/context`**: Plain Rust domain types for spaces, profiles, sidebar nodes/folders, tab data, and TOML config loading. Has no GPUI dependency.
- **`crates/settings`**: Reserved for user settings/preferences; currently a placeholder with a minimal example test.
- **`crates/ui`**: GPUI components, including `SidebarView` and macOS platform helpers.

### GPUI patterns used

- **Entities**: Top-level stateful views are GPUI `Entity<T>` values. `MainContent` owns `Entity<SidebarView>` and `Entity<SpaceContext>` and observes them for changes.
- **Actions**: User commands are declared with the `actions!` macro in `crates/bin/src/action.rs` and dispatched via `cx.bind_keys`, `on_action`, and `cx.listener`.
- **Render traits**: Components implement `gpui::Render` or `gpui::RenderOnce` and return `impl IntoElement` built with the fluent GPUI element API (`div()`, `.child(...)`, etc.).
- **Key contexts**: `MainContent` sets `.key_context("main_view")` so the `cmd-s` shortcut only fires when that view has focus.
- **Focus**: `MainContent` implements `Focusable` and calls `.track_focus(&self.focus_handle)`.
- **Async animations**: Animation loops are spawned with `cx.spawn(async move |this, cx| { ... }).detach()` and use `cx.background_executor().timer(...)` to drive frame updates.

### Sidebar state machine

The sidebar in `crates/ui/src/sidebar.rs` (`SidebarView`) has two related states:

- **Docked**: open or closed with animated `width_anim`.
- **Floating**: appears when the window is in hidden-docked mode and the user hovers the left edge. Driven by `floating_visible` and `floating_progress`.

Important transition rule (in `SidebarView::toggle`):

- If the floating sidebar is currently visible while the docked sidebar is hidden, toggling restores the docked sidebar immediately without playing the floating slide-out.
- Otherwise it toggles the docked hidden state and disables the floating sidebar.

The main content area listens to `MouseMoveEvent` to hide the floating sidebar when the cursor is over the main area.

### Domain model (`crates/context`)

- **`SpaceContext`**: Represents a workspace/space, including a profile, name, number, and sidebar state.
- **`ProfileContext` / `ProfileId`**: Identifies a user profile with a pseudonym and optional email.
- **`SidebarContext`**: Holds the sidebar node tree as `HashMap<NodeId, Node>`, plus ordered `folder` and `favorites` lists.
- **`Node` / `NodeId`**: Generic tree node. `NodeData` is either a `Folder` (with child IDs and expand flag) or a `Tab` (with `TabData` and open flag).
- **`TabData`**: Either a `Browser` tab or an `ApiRequest` tab with URL, method, params, body, headers, and authorization.
- **`config`**: A small `Config` trait for loading TOML files into typed structs (currently implemented for `SpaceContext`).

### macOS platform integration (`crates/ui/src/platform.rs`)

GPUI does not expose hiding the window traffic-light buttons, so `crates/ui/src/platform.rs` uses Objective-C runtime calls via the `objc2`/`raw-window-handle` crates (macOS-only dependencies) to find the `NSWindow` and set the standard close/minimize/zoom buttons hidden. This is a no-op on non-macOS platforms.

The sidebar calls this during render to hide traffic lights when the sidebar is fully collapsed and no floating sidebar is visible:

```rust
set_traffic_lights_hidden(window, self.width_anim < 1.0 && self.floating_progress <= 0.0);
```

## Asset handling

Assets are embedded at compile time with `rust_embed` inside `crates/assets/src/assets.rs`:

- The `Assets` struct embeds `../../assets/icons/*.svg` and `../../assets/mock/**/*.md` and exposes them through GPUI's `AssetSource` trait.
- SVGs are referenced by path, e.g. `svg().path("icons/sidebar-left.svg")`.
- Fonts are included with `include_bytes!` and loaded into the GPUI text system at startup via `load_fonts_asset`.
- `FONT_SUPPORT` is a hard-coded array; adding a new bundled font means adding an entry there.

## Code conventions

- Rust edition 2024.
- Workspace dependencies are declared in the root `Cargo.toml` and referenced with `*.workspace = true` in member crates.
- Import style: `use gpui::{..., prelude::*};` followed by crate-relative imports (`crate::action::...`) or workspace crate imports (`context::...`, `ui::...`).
- GPUI element chains are heavily indented and chained; keep this style.
- Hex colors are written as `rgb(0x636080)` and sizes with `px(...)`.
- Component builders follow the `new()` + consuming setter pattern, then implement `Render` or `RenderOnce`.
- State IDs are sometimes attached to elements for debugging (`#sidebar-hover-zone`, `#floating-sidebar`, `#sidebar-resize-handle`).

## Testing approach

The project has very little test coverage. `crates/settings/src/settings.rs` contains a minimal example unit test, but most logic (UI, domain model, request handling) is not yet tested. When adding testable logic (e.g. request handling, profile switching, sidebar state transitions, config parsing), prefer plain Rust modules/functions and add tests under the relevant crate's `src/` or in a `tests/` directory as appropriate.

## CI/CD

### Release binary (`deploy.yml`)

- Triggers on tags `v*` or manually.
- Runs on `macos-latest` (`aarch64-apple-darwin`).
- Builds with `cargo build --release --locked` (workspace default member is `crates/bin`).
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
2. **macOS-only native code.** `crates/ui/src/platform.rs` is the only place that reaches into AppKit. Keep it isolated; changes there should not leak into GPUI view code.
3. **Release CI only targets Apple Silicon macOS.** Do not expect Linux/Windows release binaries from the current workflows.
4. **Little test coverage.** Verify UI changes by running `cargo run` and manually exercising the UI.
5. **Keep the build warning-free.** The codebase currently compiles with zero warnings; keep it that way so new warnings stay meaningful.
6. **The restructure to a workspace is recent.** The previous flat `src/` layout has been replaced by `crates/`. If you encounter old paths in notes or memory, verify against the current workspace layout.
7. **Roadmap is the source of truth for features.** Check `ROADMAP.md` before starting major work; many core features (REST requests, profiles, auth, history, cache) are still "Plan".
