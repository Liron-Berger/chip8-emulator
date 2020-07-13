# Chip8 Emulator

This is my attempt at building an emulator for chip8 as a learning project for Rust.

As the project mainly helped practice the language, I'm still unsure whether I'm going to finish it. 

The project is written in Rust and uses Sdl as a GUI (for cross-platform support)

![mazze](docs/maze.png)

## Requirements
1. libsdl
2. Rust

The project was only tested on Linux Mint but should work properly on all platforms.

## How to run
```bash
cargo run <path_to_rom>
```


## Known Bugs

- [ ] In pong - the points of the first player are not updating.

## Possible future features
- [ ] Sound Driver
- [ ] Changing the GUI to WebAssembly using Yew

## Resources

These are the resources used in order to develop the project:
- https://doc.rust-lang.org/book/ - for learning Rust on the way
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM - great technical reference for chip8
- https://github.com/dmatlack/chip8/tree/master/roms/games - Lots of useful Roms to test the emulator with
