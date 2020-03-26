# Chip8 Emulation Library
Created By Grant Wade <grant.wade@meridiansky.co>

## Emulator Information

This emulator implments all 35 original Chip8 opcodes and supports a 64x32 pixel display. It does not currently support the extended Super-Chip (SCHIP|CHIP-48) instruction set or different display sizes, this may be added in the future. 

Since the implementation was done as a library it is not a full solution to run and interact with a Chip8 program. While this may implement all needed features to run the program it requires another program to display the screen and let the emulator know when a key is pressed. There will be another project that runs the emulator in WASM and uses a browser to interact and display.

The actual implementation was done in [Rust](https://www.rust-lang.org/) with minimal dependencies as a project to learn more about the language and creating an emulator. This is still a work in progress and any suggestions are welcome. For this reason there is probably no reason to submit this as a [crate](https://crates.io/).



## Chip8 Information and Resources

Chip8 is an interpreted programming language developed in the 1970's to allow video games to move easily be made for computers like the [COSMAC VIP](https://en.wikipedia.org/wiki/COSMAC_VIP) and [Telmac 1800](https://en.wikipedia.org/wiki/Telmac_1800). Classic Chip8 programs include Pong, Space Invaders, Tetris, and Pac-Man. There are a number of public domain ROMs avaliable on [GitHub](https://github.com/dmatlack/chip8/tree/master/roms). These were used for testing the emulator implementation. More information can be found on the [Chip8 Wikipedia page](https://en.wikipedia.org/wiki/CHIP-8).

## Documentation

Documentation can be generated with `rustdoc` run `cargo doc --open` to have the Rust docs for this project open in a web browser.

## License

This is licensed under an MIT so feel free to use in any way you see fit.