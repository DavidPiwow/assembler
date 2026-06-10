// Do not touch, David's

#[allow(unused)]
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

    s_pointer: usize, // supervisor stack

    us_pointer: usize,
}

#[inline(always)]
fn sign_extend(val: u16, bits: usize) -> i16 {
    (val << (16 - bits)) as i16 >> (16 - bits)
}

#[allow(unused)]
impl CPU {
    fn fetch(&mut self) {
        self.mar = self.pc;
        self.pc += 1;

        /*self.acv = self.mar < 0x3000 || self.mar >= 0xf300 && (self.psr & 0x4000) > 0;

        if self.interrupt {
            // interrupts are not a thing rn
        }

        if self.acv {
            // also not a thing
        }
        */

        self.mdr = self.memory[self.mar as usize];
        self.ir = self.mdr;

        self.set_ben();

        self.decode();
    }

    fn set_ben(&mut self) {
        let n = self.nzp >> 2 & 1;
        let z = self.nzp >> 1 & 1;
        let p = self.nzp & 1;

        let ir_11 = ((self.ir & 0x800) >> 11) as u8;
        let ir_10 = ((self.ir & 0x400) >> 10) as u8;
        let ir_9 = ((self.ir & 0x200) >> 9) as u8;

        self.ben = (ir_11 & n | ir_10 & z | ir_9 & p) > 0;
    }

    // there are three different register positions
    // so u know what

    fn get_reg1(&self) -> u16 {
        let reg_mask = 0b111 << 9;
        let reg = (self.ir & reg_mask) >> 9;

        reg
    }

    fn get_reg1_val(&self) -> u16 {
        let reg_mask = 0b111 << 9;
        let reg = (self.ir & reg_mask) >> 9;

        self.registers[reg as usize]
    }

    fn get_reg2_val(&self) -> u16 {
        let reg_mask = 0b111 << 6;
        let reg = (self.ir & reg_mask) >> 6;

        self.registers[reg as usize]
    }

    fn get_reg3_val(&self) -> u16 {
        let reg_mask = 0b111;
        let reg = self.ir & reg_mask;

        self.registers[reg as usize]
    }

    fn set_nzp(&mut self, val: i16) {
        self.nzp = 0;
        if val == 0 {
            self.nzp = 0b010;
        } else if val > 0 {
            self.nzp = 0b001;
        } else {
            self.nzp = 0b100;
        }
    }

    fn decode(&mut self) {
        let opcode: u8 = ((self.ir & 0xF000) >> 12) as u8;
        match opcode {
            0b0001 => {
                // ADD
                let i_mode = self.ir & 0x20;
                if i_mode > 0 {
                    self.execute_add_imm();
                } else {
                    self.execute_add_reg();
                }
            }
            0b0101 => {
                // AND
                let i_mode = self.ir & 0x20;
                if i_mode > 0 {
                    self.execute_and_imm();
                } else {
                    self.execute_and_reg();
                }
            }
            0b0000 => {
                // BR
                if !self.ben {
                    return;
                }
                self.evaluate_pc_relative_address();
                self.pc = self.mar;
            }
            0b1100 => {
                // JMP / RET
                let val = self.get_reg2_val();
                self.pc = val;
            }
            0b0100 => {
                // JSR/JSRR
                self.registers[7] = self.pc;

                let i_mask = 1 << 11;
                if self.ir & i_mask != 0 {
                    let imm_mask = 0x7FF;
                    let imm = self.ir & imm_mask;
                    let ex_imm = sign_extend(imm, 11);

                    self.pc = (self.pc as i16 + ex_imm) as u16;
                } else {
                    let val = self.get_reg2_val();
                    self.pc = val;
                }
            }
            0b1001 => {
                // NOT
                self.execute_not();
            }

            0b0010 => {
                // LD
                self.evaluate_pc_relative_address();
                self.load_reg_from_memory();
            }
            0b1010 => {
                // LDI
                self.evaluate_pc_relative_address();
                self.mdr = self.memory[self.mar as usize];
                self.mar = self.mdr;
                self.load_reg_from_memory();
            }
            0b0110 => {
                // LDR
                self.evalate_base_offset_address();
                self.load_reg_from_memory();
            }

            0b1110 => {
                // LEA
                let imm_mask = 0x1FF;
                let imm = self.ir & imm_mask;
                let ex_imm = sign_extend(imm, 9);

                let reg = self.get_reg1();

                let val = self.pc as i16 + ex_imm;

                self.registers[reg as usize] = val as u16
            }

            0b0011 => {
                // ST
                self.evaluate_pc_relative_address();
                self.store_reg_to_memory();
            }

            0b1011 => {
                // STI
                self.evaluate_pc_relative_address();
                self.mdr = self.memory[self.mar as usize];
                self.mar = self.mdr;
                self.store_reg_to_memory();
            }
            0b0111 => {
                // STR
                self.evalate_base_offset_address();
                self.store_reg_to_memory();
            }

            0b1111 => {
                // TRAP
                self.trap_routine();
            }
            0b1000 => { // RTI
                self.return_from_trap();
            }
            _ => {}
        }
    }

