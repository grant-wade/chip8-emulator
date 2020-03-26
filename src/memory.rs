
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
        // let ram: Vec<u8> = vec![0; 4096];
        let ram = ChipMemory::load_symbols(vec![0; 4096]);
        ChipMemory {
            ram, // Size of chip8 ram
            loaded: false,
            start: 512
        }
    }

    fn load_symbols(mut ram: Vec<u8>) -> Vec<u8> {
        let hex_chars = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for i in 0..hex_chars.len() {
            ram[i + 0x050] = hex_chars[i];
        }
        ram
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
            self.ram[i + self.start] = rom[i]; 
            
            // { // Flip bit order
            //     let mut switched: u8 = 0;
            //     let mut mask: u8 = 0x80;
            //     let mut mask_inv: u8 = 0x01;
            //     for _ in 0..8 {
            //         switched += {
            //             match mask & rom[i] == mask {
            //                 true => mask_inv,
            //                 false => 0
            //             }
            //         };
            //         mask >>= 1;
            //         mask_inv <<= 1;
            //     }
            //     switched
            // }
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
        println!("");
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