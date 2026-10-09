# AGENTS.md

Rust terminal (TUI) Space Invaders game. Binary-only crate (`src/main.rs`, no `lib.rs`), edition 2024.

## Commands

- Run: `cargo run` — needs a real TTY. Do not run it headless/piped; it enters raw mode + alt screen.
- Test: `cargo nextest run` (or `cargo test`). Single test: `cargo nextest run <name-substring>`.
- Lint: `cargo clippy --locked --all-targets --all-features -- -D warnings`
- Format: `cargo fmt` (CI enforces `cargo fmt --all --check`).
- Dependencies/licences: `cargo deny check`
- Coverage: `cargo llvm-cov --locked --all-features` (needs the `llvm-tools-preview` component).

CI (`.github/workflows/ci.yml`) runs fmt, clippy, nextest, deny, and coverage, with `RUSTFLAGS=-D warnings`. Lints are strict — clippy `all` + `pedantic` — so any new compiler or clippy warning fails the build. The only allowed clippy lints are the two float→cell casts in `Cargo.toml`.

## Architecture

- `src/main.rs` — wiring + `render()`; the loop is fixed-step, paced by `config.fps`.
- `src/config` — all tunables live in `GameConfig`.
- `src/game` — simulation: `GameState` (`state.rs`), plus `Player`, `Alien`, `Bullet`.
- `src/input` — `Input` / `EventPoller` traits, `TerminalInputHandler` + `CrosstermEventPoller`; tests inject `MockEventPoller`.
- `src/renderer` — `Renderer` owns the terminal lifecycle (raw mode, alt screen); `frame.rs` is a pure, width-aware frame buffer.

## Gotchas

- Rendering is full-frame double-buffered: write through `Renderer::draw_char` / `draw_str`, then call `present()` once per tick. Never write cells directly to stdout — that caused the earlier glyph-smearing regression. `👾` is double-width and is tracked as `Glyph` + `Continuation` in the `Frame`.
- `Renderer::Drop` restores the terminal (disables raw mode, leaves alt screen). Do not add terminal cleanup anywhere else.
- MVP scope: Aliens are static and never fire, so there is no lose condition. Deferred features are intentionally kept behind `#[allow(dead_code)]` (`Alien::update`/`reverse_direction`, `Difficulty`, `GameStatus`, `alien_speed`, `initial_lives`) — do not delete them as dead code.
- Binary crate: `pub` does not exempt items from `dead_code`, and unit tests are inline `#[cfg(test)]` modules (there is no `tests/` dir).
- New dependencies must have a licence in the `deny.toml` allow list (`MIT`, `Apache-2.0`, `Apache-2.0 WITH LLVM-exception`) or `cargo deny check` fails. Our crate is `publish = false`, so it is exempt from the licence check.

## Domain vocabulary

Use the terms defined in `GLOSSARY.md`: **Player**, **Alien**, **Wave**, **Playfield** — not character/enemy/ship/screen.
