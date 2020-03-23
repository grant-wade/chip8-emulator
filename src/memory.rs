
// Standard Library Modules //
use std::io;
use std::fs::File;
use std::io::prelude::*;

/// A representation of chip8 ram
pub struct ChipMemory {
    /// a vector representing the ram
    ram: Vec<u8>,
    /// true if a rom has been loaded
    loaded: bool, 
    /// program start location
    start: usize
}

impl ChipMemory {
    /// Init a chip8 memory structure 
    pub fn init() -> Self {
        ChipMemory {
            ram: vec![0; 4096], // Size of chip8 ram
            loaded: false,
            start: 512
        }
    }

    /// Returns true if a ROM has been loaded, false otherwise
    pub fn has_loaded(&self) -> bool {
        self.loaded
    }

    /// Return a two byte opcode
    /// 
    /// # Arguments
    /// 
    /// * `index` - index where opcode starts
    pub fn get_opcode(&self, index: u16) -> u16 {
        ((self.get_byte(index) as u16) << 8) | self.get_byte(index + 1) as u16
    }

    /// Load a binary into 
    /// 
    /// # Arguments
    /// 
    /// * `rom` - a Vec<u8> holding rom contents
    pub fn load_bytes(&mut self, rom: Vec<u8>) {
        let len = rom.len();
        for i in 0..len {
            self.ram[i + self.start] = rom[i]
        }
    }

    /// Set a byte in ram to a passed value
    /// 
    /// # Arguments
    /// 
    /// * `loc` - location to set
    /// * `val` - value to set with
    pub fn set_byte(&mut self, loc: u16, val: u8) {
        self.ram[loc as usize] = val;
    }

    /// Get a byte at `loc`
    /// 
    /// # Arguments
    /// 
    /// * `loc` - location of byte
    pub fn get_byte(&self, loc: u16) -> u8 {
        self.ram[loc as usize]
    }

    /// Get a range of bytes
    /// 
    /// # Arguments
    /// 
    /// * `loc` - start location of bytes
    /// * `nbytes` - how many bytes
    pub fn get_nbytes(&self, loc: u16, nbytes: u16) -> Vec<u8> {
        let mut out_bytes: Vec<u8> = vec![0; nbytes as usize];
        for i in 0..nbytes as usize {
            out_bytes[i] = self.get_byte(loc + i as u16);
        }
        out_bytes
    }

    /// Dump the Chip8 memory into the console as
    /// hex encoded strings. 
    pub fn dump_ram(&self) {
        let len = self.ram.len();
        for i in 0..len {
            if i % 2 == 0 {
                print!(" ");
            }
            if i % 32 == 0 {
                println!("");
            }
            print!("{:02x}", self.ram[i]);
        }
    }

    /// Load a file from disk and write its bytes into 
    /// the Chip8 memory. 
    /// 
    /// # Arguments
    /// 
    /// * `rom_file` - the filename to open and read from
    pub fn load_rom_file(&mut self, rom_file: &str) -> io::Result<()> {
        // Load bytes from file
        let mut file = File::open(rom_file)?;

        // Create vector to hold rom
        // Capacity of 3584 is max size of Chip8 rom
        let mut rom: Vec<u8> = Vec::with_capacity(3584);

        // Read rom into vector
        file.read_to_end(&mut rom)?;

        // Load bytes into chip8 ram
        self.load_bytes(rom);
        self.loaded = true;
        Ok(())
    }
}