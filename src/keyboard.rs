
// Standard Library Modules //
use std::time::Duration;
use std::thread;

/// Struct representing a keyboard that uses hex values (0-9, A-F)
/// this is represented by a boolean vector, true for pressed
pub struct ChipKeyboard {
    keys: Vec<bool>
}

impl ChipKeyboard {
    /// Initialize the Chip8 keyboard
    pub fn init() -> Self {
        // create the vector of keys
        let keys = vec![false; 15];
        ChipKeyboard {
            keys
        }
    }

    /// Returns true if any key is currently pressed (true)
    fn any_pressed(&self) -> bool {
        for i in 0..15 {
            if self.keys[i] == true {
                return true;
            }
        }
        return false;
    }

    /// Checks which key is pressed, returning its index
    fn which_pressed(&self) -> u8 {
        for i in 0..15 {
            if self.keys[i] == true {
                return i as u8;
            }
        }
        return 16;
    }

    /// Set a key to pressed (true) or not pressed (false)
    /// 
    /// # Arguments
    /// 
    /// * `index` - index of the key (0-15)
    /// * `value` - true or false
    pub fn set_key(&mut self, index: u8, value: bool) {
        self.keys[index as usize] = value;
    }

    /// Get the state of a certain key (0-15)
    pub fn get_key(&self, index: u8) -> bool{
        self.keys[index as usize]
    }

    /// Wait for a keypress to happen and return which key
    pub fn wait_key(&self) -> u8 {
        while !self.any_pressed() {
            thread::sleep(Duration::from_millis(100));
        }
        return self.which_pressed();
    }
}