    fn trap_routine(&mut self) {
        self.pc += 1;
        self.mdr = self.psr;

        let trap_vec = self.ir & 0xFF;

        self.psr &= !0x8000; // clear bit 15

        self.s_pointer -= 1;
        self.memory[self.s_pointer as usize] = self.mdr;

        self.s_pointer -= 1;
        self.memory[self.s_pointer as usize] = self.pc - 1;

        self.mdr = self.memory[trap_vec as usize];

        self.pc = self.mdr;
    }

    fn return_from_trap(&mut self) {
        self.mdr = self.memory[self.s_pointer as usize];
        self.pc = self.mdr;

        self.s_pointer += 1;
        
        self.mdr = self.memory[self.s_pointer as usize];
        self.psr = self.mdr;

        self.s_pointer += 1;
    }

    // these evaluate address functions will set mar

    // this is the same for indirect addressing
    fn evaluate_pc_relative_address(&mut self) {
        let imm_mask = 0x1FF;

        let imm = self.ir & imm_mask;

        let ex_imm = sign_extend(imm, 9);
        self.mar = (self.pc as i16 + ex_imm) as u16;
    }

    fn evalate_base_offset_address(&mut self) {
        let imm_mask = 0x3F;
        let imm = self.ir & imm_mask;

        let ex_imm = sign_extend(imm, 6);
        let reg_val = self.get_reg2_val();

        self.mar = (reg_val as i16 + ex_imm) as u16;
    }

    // ld,ldr,ldi

    fn load_reg_from_memory(&mut self) {
        self.mdr = self.memory[self.mar as usize];
        let reg = self.get_reg1();
        self.registers[reg as usize] = self.mdr;
        self.set_nzp(self.mdr as i16);
    }

    //sti,str,st

    fn store_reg_to_memory(&mut self) {
        let reg_val = self.get_reg1_val();
        self.mdr = reg_val;
        self.memory[self.mar as usize] = self.mdr;
    }

    fn execute_add_imm(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let imm = 0x1F & self.ir;
        let ex_imm = sign_extend(imm, 5);

        let val = sr1 as i16 + ex_imm;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    fn execute_and_imm(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let imm = 0x1F & self.ir;
        let ex_imm = sign_extend(imm, 5);

        let val = sr1 as i16 & ex_imm;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    fn execute_add_reg(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let sr2 = self.get_reg3_val() as i16;

        let val = sr1 + sr2;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    fn execute_and_reg(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let sr2 = self.get_reg3_val() as i16;

        let val = sr1 & sr2;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    fn execute_not(&mut self) {
        let dr = self.get_reg1();

        let val = !(self.get_reg2_val() as i16);
        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    pub fn run(&mut self) {
        while (self.mcr & 0x8000) != 0 {
            self.step();
        }
    }

    pub fn step(&mut self) {
        // program step
        self.fetch();
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

    pub fn set_program(&mut self, program: &[u16]) {
        let mut i = 0;
        for v in program {
            self.memory[i] = *v;
            i += 1;
        }
    }

    pub fn view_mcr(&self) -> u16 {
        self.mcr
    }

    pub fn view_nzp(&self) -> u8 {
        self.nzp
    }

    pub fn view_ben(&self) -> bool {
        self.ben
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            ir: 0,
            mdr: 0,
            mar: 0,
            mcr: 0x8000,
            pc: 0x0000,
            acv: false,
            memory: [0u16; 0xFFFF],
            registers: [0u16; 8],
            psr: 0,
            interrupt: false,
            ben: false,
            nzp: 0,
            s_pointer: 0x3000,
            us_pointer: 0xFDFF,
        }
    }
}
