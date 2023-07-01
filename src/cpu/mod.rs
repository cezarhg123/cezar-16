/// # Instructions Set
/// 
/// Register A and B are used as inputs for the ALU and the result is stored in register C
/// 
/// First 3 bits are for instructions. Last 3 bits are for opcodes if ALU is used
/// 
/// 000 - load to register D from program. eg 000 0101001101010 - load 2666
///
/// 001 - save to ram from register C
/// 
/// 010 - load to register D from ram
/// 
/// 011 - ALU
/// 
/// 100 - jump to address if 1 is in register C
/// 
/// 101 - jump to address if 0 is in register C
/// 
/// 110 - copy from register to register. example: 110 00 01 000000000 copies register A to register B
/// 
/// # ALU opcodes
/// 
/// ALU opcodes are stored at the end of the instruction. e.g 011 0000000000 000 = add
/// 
/// 000 - add
/// 
/// 001 - sub
/// 
/// 010 - equal. return 1 if regiser A & B are equal, 0 if not
/// 
/// # Registers
/// 
/// Register A = 00
/// 
/// Register B = 01
/// 
/// Register C = 10
/// 
/// Register D = 11
pub struct CPU {
    reg_a: u16,
    reg_b: u16,
    reg_c: u16,
    reg_d: u16,
    ram: [u16; 8000],
    program: [u16; 8000]
}

impl CPU {
    pub fn new(program: [u16; 8000]) -> CPU {
        CPU {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            ram: [0; 8000],
            program
        }
    }

    pub fn run(&mut self) {
        let mut counter = 0;
        while counter < 8000 {
            let data = self.program[counter as usize];

            // hopefully this gets the first 3 bits
            let instruction = data & 57344;

            match instruction {
                // 000
                0 => {
                    // this should get the data after the first 3 bits
                    let value = data & 8191;

                    self.reg_d = value;
                }
                // 001
                8192 => {
                    let address = data & 8191;

                    self.ram[address as usize] = self.reg_c;
                }
                // 010
                16384 => {
                    let address = data & 8191;

                    self.reg_d = self.ram[address as usize];
                }
                // 011
                24576 => {
                    let opcode = data & 7;

                    match opcode {
                        // add
                        0 => {
                            self.reg_c = self.reg_a + self.reg_b;
                        }
                        // sub
                        1 => {
                            self.reg_c = self.reg_a - self.reg_b;
                        }
                        // equal
                        2 => {
                            self.reg_c = if self.reg_a == self.reg_b {
                                1
                            } else {
                                0
                            }
                        }
                        _ => {}
                    }
                }
                // 100
                32768 => {
                    if self.reg_c == 1 {   
                        let address = data & 8191;
                        counter = address as usize;
                        continue;
                    }
                }
                // 101
                40960 => {
                    if self.reg_c == 0 {
                        let address = data & 8191;
                        counter = address as usize;
                        continue;
                    }
                }
                // 110
                49152 => {
                    let input_register = data & 6144;
                    let output_register = data & 1536;

                    let input_register_value = match input_register {
                        0 => {
                            self.reg_a
                        }
                        2048 => {
                            self.reg_b
                        }
                        4096 => {
                            self.reg_c
                        }
                        6144 => {
                            self.reg_d
                        }
                        _ => panic!("input register number is not valid")
                    };

                    match output_register {
                        0 => {
                            self.reg_a = input_register_value;
                        }
                        512 => {
                            self.reg_b = input_register_value;
                        }
                        1024 => {
                            self.reg_c = input_register_value;
                        }
                        1536 => {
                            self.reg_d = input_register_value;
                        }
                        _ => panic!("output register number is not valid")
                    }
                }
                _ => {}
            }
            counter += 1;
            // just to see if emulator is working properly
            println!("{}", self.reg_c);
        }
    }
}
