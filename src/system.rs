
// Standard Library Modules //
use std::error;
use std::fmt;
use std::thread;
use std::time::Duration;

// Local Modules Use //
use memory::ChipMemory;
use registers::ChipRegisters;
use display::ChipDisplay;
use keyboard::ChipKeyboard;

// Modules From Crates.io //
use rand::Rng;

// Local Modules //
pub mod memory;
pub mod registers;
pub mod display;
pub mod keyboard;


// Define a opcode execution error type //
pub type ExResult<T> = std::result::Result<T, ExError>;

#[derive(Debug, Clone)]
pub struct ExError {
    opcode: u16
}

impl fmt::Display for ExError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid opcode previded for execution: {:04x}", self.opcode)
    }
}

impl error::Error for ExError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

/// Representation of a 2 byte chip8 opcode
struct Opcode {
    h1: u16,
    v1: u16, 
    v2: u16, 
    v3: u16
}

impl Opcode {
    /// create a new opcode struct from a 2 byte opcode
    fn new(opcode: u16) -> Self {
        let h1 = (opcode >> 12) & 0xf;
        let v1 = (opcode >> 8) & 0xf;
        let v2 = (opcode >> 4) & 0xf;
        let v3 = (opcode) & 0xf;
        Opcode {h1, v1, v2, v3}
    }
}


/// A representation of the Chip8 Architecture
pub struct ChipSystem {
    /// Registers and related methods
    pub registers: ChipRegisters,
    /// Display and related methods
    pub display: ChipDisplay,
    /// RAM and related functions
    pub ram: ChipMemory,
    /// Keyboard and related functions
    pub keyboard: ChipKeyboard,
}

impl ChipSystem {
    /// Initialize the Chip8 System
    pub fn init() -> Self {
        let ram = ChipMemory::init();
        let disp = ChipDisplay::init();
        let reg = ChipRegisters::init();
        let key = ChipKeyboard::init();
        ChipSystem {
            registers: reg,
            display: disp,
            ram: ram,
            keyboard: key
        }
    }

    /// Return a random u8
    fn random_byte() -> u8 {
        let mut rng = rand::thread_rng();
        let rand_byte: u8 = rng.gen();
        return rand_byte;
    }

