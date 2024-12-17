# About

Example codebase demoing implementing a terminal emulator in Rust.
This implementation is notably quite inefficient!!!

![alt text](docs/image.png)

# Status
Screen content toggles when you press space.

# MacOS
```bash
brew install sdl2 sld2_ttf
cargo run
```

# TODO

 - [x] Matrix to SDL display
 - [ ] Address warnings?
 - [ ] Handle control sequences.
 - [ ] Make into library?
 - [ ] Handle escape sequence properly...
 - [ ] Remove VTerm dependency once I have successfully
       demonstrated basic ATerm in SDL2.