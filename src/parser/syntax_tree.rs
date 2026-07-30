// Do not touch, David's, comment on line 28

use std::collections::HashMap;

use crate::parser::{
    node::{
        ArithmeticNode, ArithmeticOperand, IJumpNode, IMemOpNode, IntNode, JumpNode, LCNode,
        LabelNode, MemOpNode, NotNode, OffsetType, Operation, RegisterNode, RetNode, RtiNode,
        TrapMode, TrapNode,
    },
    program::{LabelMap, NodeVec, Program},
    scanner::{Token, TokenError},
};


#[doc(hidden)]
fn convert_hex_int(s: &String) -> Option<u16> {
    let n = u16::from_str_radix(&s.trim()[1..], 16);
    n.ok()
}

#[doc(hidden)]
fn directive_convert_int(s: &String) -> Option<u16> {
    if s.starts_with('x') {
        convert_hex_int(s)
    } else {
        s.parse::<u16>().ok()
    }
}

pub fn scan_sequence(tokens: Vec<Token>) -> Result<Program, TokenError> {
    let mut pos = 0;

    let mut labels: LabelMap = HashMap::new();
    let mut nodes: NodeVec = vec![];

    let mut program_start: u16 = 0;
    let mut mem_loc_count = 0; // how many memory locations the program has taken up so far

    while pos < tokens.len() {
        match &tokens[pos] {
            Token::Directive(s) => {
                match s.as_str() {
                    ".FILL" => {
                        mem_loc_count += 1;
                        match &tokens[pos + 1] {
                            Token::Integer(s) => {
                                let int_val = directive_convert_int(s);
                                if int_val.is_none() {
                                    return Err(TokenError::MalformedInteger);
                                }
                                nodes.push(Box::from(IntNode::from(int_val.unwrap() as i16)));
                            }
                            Token::Label(label) => {
                                nodes.push(Box::from(LabelNode::from(label.to_string())));
                            }

                            _ => unreachable!(),
                        };
                    }
                    ".ORIG" => {
                        let int_str = match &tokens[pos + 1] {
                            Token::Integer(s) => s,
                            _ => unreachable!(),
                        };

                        let int_val = directive_convert_int(int_str);
                        if int_val.is_none() {
                            return Err(TokenError::MalformedInteger);
                        }

                        program_start = int_val.unwrap();
                    }
                    ".BLKW" => {}

                    ".STRINGZ" => match &tokens[pos + 1] {
                        Token::String(s) => {
                            for c in s.chars().collect::<Vec<char>>() {
                                nodes.push(Box::from(IntNode::from(c as i16)));
                            }
                            mem_loc_count += s.len();
                            nodes.push(Box::from(IntNode::from(0)));
                        }
                        _ => {
                            return Err(TokenError::UnknownToken(tokens[pos + 1].clone()));
                        }
                    },

                    _ => {}
                }
                pos += 2;
                continue;
            }
            Token::Opcode(s) => {
                mem_loc_count += 1;
                let count = token_count(s.as_str());
                let slice = &tokens[pos..pos + count + 1]; // why add 1?? is this a pattern in assembly?? 

                let node: Box<dyn LCNode> = match s.as_str() {
                    "ADD" | "AND" => Box::from(create_arithmetic_node(slice)?),
                    "LDR" | "STR" => Box::from(create_memory_node(slice)?),
                    "JMP" | "JSRR" => Box::from(create_jump_node(slice)?),
                    "BR" | "BRn" | "BRz" | "BRp" | "JSR" | "BRnz" | "BRzp" | "BRnp" | "BRnzp" => {
                        Box::from(create_ijump_node(slice)?)
                    }
                    "NOT" => Box::from(create_not_node(slice)?),
                    "LD" | "LDI" | "LEA" | "ST" | "STI" => Box::from(create_imemory_node(slice)?),
                    "HALT" | "TRAP" | "GETC" | "OUT" | "PUTS" | "IN" | "PUTSP"  => Box::from(create_trap_node(slice)?),
                    "RET" => Box::from(create_ret_node()),
                    "RTI" => Box::from(create_rti_node()),
                    _ => return Err(TokenError::UnknownToken(tokens[pos].clone())),
                };

                nodes.push(node);
                pos += count + 1;
            }
            Token::Label(s) => {
                labels.insert(s.to_string(), mem_loc_count as u16);
                pos += 1;
            }
            _ => return Err(TokenError::UnknownToken(tokens[pos].clone())),
        }
    }

    Ok(Program::from(nodes, labels, program_start))
}