    /// Execute a Chip8 Opcode
    /// 
    /// This function can deal with the original 35 Chip8 opcodes
    /// 
    /// # Arguments
    /// 
    /// * `opcode` - Two byte opcode to execute
    /// 
    /// ## Opcodes
    /// 0. 0x0nnn - Unused instruction from actual hardware, ignored
    /// 1. 0x00E0 - Clear Display
    /// 2. 0x00EE - Return from subroutine
    /// 3. 0x1nnn - Jump to address `nnn`
    /// 4. 0x2nnn - Call subroutine at `nnn` store call address on stack
    /// 5. 0x3xkk - Skip next instruction when `Vx == kk`
    /// 6. 0x4xkk - Skip next instruction when `Vx != kk`
    /// 7. 0x5xy0 - Skip next instruction when `Vx == Vy`
    /// 8. 0x6xkk - Put the byte value in passed register `Vx = kk` 
    /// 9. 0x7xkk - Add byte to register `Vx = Vx + kk` no borrow check
    /// 10. 0x8xy0 - Store value of register `Vy` in `Vx` `Vx = Vy`
    /// 11. 0x8xy1 - Logical OR the registers `Vx = Vx | Vy`
    /// 12. 0x8xy2 - Logical AND the registers `Vx = Vx & Vy`
    /// 13. 0x8xy3 - Logical XOR the registers `Vx = Vx ^ Vy`
    /// 14. 0x8xy4 - Add the registers with overflow check `Vx = Vx + Vy`
    /// 15. 0x8xy5 - Subtract the registers `Vx = Vx - Vy`, on borrow `Vf = 0 else 1`
    /// 16. 0x8xy6 - Logical right shift, LSB to `Vf`, `Vx = Vx >> 1`
    /// 17. 0x8xy7 - Subtract the registers `Vx = Vy - Vx`, on borrow `Vf = 0 else 1`
    /// 18. 0x8xyE - Logical left shift, MSB to `Vf`, `Vx = Vx << 1`
    /// 19. 0x9xy0 - SKip next instruction when `Vx != Vy`
    /// 20. 0xAnnn - Set register `I` to `nnn`, `I = nnn`
    /// 21. 0xBnnn - Jump to location `PC = nnn + V0`
    /// 22. 0xCxkk - Set register to random value `Vx = rand<u8> & kk`
    /// 23. 0xDxyn - Draw a sprite on the screen at xy of height n
    /// 24. 0xEx9E - Skip next instruction if key with value `Vx` is pressed
    /// 25. 0xExA1 - Skip next instruction if key with value `Vx` is not pressed
    /// 26. 0xFx07 - Set the value in `Vx` to the delay timer
    /// 27. 0xFx0A - Wait for keypress, store value in `Vx`
    /// 28. 0xFx15 - Set the delay timer to value in `Vx`
    /// 29. 0xFx18 - Set the sound timer to value in `Vx`
    /// 30. 0xFx1E - Set value of `I` to `I = I + Vx`
    /// 31. 0xFx29 - Set I to location of sprite location in `Vx`
    /// 32. 0xFx33 - Store BCD of `Vx` in `I, I+1, I+2`
    /// 33. 0xFx55 - Store `V0 -> Vx` at I
    /// 34. 0xFx65 - Retrieve `V0 -> Vx` from I
    ///  
    pub fn ex_opcode(&mut self, opcode: u16) -> ExResult<()> {
        let comps = Opcode::new(opcode);
        let mut update_pc = true;
        match comps.h1 {
            0x0 => {
                match comps.v3 {
                    // CLS - Clear Display
                    0 => self.display.clear_display(),
                    // RET - Return from subroutine
                    14 => {
                        let pc: u16 = self.registers.pop_stack();
                        self.registers.set_pc(pc);
                        // update_pc = false;
                    },
                    // Skip Opcode
                    _ => {},
                }
            },
            // JP - Jumps to address without modifying stack
            0x1 => {
                let pc: u16 = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_pc(pc);
                update_pc = false;
            },
            // CALL - Jump to address with push to stack
            0x2 => {
                let new_pc: u16 = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                let cur_pc = self.registers.get_pc();
                self.registers.push_stack(cur_pc);
                self.registers.set_pc(new_pc);
                update_pc = false;
            },
            // SE Vx, Byte - Skip instruction if Vx == Byte
            0x3 => {
                let comp_val: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                let reg_val: u8 = self.registers.get_gp(comps.v1 as usize);
                if comp_val == reg_val {
                    self.registers.incr_pc();
                }
            },
            // SNE Vx, Byte - Skip instruction if Vx != Byte
            0x4 => {
                let comp_val: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                let reg_val: u8 = self.registers.get_gp(comps.v1 as usize);
                if comp_val != reg_val {
                    self.registers.incr_pc();
                }
            },
            // SE Vx, Vy - Skip instruction if Vx == Vy
            0x5 => {
                let reg_x_val: u8 = self.registers.get_gp(comps.v1 as usize);
                let reg_y_val: u8 = self.registers.get_gp(comps.v2 as usize);
                if reg_x_val == reg_y_val {
                    self.registers.incr_pc();
                }
            },
            // LD Vx, Byte - Load byte value into Vx (Vx = Byte)
            0x6 => {
                let value: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                self.registers.set_gp(comps.v1 as usize, value);
            },
            // ADD Vx, Byte - Add byte value to Vx (Vx += Byte) no carry flag
            0x7 => {
                let value: u16 = ((comps.v2) << 4) + comps.v3;
                self.registers.add_gp(comps.v1 as usize, (value & 0xff) as u8);
            },
            0x8 => {
                match comps.v3 {
                    // LD Vx, Vy - Store value of Vy in Vx (Vx = Vy)
                    0x0 => {
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        self.registers.set_gp(comps.v1 as usize, reg_y_val);
                    },
                    // OR Vx, Vy - Bitwise OR on Vx, Vy store in Vx (Vx = Vx | Vy)
                    0x1 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let value = reg_x_val | reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, value);
                    },
                    // AND Vx, Vy - Bitwise AND on Vx, Vy store in Vx (Vx = Vx & Vy)
                    0x2 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let value = reg_x_val & reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, value);
                    },
                    // XOR Vx, Vy - Bitwise XOR on Vx, Vy store in Vx (Vx = Vx ^ Vy)
                    0x3 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let value = reg_x_val ^ reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, value);
                    },
                    // ADD Vx, Vy - Add Vx, Vy if > 255 set Vf to 1 (Vx = Vx + Vy)
                    0x4 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let holder: u16 = reg_x_val as u16 + reg_y_val as u16;
                        match holder > 255 {
                            true => self.registers.set_gp(15, 1),
                            false => self.registers.set_gp(15, 0)
                        }
                        self.registers.set_gp(comps.v1 as usize, (holder & 0xff) as u8);
                    },
                    // SUB Vx, Vy - Subtract Vx, Vy if Vx < Vy set Vf to 0 (Vx = Vx - Vy)
                    0x5 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        match reg_x_val < reg_y_val {
                            true => self.registers.set_gp(15, 0),
                            false => self.registers.set_gp(15, 1)
                        }
                        let holder = reg_x_val - reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, holder);
                    },
                    // SHR Vx, _ - Shift Vx right by 1, set Vf to LSB (Vx = Vx >> 1)
                    0x6 => {
                        let mut reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        self.registers.set_gp(15, reg_x_val & 0x01);
                        reg_x_val = reg_x_val >> 1; 
                        self.registers.set_gp(comps.v1 as usize, reg_x_val);
                    },
                    // SUBN Vx, Vy - Subtract Vy, Vx if Vy < Vx set Vf to 0 (Vx = Vy - Vx)
                    0x7 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        match reg_y_val < reg_x_val {
                            true => self.registers.set_gp(15, 0),
                            false => self.registers.set_gp(15, 1)
                        }
                        let holder = reg_y_val - reg_x_val;
                        self.registers.set_gp(comps.v1 as usize, holder);
                    },
                    // SHL Vx, _ - Shift Vx left by 1, set Vf to MSB (Vx = Vx << 1)
                    0xE => {
                        let mut reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        self.registers.set_gp(15, reg_x_val & 0x80);
                        reg_x_val = reg_x_val << 1;
                        self.registers.set_gp(comps.v1 as usize, reg_x_val);
                    },
                    _ => return Err(ExError {opcode})
                }
            },
            // SNE Vx, Vy - Skip next instruction if Vx != Vy
            0x9 => {
                let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                if reg_x_val != reg_y_val {
                    self.registers.incr_pc();
                }
            },
            // LD I, Addr (12bit) - Register I is set to the address
            0xA => {
                let value = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_i(value);
            },
            // JP V0, Addr (12bit) - Jump to the location Addr + V0
            0xB => {
                let reg_v0_val = self.registers.get_gp(0);
                let address = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_pc(address + reg_v0_val as u16);
                update_pc = false;
            },
            // RND Vx, Byte - Set Vx to Byte & Random byte
            0xC => {
                let byte_val: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                let value = byte_val & ChipSystem::random_byte(); 
                self.registers.set_gp(comps.v1 as usize, value);
            },
            // DRW Vx, Vy, N - Draw a sprite coord (Vx, Vy) with height N
            0xD => {
                let x_loc = self.registers.get_gp(comps.v1 as usize) as u16;
                let y_loc = self.registers.get_gp(comps.v2 as usize) as u16;
                // let y_loc = comps.v2;
                let nbytes = comps.v3;
                let sprite_mem_loc = self.registers.get_i();
                let sprite_bytes = self.ram.get_nbytes(sprite_mem_loc, nbytes);
                let overlap = self.display.draw_sprite(x_loc, y_loc, sprite_bytes);
                match overlap {
                    true => self.registers.set_gp(15, 1),
                    false => self.registers.set_gp(15, 0),
                }
            },
            0xE => {
                match (comps.v2 << 4) + comps.v3 {
                    // SKP Vx - Skip next instruction if key (0-15) is pressed
                    0x9E => {
                        let index = comps.v1 as u8;
                        let key_val = self.keyboard.get_key(index);
                        if key_val {
                            self.registers.incr_pc();
                        }
                    },
                    // SKNP Vx - Skip next instruction if key (0-15) is not pressed
                    0xA1 => {
                        let index = comps.v1 as u8;
                        let key_val = self.keyboard.get_key(index);
                        if !key_val {
                            self.registers.incr_pc();
                        }
                    }
                    _ => return Err(ExError {opcode})
                }
            },
            0xF => {
                match (comps.v2 << 4) + comps.v3 {
                    // LD Vx, DT - Set Vx to the value of the delay timer
                    0x07 => {
                        let delay_val = self.registers.get_d();
                        self.registers.set_gp(comps.v1 as usize, delay_val);
                    },
                    // LD Vx, K - Wait for keypress (halt), put key value in Vx
                    0x0A => {
                        let key = self.keyboard.wait_key();
                        let index = comps.v1 as usize;
                        self.registers.set_gp(index, key); 
                    },
                    // LD DT, Vx - Set the delay timer to the value in Vx
                    0x15 => {
                        let delay_val = comps.v1 as u8;
                        self.registers.set_d(delay_val);
                    },
                    // LD ST, Vx - Set the sound timer to the value in Vx
                    0x18 => {
                        let delay_val = comps.v1 as u8;
                        self.registers.set_s(delay_val);
                    },
                    // ADD I, Vx - Set register I to I + Vx
                    0x1E => {
                        let i_val = self.registers.get_i();
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let value = i_val + reg_x_val as u16;
                        self.registers.set_i(value);
                    },
                    // LD F, Vx - Set I to the location of sprite (I = Vx * 5)
                    0x29 => {
                        let reg_x_val = comps.v1;
                        let new_i_val = reg_x_val * 5;
                        self.registers.set_i(new_i_val);
                    },
                    // LD B, Vx - Place the BCD of Vx in I (Hundreds), I+1 (Tens), I+2 (Ones)
                    0x33 => {
                        let reg_val = self.registers.get_gp(comps.v1 as usize);
                        let i_val = self.registers.get_i();
                        let ones = reg_val % 10;
                        let tens = (reg_val / 10) % 10;
                        let huns = (reg_val / 100) % 10;
                        self.ram.set_byte(i_val, huns);
                        self.ram.set_byte(i_val + 1, tens);
                        self.ram.set_byte(i_val + 2, ones);
                    },
                    // LD I, Vx - Stores V0 to Vx in memory starting at address I, then (I = I + x + 1)
                    0x55 => {
                        let i_val = self.registers.get_i();
                        let x_range = comps.v1;
                        let mut cur_reg: u8;
                        for loc in 0..x_range {
                            cur_reg = self.registers.get_gp(loc as usize);
                            self.ram.set_byte(i_val + loc, cur_reg);
                        }
                        let new_i = i_val + x_range + 1;
                        self.registers.set_i(new_i);
                    },
                    // LD Vx, I - Fills V0 to Vx with values from memory starting at address then (I = I + x + 1)
                    0x65 => {
                        let i_val = self.registers.get_i();
                        let x_range = comps.v1;
                        let mut cur_reg: u8;
                        for loc in 0..x_range {
                            cur_reg = self.ram.get_byte(i_val + loc);
                            self.registers.set_gp(loc as usize, cur_reg);
                        }
                    },
                    _ => return Err(ExError {opcode})
                }
            }
            _ => return Err(ExError {opcode})
        }
        // Increment program counter after opcode execution
        if update_pc {
            self.registers.incr_pc()
        }
        return Ok(());
    }

    fn get_next_opcode(&self) -> u16 {
        let mut index = self.registers.get_pc();
        if index % 2 != 0 {
            index -= 1;
            println!("Program Counter is not even: {}", index);
            // panic!("Program Counter register invalid")
        }
        self.ram.get_opcode(index)
    }

    /// Run the chip8 emulator in an infinite loop
    pub fn run(&mut self) {
        if !self.ram.has_loaded() {
            println!("No ROM has been loaded.");
            return 
        }

        let mut opcode: u16;
        let mut res: ExResult<()>;
        loop {
            // Get current opcode and execute
            opcode = self.get_next_opcode();
            res = self.ex_opcode(opcode);
            match res {
                Ok(_) => self.registers.incr_pc(),
                Err(e) => {
                    println!("Execution halted; error occured");
                    println!("Error: {:#?}", e);
                    break;
                }
            }
            self.registers.decr_d();
            self.registers.decr_s();
            self.registers.dump_registers();
            thread::sleep(Duration::from_millis(16))
        }
        println!("Program Stopped");
    }

    /// Run an emulaton step, this executes a single opcode
    /// from the chip8 memory system, pointed to by the PC reg
    /// 
    /// Returns a representation of the screen if it has been modified
    pub fn step(&mut self, display_opcode:  bool) -> (u16, Option<Vec<bool>>) {
        let opcode = self.get_next_opcode();
        if display_opcode {
            println!("Opcode: {:04x}", opcode);
        }
        let res: ExResult<()> = self.ex_opcode(opcode);
        match res {
            Ok(_) => {},
            Err(e) => {
                println!("Execution halted; error occured");
                println!("Error: {:#?}", e);
            }
        }
        self.registers.decr_d();
        self.registers.decr_s();
        match self.display.mod_check() {
            true => return (opcode, Some(self.display.get_display())),
            false => return (opcode, None)
        }
    }

    /// Load a ROM into the chip8 memory
    /// 
    /// # Arguments
    /// 
    /// * `rom` - a u8 vector representing the rom
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.ram.load_bytes(rom);
    }
}