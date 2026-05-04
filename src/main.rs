use std::{fs::File, io::Read};

mod parser;

use parser::scanner::tokenize;

use parser::syntax_tree::scan_sequence;

fn main() {
    let mut file = File::open("./code.asm");
    let mut contents = String::new();

    file.unwrap().read_to_string(&mut contents);

    let res = tokenize(&contents);

    println!("{:?}", res);
    println!("{:?}", scan_sequence(res));
}
