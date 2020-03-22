
/// A struct representing the chip8 registers
pub struct ChipRegisters {
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
    pub fn init() -> Self {
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
    pub fn set_gp(&mut self, index: usize, value: u8) {
        self.gp_reg[index] = value;
    }

    /// Get the value of a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which register to get
    pub fn get_gp(&self, index: usize) -> u8 {
        self.gp_reg[index]
    }

    /// Add a value to a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which general purpose register
    /// * `value` - u8 value to add to register
    pub fn add_gp(&mut self, index: usize, value: u8) {
        self.gp_reg[index] += value;
    }

    /// Set the value of the I register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what to put in I register
    pub fn set_i(&mut self, value: u16) {
        self.i_reg = value;
    }

    /// Get the value of the I register
    pub fn get_i(&self) -> u16 {
        self.i_reg
    }

    /// Set the value in the pc register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what to put in pc register
    pub fn set_pc(&mut self, value: u16) {
        self.pc_reg = value;
    }

    /// Increment the value of the pc register by 2
    pub fn incr_pc(&mut self) {
        self.pc_reg += 2;
    }

    /// Get the value of the pc register
    pub fn get_pc(&self) -> u16 {
        self.pc_reg
    }

    /// Get the value of the delay register
    pub fn get_d(&self) -> u8 {
        self.d_reg
    }

    /// Set the value of the delay register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what value to put in the pc register
    pub fn set_d(&mut self, value: u8) {
        self.d_reg = value;
    }

    /// Get the value of the sound delay register
    pub fn get_s(&self) -> u8 {
        self.s_reg
    }

    /// Set the value of the sound delay register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what value to put in the sound register
    pub fn set_s(&mut self, value: u8) {
        self.s_reg = value;
    }

    /// Decrement the delay register if value is not 0
    pub fn decr_d(&mut self) {
        if self.d_reg > 0 {
            self.d_reg -= 1;
        }
    }

    /// Decrement the sound register if vlaue is not 0
    pub fn decr_s(&mut self) {
        if self.s_reg > 0 {
            self.s_reg -= 1;
        }
    }

    /// Push a address onto the stack, increment stack pointer
    /// 
    /// # Arguments
    /// 
    /// * `addr` - address to push to the stack
    pub fn push_stack(&mut self, addr: u16) {
        self.stack[self.sp_reg as usize] = addr;
        self.sp_reg += 1;
    }

    /// Pop an address from the stack, decrementing sp
    pub fn pop_stack(&mut self) -> u16 {
        self.sp_reg -= 1;
        let addr = self.stack[self.sp_reg];
        return addr;
    }
}
