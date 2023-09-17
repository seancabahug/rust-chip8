# CHIP-8 emulator in Rust

## Build Requirements

- [Rustup + Cargo](https://www.rust-lang.org/learn/get-started)
- SDL2 ([installation instructions](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc))

## Instructions

- To test the emulator with a pre-chosen ROM, run its respective test script.

  - Windows: `./test_logo.ps1`
  - Linux: `./test_logo`

- To test the emulator with a ROM of your choice, use `cargo run`. Make sure that this command is run at the root of the repository and that the path to your ROM is accurate.

```ps
cargo run -- ./roms/1-chip8-logo.ch8
```

## Milestone Checklist

- [x] `./test_logo` runs and logo is displayed correctly
- [x] All opcode tests pass according to `./test_opcodes`
- [ ] Inputs are functional according to `./test_keypad`
- [ ] SPACE INVADERS WORKS!!

## Post-completion Nice-to-haves

- [ ] Compile to WebAssembly
  - Use the emulator through a website instead of running a binary
