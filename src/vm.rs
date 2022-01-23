use std::{path::Path};

use crate::vm_data::{read_file, Data, Instruction, Immediate, Identifier};
use crate::opcodes::opcode_handling;

#[derive(Debug, Clone)]
pub struct VmState {
    pub ram_buffer: [f64; 255],
    pub flag: bool,
    pub hlt_flag: bool,

    pub pos: u32,

    pub instructions: Vec<Instruction>
}

pub fn run(file_path: &Path) {
    let vm_data: Data = read_file(file_path);

    let ram_buffer: [f64; 255] = [0.0; 255];
    let mut state = VmState {ram_buffer, flag: false, hlt_flag: false, instructions: vm_data.instructions, pos: 0};

    init(&vm_data.immediates, &mut state);
    execute(&mut state);

    //println!("{:?}", state);

    get_output(vm_data.output_identifier, &state);
}

//Prepare the ram buffer for the main execution loop
fn init(immediates: &Vec<Immediate>, state: &mut VmState) {
    //Load the Immediates
    for i in 0..immediates.len() {
        state.ram_buffer[immediates[i].mem_loc as usize] = immediates[i].val;
    }
}

fn execute(state: &mut VmState) {

    loop {
        if state.hlt_flag {
            break;
        }
        opcode_handling(state);

        state.pos += 1;
    }
}

fn get_output(output_identifier: Vec<Identifier>, state: &VmState) {
    for n in output_identifier {
        println!("[OUTPUT]");
        println!("{} = {}", n.identifier, state.ram_buffer[n.mem_loc as usize]);
    }

    println!("[DEBUG]\n{:?}", state.ram_buffer);
}