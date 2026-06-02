// Do not touch, David's

use std::collections::HashMap;

use crate::parser::{
    node::{
        ArithmeticNode, ArithmeticOperand, IJumpNode, IMemOpNode, IntNode, JumpNode, LCNode, LabelNode, MemOpNode, NotNode, OffsetType, Operation, RegisterNode, TrapMode, TrapNode
    },
    program::{LabelMap, NodeVec, Program},
    scanner::{Token, TokenError},
};

pub fn scan_sequence(tokens: Vec<Token>) -> Result<Program, TokenError> {
    let mut pos = 0;
    let mut instr_count = 0;

    let mut labels: LabelMap = HashMap::new();
    let mut nodes: NodeVec = vec![];

    while pos < tokens.len() {
        match &tokens[pos] {
            Token::Directive(_) => {
                pos += 2;
                continue;
            }
            Token::Opcode(s) => {
                let count = token_count(s.as_str());
                let slice = &tokens[pos..pos + count + 1];

                let node: Box<dyn LCNode> = match s.as_str() {
                    "ADD" | "AND" => {
                        Box::from(create_arithmetic_node(slice)?)
                    }
                    "LDR" | "STR" => {
                        Box::from(create_memory_node(slice)?)
                    }
                    "JMP" | "JSRR" => {
                        Box::from(create_jump_node(slice)?)
                    }
                    "BR" | "BRn" | "BRz" | "BRp" | "JSR" => {
                        
                        Box::from(create_ijump_node(slice)?)
                    }
                    "NOT" => {
                        Box::from(create_not_node(slice)?)
                    }
                    "LD" | "LDI" | "LEA" | "ST" | "STI" => {
                        Box::from(create_imemory_node(slice)?)
                    }
                    "HALT" | "TRAP" => {
                        Box::from(create_trap_node(slice)?)
                    }
                    _ => return Err(TokenError::UnknownToken(tokens[pos].clone())),
                };

                nodes.push(node);
                pos += count + 1;
                instr_count += 1;
            }
            Token::Label(s) => {
                labels.insert(s.to_string(), instr_count);
                pos += 1;
            }
            _ => return Err(TokenError::UnknownToken(tokens[pos].clone())),
        }
    }

    Ok(Program::from(nodes, labels))
}

fn token_count(s: &str) -> usize {
    match s {
        "ADD" | "AND" | "LDR" | "STR" => 3,
        "NOT" | "LD" | "LDI" | "LEA" | "ST" | "STI" => 2,
        "TRAP"  | "BR" | "BRn" | "BRz" | "BRp" | "JMP" | "JSR" | "JSRR" => 1,
        "HALT" => 0,
        _ => 0,
    }
}

fn convert_integer(token: &Token, bits: u8) -> Result<IntNode, TokenError> {
    let bit_max = 1 << (bits - 1);

    if let Token::Integer(v) = token {
        let res = v[1..].parse::<i16>();
        return match res {
            Ok(v) => {
                if v >= -bit_max && v <= (bit_max - 1) {
                    Ok(IntNode::from(v))
                } else {
                    Err(TokenError::OutOfBounds(v.to_string()))
                }
            }
            Err(_) => Err(TokenError::UnknownToken(token.clone())),
        };
    }
    Err(TokenError::UnknownToken(token.clone()))
}

fn convert_register(token: &Token) -> Result<RegisterNode, TokenError> {
    match token {
        Token::Register(val) => {
            let res = val[1..].parse::<u8>();
            if let Ok(v) = res {
                if v <= 7 {
                    return Ok(RegisterNode::from(v));
                }
            }

            Err(TokenError::UnknownRegister(val.to_string()))
        }
        _ => Err(TokenError::UnknownToken(token.clone())),
    }
}

// Add/And op, op, op
// Add/And op, op, imm
fn create_arithmetic_node(tokens: &[Token]) -> Result<ArithmeticNode, TokenError> {
    let operation = match &tokens[0] {
        Token::Opcode(s) => match s.as_str() {
            "AND" => Operation::And,
            "ADD" => Operation::Add,
            _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
        },
        _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
    };

    let operand1 = convert_register(&tokens[1])?;

    let operand2 = convert_register(&tokens[2])?;

    let operand3 = &tokens[3];

    match operand3 {
        Token::Integer(_) => {
            let intnode = convert_integer(operand3, 5)?;

            Ok(ArithmeticNode::from(
                operation,
                operand1,
                operand2,
                ArithmeticOperand::Integer(intnode),
            ))
        }
        Token::Register(_) => {
            let regnode = convert_register(operand3)?;

            Ok(ArithmeticNode::from(
                operation,
                operand1,
                operand2,
                ArithmeticOperand::Register(regnode),
            ))
        }
        _ => Err(TokenError::UnknownToken(operand3.clone())),
    }
}

