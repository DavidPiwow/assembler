use std::{fs::File, io::Read};

mod parser;
mod cpu;

use parser::scanner::tokenize;

use parser::syntax_tree::scan_sequence;

use crate::cpu::CPU;

fn main() {
    let file = File::open("./code.asm");
    let mut contents = String::new();

    file.unwrap().read_to_string(&mut contents);

    let res = tokenize(&contents);

    println!("{:?}", scan_sequence(res));


    let mut test_cpu = CPU::default();

    let program = [0x1023, 0x1265, 0x1642, 0x98FF, 0xB801, 0,10];
    test_cpu.set_program(&program);

    
    println!("{:?}", test_cpu.view_registers());
    println!("{:?}", test_cpu.view_memory_slice(0,11));
    

    for _ in program {
        test_cpu.step();
        println!("{:?}", test_cpu.view_registers());
    }
    
   
    println!("{:?}", test_cpu.view_memory_slice(0,11));

}
