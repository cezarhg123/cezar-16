pub mod cpu;

use cpu::CPU;

fn main() {
    let program_binary = std::fs::read("output.bin").unwrap();
    let mut program = [0; 8000];
    for i in 0..program_binary.len() / 2 {
        let binary_index = i * 2;
        let instruction = u16::from_be_bytes(program_binary[binary_index..binary_index+2].try_into().unwrap());
        program[i] = instruction;
    }

    let mut cpu = CPU::new(program);

    cpu.run();
}
