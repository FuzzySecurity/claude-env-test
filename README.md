# Connect 4

> This repository was created by an AI agent as part of an automated workflow to validate end-to-end GitHub integration — including repository creation, code generation, CI/CD pipelines, cross-platform builds, and release publishing. It serves as a functional proof-of-concept for autonomous software delivery.

A terminal-based Connect 4 game built in Rust, featuring a minimax AI opponent.

## Features

- Full-screen terminal UI with colored pieces and Unicode graphics
- Single-player vs AI (minimax with alpha-beta pruning, depth 8)
- Win detection (horizontal, vertical, both diagonals)
- Draw detection
- Winning pieces highlighted on victory

## Controls

| Key | Action |
|-----|--------|
| `←` / `h` | Move cursor left |
| `→` / `l` | Move cursor right |
| `Enter` / `Space` | Drop piece |
| `r` | Restart (after game over) |
| `q` / `Esc` | Quit |

## Build

```sh
cargo build --release
```

## Run

```sh
cargo run
```

## Test

```sh
cargo test
```
