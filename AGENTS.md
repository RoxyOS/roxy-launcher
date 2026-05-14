# Repository Guidelines

## Project Structure & Module Organization
`src/` contains the application crate. `main.rs` boots the native `eframe` app, while `lib.rs` exposes shared modules. Core launcher logic lives in `src/core/`, UI widgets live in `src/ui/`, and domain models such as `game.rs`, `profile.rs`, and `app.rs` sit at the crate root. Shared helpers are grouped under `src/utils/`. Static icons and app images are stored in `assets/`. Nix-based development environment files live in `flake.nix` and `flake.lock`.

## Build, Test, and Development Commands
Use Cargo for the normal workflow:

- `cargo run` starts the desktop launcher locally.
- `cargo check --workspace --all-targets` verifies the crate compiles quickly.
- `cargo test --workspace --all-targets --all-features` runs unit tests.
- `cargo fmt --all` formats the codebase with `rustfmt`.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::all` enforces lint cleanliness.
- `./check.sh` runs the same CI-style sequence used by the repo: check, fmt, clippy, unit tests, and doc tests.

If you use Nix, enter the pinned toolchain with `nix develop`.

## Coding Style & Naming Conventions
Follow standard Rust formatting with 4-space indentation and let `cargo fmt` decide layout. Keep modules and files in `snake_case`; use `CamelCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants. Prefer small focused modules, and keep UI code inside `src/ui/` rather than mixing it into launcher logic. This crate treats Clippy warnings as errors during checks, so fix lints before opening a PR.

## Testing Guidelines
Tests currently live inline under `#[cfg(test)]` modules inside the relevant source files, for example in `src/game.rs` and `src/profile.rs`. Add new tests close to the code they cover and name them after observable behavior, such as `launch_button_is_disabled_while_running`. Run `cargo test` for quick validation and `./check.sh` before submission.

## Commit & Pull Request Guidelines
Recent history uses short, imperative commit subjects such as `format`, `clean unused imports`, and `added some tests`. Keep commit titles brief, lowercase is acceptable, and describe one logical change per commit. Pull requests should summarize behavior changes, mention any user-visible UI impact, and list the verification commands you ran. Include screenshots when changing the `egui` interface.

## Recent Task Notes
- `src/language/load.rs` now loads language resources from YAML files using `serde_yaml`.
- Missing translation keys fall back to `DEFAULT_EN_LANG_MAP`, then to the key string itself.
- A `cargo check` attempt after this change was blocked by an existing dependency issue in `Cargo.toml`: the crate name is written as `egui_notify`, but crates.io expects `egui-notify`.
