// Do not touch, David's

use std::{error::Error, fmt};

use crate::parser::scanner::TokenError::{MissingEndDirective, MissingOrigDirective};

/// The smallest components of a program, the enum type describes
/// what kind while the String keeps the original data
#[derive(Debug, Clone)]
pub enum Token {
    Label(String),
    Opcode(String),
    Register(String),
    Integer(String),
    Directive(String),
    String(String),
    _Block,
}

// this should be expanded, more descriptive error messages and such
#[derive(Debug)]
pub enum TokenError {
    OutOfBounds(String),
    UnknownRegister(String),
    UnknownToken(Token),
    EmptyProgram,
    MalformedInteger,
    MissingEndDirective,
    MissingOrigDirective,
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
            TokenError::EmptyProgram => write!(f, "Program file is empty!"),
            TokenError::MalformedInteger => write!(f, "Program contains a malformed integer"),
            TokenError::MissingEndDirective => write!(f, "Your file is missing an '.END' directive"),
            TokenError::MissingOrigDirective => write!(f, "Your program is missing an '.ORIG' directive")
        }
    }
}

#[inline(always)]
#[doc(hidden)]
fn starts_opcode(c: char) -> bool {
    match c {
        'A' | 'B' | 'J' | 'L' | 'N' | 'R' | 'S' | 'T' | 'H' | 'P' | 'G'
        | 'O' | 'I' => true,
        _ => false,
    }
}

#[inline(always)]
#[doc(hidden)]
fn is_seperator(c: char) -> bool {
    match c {
        ' ' | '\n' | '\r' | ',' | '\0' => true,
        _ => false,
    }
}

/// Takes in a string and converts it to a vector of tokens
pub fn tokenize(text: &str) -> Result<Vec<Token>, TokenError> {
    let chars = text.chars().collect::<Vec<char>>();
    let mut char_stack: Vec<char> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();

    let mut string_pos = 0;
    let mut cur_char;


    // every program needs a .END directive
    let mut has_end = false;

    while string_pos < text.len() {
        cur_char = chars[string_pos];

        // im just ignoring comments
        if cur_char == ';' {
            while string_pos < text.len() && cur_char != '\n' {
                string_pos += 1;
                cur_char = chars[string_pos];
            }
            if cur_char == '\n' {
                continue;
            }
        }

        // if its whitespace, we know its a word so can look at whats on the stack
        if !char_stack.is_empty() && (is_seperator(cur_char)) {
            // if its a register
            if char_stack[0] == 'R' && char_stack[1].is_numeric() {
                tokens.push(Token::Register(char_stack.iter().collect()));
            // if the first char is an opcode we can check if its one or its a label
            } else if starts_opcode(char_stack[0]) {
                let op = find_opcode(&char_stack);
                if op.is_some() {
                    tokens.push(op.unwrap());
                } else {
                    tokens.push(Token::Label(char_stack.iter().collect()));
                }
            // if its a number
            } else if char_stack[0] == '#' || char_stack[0] == 'x' || char_stack[0].is_digit(10) {
                tokens.push(Token::Integer(char_stack.iter().collect()));
            // if its a directive
            } else if char_stack[0] == '.' {
                tokens.push(Token::Directive(char_stack.iter().collect()));
                if let Token::Directive(s) = &tokens[tokens.len() - 1] {
                    match s.as_str() {
                        ".BLKW"|
                        ".ORIG" => {
                            string_pos += 1;
                            while chars[string_pos] == ' ' {
                                string_pos += 1;
                            }
                            let mut count_end = string_pos + 1;
                            while !chars[count_end].is_whitespace() {
                                count_end += 1;
                            }
                            if chars[string_pos].is_digit(10) || chars[string_pos] == 'x' {
                                tokens.push(Token::Integer(text[string_pos..count_end].to_string()))
                            } else {
                                return Err(TokenError::MalformedInteger);
                            }
                            string_pos = count_end;
                        
                        },
                        
                        ".STRINGZ" => {
                            string_pos += 1;
                            while chars[string_pos] != '\"' {
                                string_pos += 1;
                            }
                            let s_start =  string_pos + 1;
                            let mut str_end = string_pos + 1;
                            while chars[str_end] != '\"' {
                                str_end += 1;
                            }
                            string_pos = str_end + 1;
                            tokens.push(Token::String(text[s_start..str_end].to_string()))
                        },
                        ".END" => {
                            has_end = true;
                            char_stack.clear();
                            break;
                        }
                        _ => {}
                    }
                }
            // if none of the above apply, call it a label
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

    // even though its the end of a file, have to still check the char stack
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
            if let Token::Directive(s) = &tokens[tokens.len() - 1] {
                    // Only need to check for .END as that should be the last line always
                    // (except it might not be but im going to look away)
                    match s.as_str() {
                       ".END" => {
                            has_end = true;
                            char_stack.clear();
                        }
                        _ => {}
                    }
                }
        }
        char_stack.clear();
    }

    if has_end {
        Ok(tokens)
    } else {
        Err(MissingEndDirective)
    }

}

/// Returns an opcode token if the string passed in represents an opcode
fn find_opcode(chars: &[char]) -> Option<Token> {
    let s: String = chars.iter().collect();
    match s.as_str() {
        "ADD" | "AND" | "NOT" | "BR" | "BRn" | "BRz" | "BRp" | "BRnz" | "BRnp" | "BRzp"
        | "BRnzp" | "JMP" | "JSR" | "JSRR" | "LD" | "LDI" | "LDR" | "LEA" | "RET" | "RTI"
        | "ST" | "STI" | "STR" | "TRAP" | "HALT" | "GETC" | "OUT" 
        | "PUTS" | "IN" | "PUTSP"   => Some(Token::Opcode(s)),
        _ => None,
    }
}
