// Do not touch, David's

use std::fmt::Debug;

/// A trait for representing Nodes, rather than a struct as it 
/// allows for more flexibility
pub trait LCNode: Debug {
    /// Converts the node to its binary representation
    fn to_binary(&self) -> u16;
    /// Gets operation type
    fn get_operation(&self) -> Operation;
    /// Only used for labels, gets the user-defined name 
    fn get_label_name(&self) -> Option<&String>;
    /// Only used for labels, sets the actual offset of the instruction since
    /// it can only be known after creating the program representation
    fn init_label(&mut self, offset: i16);

}


#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    And,
    Br,
    Brp,
    Brn,
    Brz,
    Brpn,
    Brpz,
    Brnz,
    Jmp,
    Jsr,
    Jsrr,
    Ld,
    Ldi,
    Ldr,
    Lea,
    Not,
    Ret,
    Rti,
    St,
    Sti,
    Str,
    Trap,
    Literal // for rare cases when labels need to be represented by where in memory they are instead of offset
}

#[derive(Debug)]
pub enum TrapMode {
    Getc,
    Out,
    Puts,
    In,
    Halt,
}

#[derive(Debug)]
pub struct RegisterNode {
    value: u8,
}
impl RegisterNode {
    pub fn from(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct IntNode {
    value: i16,
}

impl IntNode {
    pub fn from(value: i16) -> Self {
        Self { value }
    }
}

impl LCNode for IntNode {
    fn to_binary(&self) -> u16 {
        self.value as u16
    }

    fn get_operation(&self) -> Operation {
        Operation::Literal
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, offset: i16) {
        unreachable!()
    }
}



#[derive(Debug)]
pub enum ArithmeticOperand {
    Register(RegisterNode),
    Integer(IntNode),
}

#[derive(Debug)]
pub enum OffsetType {
    Label(LabelNode),
    Integer(IntNode),
}

#[derive(Debug)]
pub struct LabelNode {
    label: String,
    location: Option<i16>,
}

impl LabelNode {
    pub fn from(label: String) -> Self {
        Self {
            label,
            location: None,
        }
    }
}

impl LCNode for LabelNode {
    fn get_label_name(&self) -> Option<&String> {
        Some(&self.label)
    }

    fn init_label(&mut self, offset: i16) {
        self.location = Some(offset);
    }

    fn to_binary(&self) -> u16 {
        self.location.unwrap() as u16
    }
    fn get_operation(&self) -> Operation {
        Operation::Literal
    }
}


#[derive(Debug)]
pub struct ArithmeticNode {
    operation: Operation,
    operand1: RegisterNode,
    operand2: RegisterNode,
    operand3: ArithmeticOperand,
}

impl ArithmeticNode {
    pub fn from(
        operation: Operation,
        operand1: RegisterNode,
        operand2: RegisterNode,
        operand3: ArithmeticOperand,
    ) -> Self {
        Self {
            operation,
            operand1,
            operand2,
            operand3,
        }
    }
}

impl LCNode for ArithmeticNode {
    fn to_binary(&self) -> u16 {
        let opcode = match self.operation {
            Operation::Add => 0b0001 << 12,
            Operation::And => 0b0101 << 12,
            _ => unreachable!(),
        };

        let dr = (self.operand1.value as u16) << 9;
        let sr1 = (self.operand2.value as u16) << 6;

        match &self.operand3 {
            ArithmeticOperand::Register(sr2) => opcode | dr | sr1 | (sr2.value as u16),
            ArithmeticOperand::Integer(imm) => {
                opcode | dr | sr1 | (0b1 << 5) | (imm.value as u16) & ((0b1 << 5) - 1)
            }
        }
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}


#[allow(unused)]
#[derive(Debug)]
pub struct NotNode {
    operation: Operation,
    operand1: RegisterNode,
    operand2: RegisterNode,
}

impl NotNode {
    pub fn from(operation: Operation, operand1: RegisterNode, operand2: RegisterNode) -> Self {
        Self {
            operation,
            operand1,
            operand2,
        }
    }
}

impl LCNode for NotNode {
    fn to_binary(&self) -> u16 {
        let opcode: u16 = 0b1001 << 12;
        let dr = (self.operand1.value as u16) << 9;
        let sr = (self.operand2.value as u16) << 6;
        let imm_field: u16 = 0b111111;
        opcode | dr | sr | imm_field
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}

#[derive(Debug)]
pub struct MemOpNode {
    operation: Operation,
    operand1: RegisterNode,
    operand2: RegisterNode,
    offset: IntNode,
}

impl MemOpNode {
    pub fn from(
        operation: Operation,
        operand1: RegisterNode,
        operand2: RegisterNode,
        offset: IntNode,
    ) -> Self {
        Self {
            operation,
            operand1,
            operand2,
            offset,
        }
    }
}

impl LCNode for MemOpNode {
    fn to_binary(&self) -> u16 {
        let opcode = match self.operation {
            Operation::Ldr => 0b0110 << 12,
            Operation::Str => 0b0111 << 12,
            _ => unreachable!(),
        };
        let dr = (self.operand1.value as u16) << 9;
        let base_r = (self.operand2.value as u16) << 6;
        let offset6 = (self.offset.value as u16) & ((0b1 << 6) - 1);

        opcode | dr | base_r | offset6
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}

#[derive(Debug)]
pub struct IMemOpNode {
    operation: Operation,
    operand1: RegisterNode,
    offset: OffsetType,
}

impl IMemOpNode {
    pub fn from(operation: Operation, operand1: RegisterNode, offset: OffsetType) -> Self {
        Self {
            operation,
            operand1,
            offset,
        }
    }
}

impl LCNode for IMemOpNode {
    fn to_binary(&self) -> u16 {
        let opcode = match self.operation {
            Operation::Ld => 0b0010 << 12,
            Operation::St => 0b0011 << 12,
            Operation::Ldi => 0b1010 << 12,
            Operation::Sti => 0b1011 << 12,
            Operation::Lea => 0b1110 << 12,
            _ => unreachable!(),
        };
        let reg = (self.operand1.value as u16) << 9;

        let offset9 = match &self.offset {
            OffsetType::Integer(imm) => (imm.value as i16) & ((0b1 << 9) - 1),
            OffsetType::Label(label) => {
                (label.location.unwrap() as i16) & ((0b1 << 9) - 1) 
            }
        } as u16;

        opcode | reg | offset9
    }

    fn get_label_name(&self) -> Option<&String> {
        match &self.offset {
            OffsetType::Integer(_) => None,
            OffsetType::Label(l) => Some(&l.label),
        }
    }

    fn init_label(&mut self, offset: i16) {
        if let OffsetType::Label(label) = &mut self.offset {
            label.location = Some(offset)
        }
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}


#[allow(unused)]
#[derive(Debug)]
pub struct TrapNode {
    operation: Operation,
    val: TrapMode,
}

impl TrapNode {
    pub fn from(operation: Operation, val: TrapMode) -> Self {
        Self { operation, val }
    }
}

impl LCNode for TrapNode {
    fn to_binary(&self) -> u16 {
        let opcode = 0b1111 << 12;
        let vector: u16 = match self.val {
            TrapMode::Getc => 0x20,
            TrapMode::Out => 0x21,
            TrapMode::Puts => 0x22,
            TrapMode::In => 0x23,
            TrapMode::Halt => 0x25,
        };

        opcode | vector
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}

#[derive(Debug)]
pub struct JumpNode {
    operation: Operation,
    operand1: RegisterNode,
}

impl JumpNode {
    pub fn from(operation: Operation, operand1: RegisterNode) -> Self {
        Self {
            operation,
            operand1,
        }
    }
}

impl LCNode for JumpNode {
    fn to_binary(&self) -> u16 {
        let opcode = match self.operation {
            Operation::Jmp => 0b1100 << 12,
            Operation::Jsrr => 0b0100 << 12,
            _ => unreachable!(),
        };
        let base_r = (self.operand1.value as u16) << 6;

        opcode | base_r
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}

#[derive(Debug)]
pub struct IJumpNode {
    operation: Operation,
    offset: OffsetType,
}

impl IJumpNode {
    pub fn from(operation: Operation, offset: OffsetType) -> Self {
        Self { operation, offset }
    }
}

impl LCNode for IJumpNode {
    fn to_binary(&self) -> u16 {
        match self.operation {
            Operation::Br
            | Operation::Brn
            | Operation::Brnz
            | Operation::Brp
            | Operation::Brpn
            | Operation::Brpz
            | Operation::Brz => {
                let opcode = 0b0000 << 12;
                let nzp = match self.operation {
                    Operation::Br => 0b111,
                    Operation::Brn => 0b100,
                    Operation::Brz => 0b010,
                    Operation::Brp => 0b001,
                    Operation::Brnz => 0b110,
                    Operation::Brpn => 0b101,
                    Operation::Brpz => 0b011,
                    _ => unreachable!(),
                } << 9;

                let offset9 = match &self.offset {
                    OffsetType::Integer(imm) => (imm.value) & ((0b1 << 9) - 1),
                    OffsetType::Label(label) => (label.location.unwrap() as i16) & ((0b1 << 9) - 1),
                } as u16;

                opcode | nzp | offset9
            }
            Operation::Jsrr => {
                let opcode = 0b0100 << 12;
                let bit = 1 << 11;
                let offset11 = match &self.offset {
                    OffsetType::Integer(imm) => (imm.value) & ((0b1 << 11) - 1),
                    OffsetType::Label(label) => {
                        (label.location.unwrap() as i16) & ((0b1 << 11) - 1)
                    }
                } as u16;
                opcode | bit | offset11
            }
            _ => unreachable!(),
        }
    }

    fn get_label_name(&self) -> Option<&String> {
        match &self.offset {
            OffsetType::Integer(_) => None,
            OffsetType::Label(l) => Some(&l.label),
        }
    }

    fn init_label(&mut self, offset: i16) {
        if let OffsetType::Label(label) = &mut self.offset {
            label.location = Some(offset)
        }
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}


#[allow(unused)]
#[derive(Debug)]
pub struct RetNode {
    operation: Operation,
}



#[allow(unused)]
impl RetNode {
    pub fn from(operation: Operation) -> Self {
        Self { operation }
    }
}

impl LCNode for RetNode {
    fn to_binary(&self) -> u16 {
        let instruction = 0b1100000111000000;
        instruction
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}


#[allow(unused)]
#[derive(Debug)]
pub struct RtiNode {
    operation: Operation,
}



#[allow(unused)]

impl RtiNode {
    pub fn from(operation: Operation) -> Self {
        Self { operation }
    }
}

impl LCNode for RtiNode {
    fn to_binary(&self) -> u16 {
        let instruction = 0b1000000000000000;
        instruction
    }

    fn get_label_name(&self) -> Option<&String> {
        None
    }

    fn init_label(&mut self, _: i16) {
        unreachable!()
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}