// How many tokens follow an opcode to specify the full instruction
#[doc(hidden)]
fn token_count(s: &str) -> usize {
    match s {
        "ADD" | "AND" | "LDR" | "STR" => 3,
        "NOT" | "LD" | "LDI" | "LEA" | "ST" | "STI" => 2,
        "TRAP" | "BR" | "BRn" | "BRz" | "BRp" | "BRnz" | "BRzp" | "BRnp" | "BRnzp" | "JMP"
        | "JSR" | "JSRR" => 1,
        "HALT" | "RET" | "RTI" => 0,
        _ => 0,
    }
}


// numbers in the form #[num]
#[doc(hidden)]
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

#[doc(hidden)]
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
/// Creates an arithmetic node using a slice of tokens.
/// 
/// # Arguments
/// A slice of tokens containing the operation, two register tokens, and either an integer or register
/// 
/// # Returns
/// A node representing an arithmetic operation
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



/// Creates a not node using a slice of tokens.
/// 
/// # Arguments
/// A slice of tokens containing the operation, and two registers
/// 
/// # Returns
/// A node representing a not operation
fn create_not_node(tokens: &[Token]) -> Result<NotNode, TokenError> {
    let operand1 = convert_register(&tokens[1])?;
    let operand2 = convert_register(&tokens[2])?;

    Ok(NotNode::from(Operation::Not, operand1, operand2))
}


/// Creates an RET node using a slice of tokens.
fn create_ret_node() -> RetNode {
    RetNode::from(Operation::Ret)
}



/// Creates an RTI node using a slice of tokens.
fn create_rti_node() -> RtiNode {
    RtiNode::from(Operation::Rti)
}



/// Creates a node representing either LDR or STR using a slice of tokens.
/// 
/// # Arguments
/// A slice of tokens containing the operation, two operands, and an offset
/// 
/// # Returns
/// A node representing a memory operation
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



/// Creates a node representing an immediate memory access operation using a slice of tokens.
/// 
/// # Arguments
/// A slice of tokens containing the operation, register operand, and an offset
/// 
/// # Returns
/// A node representing a memory operation
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



/// Creates a node representing a trap instruction using a slice of tokens.
/// 
/// # Arguments
/// A slice of tokens containing the operation and trap vector, or a named trap function
/// 
/// # Returns
/// A node representing a trap operation
fn create_trap_node(tokens: &[Token]) -> Result<TrapNode, TokenError> {
    if tokens.len() == 1 {
        let token = &tokens[0];
        match token {
            Token::Opcode(s) => {

                match s.as_str() {
                    "GETC" => {
                        return Ok(TrapNode::from(Operation::Trap, TrapMode::Getc))
                    },
                    "OUT" => {

                        return Ok(TrapNode::from(Operation::Trap, TrapMode::Out))
                    },
                    "PUTS" => {
                        return Ok(TrapNode::from(Operation::Trap, TrapMode::Puts))
                    },
                    "IN" => {
                        
                        return Ok(TrapNode::from(Operation::Trap, TrapMode::In))
                    },
                    "PUTSP" => {

                        return Ok(TrapNode::from(Operation::Trap, TrapMode::Puts))
                    },
                    "HALT" => {
                        return Ok(TrapNode::from(Operation::Trap, TrapMode::Halt))
                    }
                    _ => {
                        return Err(TokenError::UnknownToken(token.clone()))
                    }
                }
            }
            _ => return Err(TokenError::UnknownToken(token.clone())),
        }
    }

    let val = &tokens[1];

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



/// Creates a node representing either JMP or JSRR using a slice of tokens.
/// 
/// # Arguments
/// A slice of tokens containing the operation, and operand
/// 
/// # Returns
/// A node representing a jump operation
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



/// Creates a node representing an immediate jump instruction.
/// 
/// # Arguments
/// A slice of tokens containing the operation, and offset
/// 
/// # Returns
/// A node representing a jump operation
fn create_ijump_node(tokens: &[Token]) -> Result<IJumpNode, TokenError> {
    let operation = match &tokens[0] {
        Token::Opcode(s) => match s.as_str() {
            "BR" | "BRnzp" | "BRnpz" | "BRpzn" | "BRpnz" | "BRznp" | "BRzpn" => Operation::Br,
            "BRz" => Operation::Brz,
            "BRn" => Operation::Brn,
            "BRp" => Operation::Brp,
            "BRnp" | "BRpn" => Operation::Brpn,
            "BRzp" | "BRpz" => Operation::Brpz,
            "BRnz" | "BRzn" => Operation::Brnz,
            "JSR" => Operation::Jsr,
            _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
        },
        _ => return Err(TokenError::UnknownToken(tokens[0].clone())),
    };

    let offset = match &tokens[1] {
        Token::Label(s) => OffsetType::Label(LabelNode::from(s.to_string())),
        Token::Integer(_) => {
            let res = convert_integer(&tokens[1], 9)?;
            OffsetType::Integer(res)
        }
        _ => return Err(TokenError::UnknownToken(tokens[1].clone())),
    };

    Ok(IJumpNode::from(operation, offset))
}
