// Do not touch, David's


const LC3_KBSR: u16 = 0xFE00;
const LC3_KBDR: u16 = 0xFE02;
const LC3_DDR: u16 = 0xFE06;

/// A struct representing an LC-3 CPU
/// 
/// 
#[allow(unused)]
pub struct CPU {
    /// Instruction register
    ir: u16,  
    /// Memory data register - the data to read/write
    mdr: u16, 
    /// Memory address register - where to read/write data
    mar: u16,
    /// Machine control register - only used for bit 15, the power bit for the CPU
    mcr: u16,
    /// Access control violation - has to be implemented fully
    acv: bool,
    interrupt: bool,
    /// Program counter - has a value of the current instruction + 1, as IR is used to store the current instruction
    pc: u16, 
    /// Memory locations - all memory is in 16 bit two's complement representation
    memory: [u16; 65536],
    /// Supervisor stack pointer - used for TRAP instructions
    s_pointer: usize, // supervisor stack
    us_pointer: usize,
    /// Registers - there are eight 16 bit registers available 
    registers: [u16; 8],
    /// Process status register
    /// 
    /// * PSR[15] is set to 1 for user mode, set to 0 for supervisor mode
    /// * PSR[3..0] are used for NZP values. These represent the result from 
    /// an arithmetic instruction and show whether it was **n**egative, **z**ero, or **p**ositive
    psr: u16,
    /// **B**ranch **en**able bit
    ben: bool,

}


#[doc(hidden)]
#[inline(always)]
// just a simple sign extend helper function
fn sign_extend(val: u16, bits: usize) -> i16 {
    (val << (16 - bits)) as i16 >> (16 - bits)
}

#[allow(unused)]
impl CPU {
    /// Runs a fetch cycle on the CPU
    /// 
    /// # Effects
    /// Sets the IR to the instruction that will be executed, and increments the PC.
    /// Also calls `set_ben()` and starts the decode cycle
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

    /// Sets the branch enable bit
    /// 
    /// # Effects
    /// Takes the instruction from `IR` and compares it to the `NZP` bits, 
    /// this will set `BEN` and determine if a branch is taken
    fn set_ben(&mut self) {
        let n = (self.psr >> 2 & 1) as u8;
        let z = (self.psr >> 1 & 1) as u8;
        let p = (self.psr & 1) as u8;

        let ir_11 = ((self.ir & 0x800) >> 11) as u8;
        let ir_10 = ((self.ir & 0x400) >> 10) as u8;
        let ir_9 = ((self.ir & 0x200) >> 9) as u8;

        self.ben = (ir_11 & n | ir_10 & z | ir_9 & p) > 0;
    }

    // there are three different register positions
    // so u know what


    #[doc(hidden)]
    fn get_reg1(&self) -> u16 {
        let reg_mask = 0b111 << 9;
        let reg = (self.ir & reg_mask) >> 9;

        reg
    }

    #[doc(hidden)]
    fn get_reg1_val(&self) -> u16 {
        let reg_mask = 0b111 << 9;
        let reg = (self.ir & reg_mask) >> 9;

        self.registers[reg as usize]
    }

    #[doc(hidden)]
    fn get_reg2_val(&self) -> u16 {
        let reg_mask = 0b111 << 6;
        let reg = (self.ir & reg_mask) >> 6;

        self.registers[reg as usize]
    }

    #[doc(hidden)]
    fn get_reg3_val(&self) -> u16 {
        let reg_mask = 0b111;
        let reg = self.ir & reg_mask;

        self.registers[reg as usize]
    }

    /// Sets NZP based on a value
    /// 
    /// # Arguments
    /// * `val` - Result from an instruction
    /// 
    /// # Effects
    /// Sets N if `val` is negative, Z if `val` is zero, and P if `val` is positive
    fn set_nzp(&mut self, val: i16) {
        self.psr &= 0xFFF8;
        if val == 0 {
            self.psr |= 0b010;
        } else if val > 0 {
            self.psr |= 0b001;
        } else {
            self.psr |= 0b100;
        }
    }


