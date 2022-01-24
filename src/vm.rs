use std::process::exit;
use std::time::Instant;
use std::{path::Path};

use crate::vm_data::{read_file, Data, Instruction, Identifier};
use crate::opcodes::opcode_handling;

#[derive(Debug, Clone)]
pub struct VmState {
    pub ram_buffer: [f64; 255],
    pub flag: bool,
    pub hlt_flag: bool,

    pub pos: u32,

    pub instructions: Vec<Instruction>,
    pub args: Vec<f64>,
}

pub fn run(file_path: &Path, args: Vec<f64>) {
    let vm_data: Data = read_file(file_path);

    let ram_buffer: [f64; 255] = [0.0; 255];
    let mut state = VmState {ram_buffer,
        flag: false, 
        hlt_flag: false, 
        instructions: vm_data.instructions.clone(), 
        pos: 0, 
        args,
    };

    let start = Instant::now();

    init(&vm_data, &mut state);

    /*let mut i = 0;
    for n in &state.instructions {
        println!("[{}] {:?} ${} ${}", i, n.opcode, n.data1, n.data2);
        i+=1;
    }*/

    execute(&mut state);

    let duration = start.elapsed();
    println!("{:?}", duration);

    get_output(vm_data.output_identifier, &state);
}

//Prepare the vm for the main execution loop
fn init(vm_data: &Data, state: &mut VmState) {
    //Load the Immediates
    for i in 0..vm_data.immediates.len() {
        state.ram_buffer[vm_data.immediates[i].mem_loc as usize] = vm_data.immediates[i].val;
    }

    if state.args.len() != vm_data.input_identifier.len() {
        println!("Input values do not match the ones from the script. Please check your input variables!");
        exit(1);
    }

    for i in 0..state.args.len() {
        state.ram_buffer[vm_data.input_identifier[i].mem_loc as usize] = state.args[i];
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

    //println!("[DEBUG]\n{:?}", state.ram_buffer);
}