// Crates used for this project //

// Standard Library Modules //

// Local Modules Use //
use system::ChipSystem;

// Modules From Crates.io //

// Local Modules //
pub mod system;


fn main() {
    let mut sys = ChipSystem::init();
    let res = sys.ram.load_rom_file("roms/Trip8_Demo.ch8");
    match res {
        Ok(_) => println!("Rom file sucessfully read"),
        Err(e) => println!("Could not read rom file: {}", e)
    }
    // sys.ram.dump_ram();
}
