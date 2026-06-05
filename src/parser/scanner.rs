// Do not touch, David's

use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum Token {
    Label(String),
    Opcode(String),
    Register(String),
    Integer(String),
    Directive(String),
}

#[derive(Debug)]
pub enum TokenError {
    OutOfBounds(String),
    UnknownRegister(String),
    UnknownToken(Token),
    EmptyProgram,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::OutOfBounds(value) => {
                write!(f, "{value} exceeds the limit for this operation")
            }
            TokenError::UnknownRegister(register) => {
                write!(f, "{register} is not a valid register, only R0-R7 exists")
            }

            TokenError::UnknownToken(token) => write!(f, "{token:?} was not expected here"),
            TokenError::EmptyProgram => write!(f, "Program file is empty!")
        }
    }
}

#[inline(always)]
fn starts_opcode(c: char) -> bool {
    match c {
        'A' | 'B' | 'J' | 'L' | 'N' | 'R' | 'S' | 'T' | 'H' => true,
        _ => false,
    }
}

#[inline(always)]
fn is_seperator(c: char) -> bool {
    match c {
        ' ' | '\n' | '\r' | ',' | '\0' => true,
        _ => false,
    }
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let chars = text.chars().collect::<Vec<char>>();
    let mut char_stack: Vec<char> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();

    let mut string_pos = 0;
    let mut cur_char;

    while string_pos < text.len() {
        cur_char = chars[string_pos];

        if cur_char == ';' {
            while string_pos < text.len() && cur_char != '\n' {
                string_pos += 1;
                cur_char = chars[string_pos];
            }
            if cur_char == '\n' {
                continue;
            }
        }

        if !char_stack.is_empty() && (is_seperator(cur_char)) {
            if char_stack[0] == 'R' && char_stack[1].is_numeric() {
                tokens.push(Token::Register(char_stack.iter().collect()));
            } else if starts_opcode(char_stack[0]) {
                let op = find_opcode(&char_stack);
                if op.is_some() {
                    tokens.push(op.unwrap());
                } else {
                    tokens.push(Token::Label(char_stack.iter().collect()));
                }
            } else if char_stack[0] == '#' || char_stack[0] == 'x' {
                tokens.push(Token::Integer(char_stack.iter().collect()));
            } else if char_stack[0] == '.' {
                tokens.push(Token::Directive(char_stack.iter().collect()));
                if let Token::Directive(s) = &tokens[tokens.len() - 1] {
                    if s == ".END" {
                        char_stack.clear();
                        break;
                    }
                }
            } else {
                tokens.push(Token::Label(char_stack.iter().collect()));
            }
            char_stack.clear();
        } else {
            if !(is_seperator(cur_char)) {
                char_stack.push(cur_char);
            }
        }

        

        string_pos += 1;
    }


    if !char_stack.is_empty() {
        
        if char_stack.len() == 2 {
            if char_stack[0] == 'R' && char_stack[1].is_numeric() {
                tokens.push(Token::Register(char_stack.iter().collect()));
            }
        } else if starts_opcode(char_stack[0]) {
            let op = find_opcode(&char_stack);
            if op.is_some() {
                tokens.push(op.unwrap());
            } else {
                tokens.push(Token::Label(char_stack.iter().collect()));
            }
        } else if char_stack[0] == '#' || char_stack[0] == 'x' {
            tokens.push(Token::Integer(char_stack.iter().collect()));
        } else if char_stack[0] == '.' {
            tokens.push(Token::Directive(char_stack.iter().collect()));
        }
        char_stack.clear();
    }

    tokens
}

fn find_opcode(chars: &[char]) -> Option<Token> {
    let s: String = chars.iter().collect();
    match s.as_str() {
        "ADD" | "AND" | "NOT" | "BR" | "BRn" | "BRz" | "BRp" | "BRnz" | "BRnp" | "BRzp"
        | "BRnzp" | "JMP" | "JSR" | "JSRR" | "LD" | "LDI" | "LDR" | "LEA" | "RET" | "RTI"
        | "ST" | "STI" | "STR" | "TRAP" | "HALT" => Some(Token::Opcode(s)),
        _ => None,
    }
}