fn create_not_node(tokens: &[Token]) -> Result<NotNode, TokenError> {
    let operand1 = convert_register(&tokens[1])?;
    let operand2 = convert_register(&tokens[2])?;

    Ok(NotNode::from(Operation::Not, operand1, operand2))
}

fn create_memory_node(tokens: &[Token]) -> Result<MemOpNode, TokenError> {
    let operation = match &tokens[0] {
        Token::Opcode(s) => match s.as_str() {
            "LDR" => Operation::Ldr,
            "STR" => Operation::Str,
            _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
        },
        _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
    };

    let operand1 = convert_register(&tokens[1])?;
    let operand2 = convert_register(&tokens[2])?;
    let offset = convert_integer(&tokens[3], 6)?;

    Ok(MemOpNode::from(operation, operand1, operand2, offset))
}

fn create_imemory_node(tokens: &[Token]) -> Result<IMemOpNode, TokenError> {
    let operation = match &tokens[0] {
        Token::Opcode(s) => match s.as_str() {
            "LD" => Operation::Ld,
            "LDI" => Operation::Ldi,
            "LEA" => Operation::Lea,
            "ST" => Operation::St,
            "STI" => Operation::Sti,
            _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
        },
        _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
    };

    let operand1 = convert_register(&tokens[1])?;

    let offset = match &tokens[2] {
        Token::Label(s) => OffsetType::Label(LabelNode::from(s.to_string())),
        Token::Integer(_) => {
            let res = convert_integer(&tokens[2], 9)?;
            OffsetType::Integer(res)
        }
        _ => return Err(TokenError::UnknownToken(tokens[2].clone())),
    };

    Ok(IMemOpNode::from(operation, operand1, offset))
}

fn create_trap_node(tokens: &[Token]) -> Result<TrapNode, TokenError> {
    if tokens.len() == 1 {
        let token = &tokens[0];
        match token {
            Token::Opcode(s) => {
                return if s == "HALT" {
                    Ok(TrapNode::from(Operation::Trap, TrapMode::Halt))
                } else {
                    Err(TokenError::UnknownToken(token.clone()))    
                }
            },
            _ => return Err(TokenError::UnknownToken(token.clone())),
        }
    }

    let val = &tokens[1];
    println!("{tokens:?}");
    if let Token::Integer(s) = val {
        let trap_mode = match s.as_str() {
            "x20" => TrapMode::Getc,
            "x21" => TrapMode::Out,
            "x22" => TrapMode::Puts,
            "x23" => TrapMode::In,
            "x25" => TrapMode::Halt,
            _ => return Err(TokenError::UnknownToken(val.clone())),
        };


        return Ok(TrapNode::from(Operation::Trap, trap_mode));
    }

    Err(TokenError::UnknownToken(val.clone()))
}

fn create_jump_node(tokens: &[Token]) -> Result<JumpNode, TokenError> {
    let operation = match &tokens[0] {
        Token::Opcode(s) => match s.as_str() {
            "JMP" => Operation::Jmp,
            "JSRR" => Operation::Jsrr,
            _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
        },
        _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
    };

    let operand1 = convert_register(&tokens[1])?;

    Ok(JumpNode::from(operation, operand1))
}

fn create_ijump_node(tokens: &[Token]) -> Result<IJumpNode, TokenError> {
    let operation = match &tokens[0] {
        Token::Opcode(s) => match s.as_str() {
            "BR" | "BRz" | "BRp" | "BRn" | "BRnz" | "BRnp" | "BRzn" | "BRzp" | "BRpz" | "BRpn"
            | "BRnzp" | "BRnpz" | "BRpzn" | "BRpnz" | "BRznp" | "BRzpn" => Operation::Br,
            "JSR" => Operation::Jsr,
            _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
        },
        _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
    };

    let offset = match &tokens[1] {
        Token::Label(s) => OffsetType::Label(LabelNode::from(s.to_string())),
        Token::Integer(i) => {
            let res = convert_integer(&tokens[1], 9)?;
            OffsetType::Integer(res)
        }
        _ => return Err(TokenError::UnknownToken(tokens[1].clone())),
    };

    Ok(IJumpNode::from(operation, offset))
}
