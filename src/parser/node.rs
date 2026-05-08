use std::fmt::Debug;


pub trait LCNode: Debug {
    fn to_binary(&self) -> u16;
}

#[derive(Debug)]
pub enum Operation {
    Add,
    And,
    Br,
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
}

impl LabelNode {
    pub fn from(label: String) -> Self {
        Self { label }
    }
}

impl LCNode for LabelNode {
    fn to_binary(&self) -> u16 {
        todo!()
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
        todo!()
    }
}

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
        todo!()
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
        todo!()
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
        todo!()
    }
}

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
        todo!()
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
        todo!()
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
        todo!()
    }
}

#[derive(Debug)]
pub struct RetNode {
    operation: Operation,
}

impl RetNode {
    pub fn from(operation: Operation) -> Self {
        Self { operation }
    }
}

impl LCNode for RetNode {
    fn to_binary(&self) -> u16 {
        todo!()
    }
}

#[derive(Debug)]
pub struct RtiNode {
    operation: Operation,
}

impl RtiNode {
    pub fn from(operation: Operation) -> Self {
        Self { operation }
    }
}

impl LCNode for RtiNode {
    fn to_binary(&self) -> u16 {
        todo!()
    }
}
