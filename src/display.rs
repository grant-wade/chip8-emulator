
// Modules from crates.io //

/// A struct representing the chip8 display
pub struct ChipDisplay {
    /// A boolean vector representing the display
    display: Vec<bool>,
    /// String to divide display with
    divider: String,
    /// If display has been modified
    modified: bool
}

impl ChipDisplay {
    /// Initialize the chip8 display struct
    pub fn init() -> Self {
        let divider = match String::from_utf8(vec![b'-'; 64]) {
            Ok(s) => s,
            Err(_) => String::from("ERROR")
        };
        ChipDisplay {
            display: vec![false; 2048],
            divider,
            modified: false
        }
    }

    /// Check if the display has been modified
    pub fn mod_check(&mut self) -> bool {
        let ret = match self.modified {
            true => {
                self.modified = false;
                true
            }
            false => false
        };
        ret
    }

    /// Get a copy of the display vector
    pub fn get_display(&self) -> Vec<bool> {
        self.display.clone()
    }


    /// Draw a sprite into the chip8 display buffer, returns true if
    /// a cell has a deletion, false otherwise
    /// 
    /// # Arguments
    /// 
    /// * `x_loc` - x starting position
    /// * `y_loc` - y starting position
    /// * `sprite` - a vector of bytes representing the sprite
    /// 
    pub fn draw_sprite(&mut self, x_loc: u16, y_loc: u16, sprite: Vec<u8>) -> bool {
        let row_count = sprite.len();
        let mut pos;
        // let mut index;
        let mut mask;
        let mut init_val;
        let mut ret = false;
        println!("Sprite: {:#?}", sprite);
        for row in 0..row_count {
            // index = row * 8;
            mask = 0x01;
            for i in 0..8 {
                // Calculate bit position with wrap around
                pos = (((y_loc + row as u16) % 32) * 64) + ((x_loc + i) % 64);
                init_val = self.display[pos as usize];
                match sprite[row] & mask == mask {
                    true => self.display[pos as usize] ^= true,
                    false => self.display[pos as usize] ^= false
                }
                // Check if deletion occured
                if ret == false && init_val == true && self.display[pos as usize] == false {
                    ret = true
                }
                mask <<= 1;
            }
        }
        self.draw_display();
        return ret
    }

    /// Clear the display array
    pub fn clear_display(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                let pos: usize = y * 64 + x;
                self.display[pos] = false;
            }
        }
    }

    /// Draw the chip8 display in the terminal
    pub fn draw_display(&self) {
        println!("|{}|", self.divider);
        for x in 0..32 {
            print!("|");
            for y in 0..64 {
                let pos: usize = x * 64 + y;
                if self.display[pos] == true {
                    print!("#")
                }
                else {
                    print!(" ")
                }
            }
            println!("|");
        }
        println!("|{}|", self.divider);
    }
}
