use std::{io::{Read, Write}, fs::File, collections::HashMap};



fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();

    match std::fs::File::open(filename) {
        Ok(mut file) => {
            let mut output_file = File::create("output.bin").unwrap();
            let mut labels: HashMap<String, u16> = HashMap::new();

            let mut source = String::new();
            file.read_to_string(&mut source).unwrap();
            
            let lines: Vec<&str> = source.lines().collect();
            let mut program_line_count = 0;

            for line in lines {
                let first_char = line.chars().next().unwrap_or('#');
                if first_char == '#' || first_char == '\n' {
                    continue;
                }

                if first_char == '.' {
                    let label = line.get(1..).unwrap();

                    labels.insert(label.to_string(), program_line_count);
                    continue;
                }

                let instruction = line.get(0..4).unwrap();

                match instruction {
                    // copies a value from one register to another
                    "move" => {
                        let (_, args) = line.split_once(" ").unwrap();

                        let (arg1, arg2) = args.split_once(":").unwrap();

                        let mut binary_instruction: u16 = 49152;
                        match arg1 {
                            "a" => {
                                binary_instruction = binary_instruction | 0;
                            }
                            "b" => {
                                binary_instruction = binary_instruction | 2048;
                            }
                            "c" => {
                                binary_instruction = binary_instruction | 4096;
                            }
                            "d" => {
                                binary_instruction = binary_instruction | 6144;
                            }
                            _ => {}
                        }
                        match arg2 {
                            "a" => {
                                binary_instruction = binary_instruction | 0;
                            }
                            "b" => {
                                binary_instruction = binary_instruction | 512;
                            }
                            "c" => {
                                binary_instruction = binary_instruction | 1024;
                            }
                            "d" => {
                                binary_instruction = binary_instruction | 1536;
                            }
                            _ => {}
                        }

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // load a value from the program
                    "lval" => {
                        let (_, arg) = line.split_once(" ").unwrap();

                        let mut binary_instruction: u16 = 0;
                        let value: u16 = arg.trim().parse().unwrap();
                        binary_instruction = binary_instruction | value;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // save a value to the ram
                    "save" => {
                        let (_, arg) = line.split_once(" ").unwrap();

                        let mut binary_instruction: u16 = 8192;
                        // address in ram
                        let value: u16 = arg.trim().parse().unwrap();
                        binary_instruction = binary_instruction | value;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // load a value from the ram
                    "load" => {
                        let (_, arg) = line.split_once(" ").unwrap();

                        let mut binary_instruction: u16 = 16384;
                        // address in ram
                        let value: u16 = arg.trim().parse().unwrap();
                        binary_instruction = binary_instruction | value;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // is name addn because it needs to be 4 characters long
                    "addn" => {
                        let binary_instruction: u16 = 24576;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // is name subn because it needs to be 4 characters long
                    "subn" => {
                        let binary_instruction: u16 = 24577;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    "equl" => {
                        let binary_instruction: u16 = 24578;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // jump if true or 1 is in register C
                    "jmpt" => {
                        let (_, arg) = line.split_once(" ").unwrap();

                        let mut binary_instruction: u16 = 32768;
                        // address in ram
                        let value: u16 = labels.get(arg).unwrap().clone();
                        binary_instruction = binary_instruction | value;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    // jump if false or 0 is in register C
                    "jmpf" => {
                        let (_, arg) = line.split_once(" ").unwrap();

                        let mut binary_instruction: u16 = 40960;
                        // address in ram
                        let value: u16 = labels.get(arg).unwrap().clone();
                        binary_instruction = binary_instruction | value;

                        output_file.write(binary_instruction.to_be_bytes().as_slice()).unwrap();
                    }
                    _ => {}
                }
                program_line_count += 1;
            } 
        }
        Err(e) => {
            println!("{e}");
        }
    }
}