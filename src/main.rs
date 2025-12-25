use std::fs;

mod instruction;
mod cpu;

fn main() {
    let mut cpu = cpu::cpu::new();

    // Gets the program path from command line arguments
    let args = std::env::args().collect::<Vec<String>>();
    let program_path = &args[1];
    println!("Loading program from {}", program_path);
    let program_data = fs::read(program_path).expect("Failed to read program file");
    cpu.load_program(&program_data);
    
    
}