    /// Runs a decode cycle on the CPU
    /// 
    /// # Effects
    /// This will lead to the execution cycle of an instruction/trap routine,
    /// or change the value of PC for a jump command. 
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
                if self.mar == LC3_KBDR {
                    todo!()
                } else {
                    self.load_reg_from_memory();
                }
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
                if self.mar == LC3_DDR {
                    println!("{}", self.registers[0] as u8 as char);
                }
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


    /// Trap routine
    /// 
    /// # Effects
    /// * Increments `pc` and clears bit 15 in `PSR`, setting the CPU to supervisor mode.
    /// * Copies incremented `pc` and current `PSR` into the supervisor stack. 
    /// * Copies the memory address contained in the pointer from the trap vector and 
    /// sets `PC` to it.
    /// * In the case of the HALT routine, this also stops execution. 
    fn trap_routine(&mut self) {
        self.pc += 1;
        self.mdr = self.psr;

        let trap_vec = self.ir & 0xFF;
        if trap_vec == 0x25 { // halt
            self.mcr = 0;
            return;
        } else if trap_vec == 0x20 { // getc
            self.memory[LC3_KBDR as usize] = 'A' as u16;
            self.memory[LC3_KBSR as usize] = 0xFFFF; // signals there is data to read
        }

        self.psr &= !0x8000; // clear bit 15

        self.s_pointer -= 1;
        self.memory[self.s_pointer as usize] = self.mdr;

        self.s_pointer -= 1;
        self.memory[self.s_pointer as usize] = self.pc-1 ;

        self.mdr = self.memory[trap_vec as usize];

        self.pc = self.mdr;

        
    }


    // this will have to change because the way s pointer works is stupid

    /// The RTI instruction
    /// 
    /// # Effects
    /// Pops the saved `PC` and `PSR` from the stack, and restores these values in the CPU.
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
    /// Evaluates addressing for PC relative addresses
    /// 
    /// # Effects
    /// Sets `MAR` to `PC + immediate (sign extended)`
    fn evaluate_pc_relative_address(&mut self) {
        let imm_mask = 0x1FF;

        let imm = self.ir & imm_mask;

        let ex_imm = sign_extend(imm, 9);
        self.mar = (self.pc as i16 + ex_imm) as u16;
    }



    /// Evaluates addressing for base-offset relative addresses
    /// 
    /// # Effects
    /// Sets `MAR` to `register value + immediate (sign extended)`    
    fn evalate_base_offset_address(&mut self) {
        let imm_mask = 0x3F;
        let imm = self.ir & imm_mask;

        let ex_imm = sign_extend(imm, 6);
        let reg_val = self.get_reg2_val();

        self.mar = (reg_val as i16 + ex_imm) as u16;
    }

    // ld,ldr,ldi

    /// Loads a word from memory[MAR]
    /// 
    /// # Effects
    /// Sets NZP and register to memory[MAR]
    fn load_reg_from_memory(&mut self) {
        self.mdr = self.memory[self.mar as usize];
        let reg = self.get_reg1();
        self.registers[reg as usize] = self.mdr;
        self.set_nzp(self.mdr as i16);
    }

    //sti,str,st

    /// Stores a register value to MAR
    /// 
    /// # Effects
    /// Sets memory[MAR] to the register value
    fn store_reg_to_memory(&mut self) {
        let reg_val = self.get_reg1_val();
        self.mdr = reg_val;
        self.memory[self.mar as usize] = self.mdr;
    }

