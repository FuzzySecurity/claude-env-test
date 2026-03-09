# Connect 4

A beautiful terminal-based Connect 4 game built in Rust.

## Features

- Full-screen terminal UI with colored pieces and Unicode graphics
- Two-player local mode (Red vs Yellow)
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
