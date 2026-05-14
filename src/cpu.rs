pub struct CPU {
    ir: u16,  // instruction register (actual instruction data)
    mdr: u16, // memory data    register (data  to read/write)
    mar: u16, // memory address register (where to read/write)
    mcr: u16, // machine control register
    pc: u16,  // program counter (curr instr +1)
    acv: bool,
    memory: [u16; 0xFFFF],
    registers: [u16; 8],
    psr: u16,
    interrupt: bool,
    ben: bool,
    nzp: u8, // 00000nzp
}

#[inline(always)]
fn sign_extend(val: i16, bits: usize) -> i16 {
    (val << (16 - bits)) >> (16 - bits)
}

impl CPU {
    fn fetch(&mut self) {
        self.mar = self.pc;
        self.pc += 1;
        self.acv = self.mar < 0x3000 || self.mar >= 0xf300 && (self.psr & 0x4000) > 0;

        if self.interrupt {
            // interrupts are not a thing rn
        }

        if self.acv {
            // also not a thing
        }

        self.mdr = self.memory[self.mar as usize];
        self.ir = self.mdr;

        self.set_ben();
    }

    fn set_ben(&mut self) {
        let n = self.nzp >> 2 & 1;
        let z = self.nzp >> 1 & 1;
        let p = self.nzp & 1;

        let ir_11 = ((self.ir & 0x400) >> 10) as u8;
        let ir_10 = ((self.ir & 0x200) >> 9) as u8;
        let ir_9 = ((self.ir & 0x100) >> 8) as u8;

        self.ben = (ir_11 & n | ir_10 & z | ir_9 & p) > 0;
    }

    fn decode(&mut self) {
        let opcode: u8 = ((self.mdr & 0xF000) >> 12) as u8;
        match opcode {
            0b0001 => { // ADD
            }
            0b0101 => { // AND
            }
            0b0000 => { // BR
            }
            0b1100 => { // JMP / RET
            }
            0b0100 => { // JSR/JSRR
            }
            0b0010 => { // LD
            }
            0b1010 => { // LDI
            }
            0b0110 => { // LDR
            }
            0b1110 => { // LEA
            }
            0b1001 => { // NOT
            }
            0b0011 => { // ST
            }
            0b1011 => { // STI
            }
            0b0111 => { // STR
            }
            0b1111 => { // TRAP
            }
            _ => {}
        }
    }

    fn evaluate_address(val: u16, imm_size: usize) -> i16 {
        let imm_mask = (1 << imm_size) - 1u16;
        let pc_offset = (val & imm_mask) as i16;

        let ext_offset = sign_extend(pc_offset, imm_size);

        todo!()
    }

    pub fn run(&mut self) {
        while (self.mcr & 0x8000) != 0 {
            self.step();
        }
    }

    pub fn step(&mut self) {
        // program step
        self.fetch();
        self.decode();
    }

    pub fn view_memory_slice(&self, start: usize, end: usize) -> &[u16] {
        // look at memory from start to end location
        &self.memory[start..end]
    }

    pub fn view_all_memory(&self) -> &[u16] {
        &self.memory
    }

    pub fn view_registers(&self) -> &[u16; 8] {
        // view all 8 registers
        &self.registers
    }

    pub fn view_pc(&self) -> u16 {
        // get current pc value
        self.pc
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            ir: 0,
            mdr: 0,
            mar: 0,
            mcr: 0x8000,
            pc: 0x3000,
            acv: false,
            memory: [0u16; 0xFFFF],
            registers: [0u16; 8],
            psr: 0,
            interrupt: false,
            ben: false,
            nzp: 0,
        }
    }
}
