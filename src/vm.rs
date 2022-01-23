use std::{path::Path};

use crate::vm_data::{read_file, Data, Instruction, Immediate};
use crate::opcodes::opcode_handling;

pub fn run(file_path: &Path) {
    let vm_data: Data = read_file(file_path);

    let mut ram_buffer: [f64; 255] = [0.0; 255];

    init(&vm_data.immediates, &mut ram_buffer);

    println!("{:?}", ram_buffer);

    execute(&vm_data.instructions, &mut ram_buffer);
}

//Prepare the ram buffer for the main execution loop
fn init(immediates: &Vec<Immediate>, ram_buffer: &mut [f64; 255]) {
    //Load the Immediates
    for i in 0..immediates.len() {
        ram_buffer[immediates[i].mem_loc as usize] = immediates[i].val;
    }
}

fn execute(instructions: &Vec<Instruction>, ram_buffer: &mut [f64; 255]) {
    for i in instructions {
        opcode_handling(ram_buffer, &i.opcode, i.data1, i.data2);
        println!("{:?}", i);
    }
}