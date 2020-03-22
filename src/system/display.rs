
/// A struct representing the chip8 display
pub struct ChipDisplay {
    /// A boolean vector representing the display
    display: Vec<bool>,
    /// String to divide display with
    divider: String,
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
        }
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