    /// Executes an ADD instruction that uses a register and immediate value s as data
    /// 
    /// # Effects
    /// Sets NZP and DR to SR1 + val
    fn execute_add_imm(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let imm = 0x1F & self.ir;
        let ex_imm = sign_extend(imm, 5);

        let val = sr1 as i16 + ex_imm;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    /// Executes an AND instruction that uses a register and immediate value s as data
    /// 
    /// # Effects
    /// Sets NZP and DR to SR1 & val
    fn execute_and_imm(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let imm = 0x1F & self.ir;
        let ex_imm = sign_extend(imm, 5);

        let val = sr1 as i16 & ex_imm;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    /// Executes an ADD instruction that uses two registers as data
    /// 
    /// # Effects
    /// Sets NZP and DR to SR1 + SR2 
    fn execute_add_reg(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let sr2 = self.get_reg3_val() as i16;

        let val = sr1 + sr2;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }


    /// Executes an AND instruction that uses two registers as data
    /// 
    /// # Effects
    /// Sets NZP and DR to SR1 & SR2 
    fn execute_and_reg(&mut self) {
        let dr = self.get_reg1();

        let sr1 = self.get_reg2_val() as i16;
        let sr2 = self.get_reg3_val() as i16;

        let val = sr1 & sr2;

        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }


    /// Executes a NOT instruction
    /// 
    /// # Effects
    /// Sets NZP and the DR to ~SR
    fn execute_not(&mut self) {
        let dr = self.get_reg1();

        let val = !(self.get_reg2_val() as i16);
        self.set_nzp(val);

        self.registers[dr as usize] = val as u16;
    }

    /// Continuously steps the CPU until the MCR power bit is cleared
    pub fn run(&mut self) {
        while self.mcr != 0 {
           // println!("MCR: {}", self.mcr);
            self.step();
        }
    }

    /// Calls the fetch cycle, represents an entire instruction cycle of the LC-3
    pub fn step(&mut self) {
        // program step
        self.fetch();
    }

    /// Helper function to show a slice of memory
    /// 
    /// # Arguments
    /// * `start` - Where in memory to begin the slice
    /// * `end`   - Where in memory to end the slice (exclusive)
    /// 
    /// # Returns 
    /// A slice of memory from start..end (exclusive)
    pub fn view_memory_slice(&self, start: usize, end: usize) -> &[u16] {
        // look at memory from start to end location
        &self.memory[start..end]
    }

    /// Helper function to show all of memory
    /// 
    /// # Returns
    /// A slice of all the current memory in the CPU
    pub fn view_all_memory(&self) -> &[u16] {
        &self.memory
    }

    /// Helper function to show the registers
    /// 
    /// # Returns
    /// A slice of all 8 registers
    pub fn view_registers(&self) -> &[u16; 8] {
        // view all 8 registers
        &self.registers
    }

    /// Helper function to show the PC value
    pub fn view_pc(&self) -> u16 {
        // get current pc value
        self.pc
    }


    /// Helper function to set a program
    /// 
    /// # Arguments
    /// * `program` - A slice of a program, in LC-3 machine code
    /// 
    /// # Effects
    /// Copies the slice into memory
    pub fn set_program(&mut self, start: u16, program: &[u16]) {
        self.pc = start;
        let mut i = 0;
        for v in program {
            self.memory[(start + i) as usize] = *v;
            i += 1;
        }
    }

    /// Helper function to show the MCR value
    pub fn view_mcr(&self) -> u16 {
        self.mcr
    }

    /// Helper function to show the NZP value
    pub fn view_nzp(&self) -> u8 {
        (self.psr & 0b111) as u8
    }

    /// Helper function to show the BEN value
    pub fn view_ben(&self) -> bool {
        self.ben
    }
}

impl Default for CPU {
    fn default() -> Self {
        let mut c = CPU {
            ir: 0,
            mdr: 0,
            mar: 0,
            mcr: 0x8000,
            pc: 0x0000,
            acv: false,
            memory: [0u16; 65536],
            registers: [0u16; 8],
            psr: 0,
            interrupt: false,
            ben: false,
            s_pointer: 0x3000,
            us_pointer: 0xFDFF,
        };
        c.memory[0xFE04] = 0x8000;
        return c;
    }
}
