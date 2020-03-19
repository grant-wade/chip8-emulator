use std::io;
use std::fs::File;
use std::io::prelude::*;


/// A representation of chip8 ram
struct ChipMemory {
    /// a vector representing the ram
    ram: Vec<u8>,
    /// program start location
    start: usize
}

impl ChipMemory {
    /// Init a chip8 memory structure 
    fn init() -> Self {
        ChipMemory {
            ram: vec![0; 4096], // Size of chip8 ram
            start: 512
        }
    }

    /// Load a binary into 
    /// 
    /// # Arguments
    /// 
    /// * `rom` - a Vec<u8> holding rom contents
    fn load_bytes(&mut self, rom: Vec<u8>) {
        let len = rom.len();
        for i in 0..len {
            self.ram[i + self.start] = rom[i]
        }
    }

    /// Dump the Chip8 memory into the console as
    /// hex encoded strings. 
    fn dump_ram(&self) {
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
    fn load_rom_file(&mut self, rom_file: &str) -> io::Result<()> {
        // Load bytes from file
        let mut file = File::open(rom_file)?;

        // Create vector to hold rom
        // Capacity of 3584 is max size of Chip8 rom
        let mut rom: Vec<u8> = Vec::with_capacity(3584);

        // Read rom into vector
        file.read_to_end(&mut rom)?;

        // Load bytes into chip8 ram
        self.load_bytes(rom);
        Ok(())
    }
}


/// A struct representing the chip8 registers
struct ChipRegisters {
    /// General purpose registers
    gp_reg: Vec<u8>, 
    /// Address call stack
    stack: Vec<u16>, 
    /// Register I, address storage
    i_reg: u16,      
    /// Delay timer register
    d_reg: u8,       
    /// Sound timer register
    s_reg: u8,       
    /// Program counter, current addr
    pc_reg: u16,     
    /// Stack pointer
    sp_reg: usize,      
}

impl ChipRegisters {
    /// Init a Chip8 register struct
    fn init() -> Self {
        let mut gp_reg = vec![0; 16];
        let mut stack = vec![0; 16];
        ChipRegisters {
            gp_reg,
            stack,
            i_reg: 0,
            d_reg: 0,
            s_reg: 0,
            pc_reg: 0,
            sp_reg: 0,
        }
    }

    /// Set a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which general purpose register
    /// * `value` - value to fill register with
    fn set_gp(&mut self, index: usize, value: u8) {
        self.gp_reg[index] = value;
    }

    /// Get the value of a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which register to get
    fn get_gp(&self, index: usize) -> u8 {
        self.gp_reg[index]
    }

    /// Set the value of the I register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what to put in I register
    fn set_i(&mut self, value: u16) {
        self.i_reg = value;
    }

    /// Get the value of the I register
    fn get_i(&self) -> u16 {
        self.i_reg
    }

    /// Set the value in the pc register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what to put in pc register
    fn set_pc(&mut self, value: u16) {
        self.pc_reg = value;
    }

    /// Get the value of the pc register
    fn get_pc(&self) -> u16 {
        self.pc_reg
    }

    /// Set the value of the delay register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what value to put in the pc register
    fn set_d(&mut self, value: u8) {
        self.d_reg = value;
    }

    /// Set the value of the sound delay register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what value to put in the sound register
    fn set_s(&mut self, value: u8) {
        self.s_reg = value;
    }

    /// Decrement the delay register if value is not 0
    fn decr_d(&mut self) {
        if self.d_reg > 0 {
            self.d_reg -= 1;
        }
    }

    /// Decrement the sound register if vlaue is not 0
    fn decr_s(&mut self) {
        if self.s_reg > 0 {
            self.s_reg -= 1;
        }
    }

    /// Push a address onto the stack, increment stack pointer
    /// 
    /// # Arguments
    /// 
    /// * `addr` - address to push to the stack
    fn push_stack(&mut self, addr: u16) {
        self.stack[self.sp_reg as usize] = addr;
        self.sp_reg += 1;
    }

    /// Pop an address from the stack, decrementing sp
    fn pop_stack(&mut self) -> u16 {
        let addr = self.stack[self.sp_reg];
        self.sp_reg -= 1;
        return addr;
    }
}


/// A struct representing the chip8 display
struct ChipDisplay {
    /// A boolean vector representing the display
    display: Vec<bool>,
    /// String to divide display with
    divider: String
}

impl ChipDisplay {
    /// Initialize the chip8 display struct
    fn init() -> Self {
        let divider = match String::from_utf8(vec![b'-'; 64]) {
            Ok(s) => s,
            Err(_) => String::from("ERROR")
        };

        ChipDisplay {
            display: vec![false; 2048],
            divider
        }
    }

    fn clear_display(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                let pos: usize = y * 64 + x;
                self.display[pos] = false;
            }
        }
    }

    /// Draw the chip8 display in the terminal
    fn draw_display(&self) {
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


struct Opcode {
    h1: u16,
    v1: u16, 
    v2: u16, 
    v3: u16
}


struct ChipSystem {
    registers: ChipRegisters,
    display: ChipDisplay,
    ram: ChipMemory
}

impl ChipSystem {
    fn new(reg: ChipRegisters, disp: ChipDisplay, ram: ChipMemory) -> Self {
        ChipSystem {
            registers: reg,
            display: disp,
            ram: ram
        }
    }

    fn ex_opcode(&mut self, opcode: u16) {
        println!("Current Opcode: {:04x}", opcode);
        let comps = {
            let h1 = (opcode >> 12) & 0xf;
            let v1 = (opcode >> 8) & 0xf;
            let v2 = (opcode >> 4) & 0xf;
            let v3 = (opcode) & 0xf;
            Opcode {h1, v1, v2, v3}
        };

        match comps.h1 {
            0 => {
                match comps.v3 {
                    0 => self.display.clear_display(),
                    14 => {
                        let pc: u16 = self.registers.pop_stack();
                        self.registers.set_pc(pc);
                    },
                    _ => panic!("Invalid Opcode: {:04x}", opcode)
                }
            },
            1 => {
                let pc: u16 = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_pc(pc);
            },
            2 => {},
            3 => {},
            4 => {},
            5 => {},
            6 => {},
            7 => {},
            8 => {},
            9 => {},
            10 => {},
            11 => {},
            12 => {},
            13 => {},
            14 => {},
            15 => {}
            _ => panic!("Invalid Opcode header: {:02x}", comps.h1)
        }
    }
}


fn init_chip8() -> ChipSystem {
    let ram = ChipMemory::init();
    let disp = ChipDisplay::init();
    let reg = ChipRegisters::init();
    ChipSystem::new(reg, disp, ram)
}



fn main() {
    let mut sys = init_chip8();
    let res = sys.ram.load_rom_file("roms/Trip8_Demo.ch8");
    match res {
        Ok(_) => println!("Rom file sucessfully read"),
        Err(e) => println!("Could not read rom file: {}", e)
    }
    // sys.ram.dump_ram();

    sys.display.draw_display();
}
