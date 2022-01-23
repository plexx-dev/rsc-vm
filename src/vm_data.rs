use std::{fs::File, path::Path, process::exit, io::Read};
use crate::opcodes::Opcode;

use std::convert::TryFrom;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

#[derive(Debug)]
pub struct Identifier {
    pub identifier: String,
    pub mem_loc: u8
}

#[derive(Debug)]
pub struct Immediate {
    pub val: f64,
    pub mem_loc: u8
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub data1: u8,
    pub data2: u8,
}

#[derive(Debug)]
pub struct Header {
    magic_word: u32,
    version: u32,
    mem_len: u8,
}

#[derive(Debug)]
pub struct Data {
    pub header: Header,

    pub input_identifier: Vec<Identifier>,

    pub output_identifier: Vec<Identifier>,

    pub immediates: Vec<Immediate>,

    pub instructions: Vec<Instruction>,
}

//Get the Data from the file and return it as a vec of bytes (u8)
fn get_data(file_path: &Path) -> Vec<u8> {
    let mut file = match File::open(file_path) {
        Err(e) => panic!("Couldnt open {}: {}", file_path.display(), e),
        Ok(file) => file,
    };

    let mut rsbf_data: Vec<u8> = Vec::new();

    match file.read_to_end(&mut rsbf_data) {
        Err(e) => panic!("Couldnt read {}: {}", file_path.display(), e),
        Ok(bytes) => println!("Successfully read {} bytes from {}", bytes, file_path.display()),
    }

    rsbf_data
}

const MAGIC_WORD: u32 = 0xF00D4;
pub fn read_file(file_path: &Path) -> Data {
    let rsbf_data = get_data(file_path);
    let next_pos = 0;

    //Read the Header
    let (header, next_pos) = read_header(&rsbf_data, next_pos);
    //println!("Checking Magic Word...");

    //Check Magic Word to check if the file could be a valid one
    if header.magic_word != MAGIC_WORD {
        println!("[ERROR] Magic Word did not match");
        exit(0x0100); // idk took some random code :) to exit here
    }
    //println!("Magic Word was correct");

    //println!("Checking Version...");
    if header.version != 4 {
        println!("Go to https://github.com/plexx-dev/rsc-vm and add a new Version to this shit idk man it's just not supported yet :(");
        exit(1);
    }
    //println!("Version supported");

    //println!("Version: {}\nMemory Size: {}bytes with {} Memory Addresses"
    //        , header.version, (header.mem_len as u64*4*8), header.mem_len);

    //Read the I/O Section
    let (num_input_identifier, next_pos) = read_u32(&rsbf_data, next_pos);
    let (input_identifier, next_pos) = read_identifier(&rsbf_data, next_pos, num_input_identifier);

    let (num_output_identifier, next_pos) = read_u32(&rsbf_data, next_pos);
    let (output_identifier, next_pos) = read_identifier(&rsbf_data, next_pos, num_output_identifier);

    //Read the Immediate Section
    let (num_immediates, next_pos) = read_u32(&rsbf_data, next_pos);
    let (immediates, next_pos) = read_immediates(&rsbf_data, next_pos, num_immediates);

    //Read the Text Section
    let (num_instruction, next_pos) = read_u32(&rsbf_data, next_pos);
    let (instructions, _next_pos) = read_instructions(&rsbf_data, next_pos, num_instruction);

    Data {header, input_identifier, output_identifier, immediates, instructions}
}

//Basic Function to read the Header
fn read_header(buffer: &Vec<u8>, start_pos: usize) -> (Header, usize) {
    let (magic_word, next_pos) = read_u32(&buffer, start_pos);
    let (version, next_pos) = read_u32(&buffer, next_pos);
    let (mem_len, next_pos) = read_u8(&buffer, next_pos);

    (Header {magic_word, version, mem_len}, next_pos)
}

//Basic Function to read a u8 from the buffer
fn read_u8(buffer: &Vec<u8>, pos: usize) -> (u8, usize) {
    (buffer[pos], pos + 1)
}

//Basic Function to read a u32 from the buffer
fn read_u32(buffer: &Vec<u8>, start_pos: usize) -> (u32, usize) {
    let mut tmp_buffer: [u8; 4] = [0; 4];

    let mut n = 0;
    for i in start_pos..start_pos+4 {
        tmp_buffer[n] = buffer[i];
        n+=1;
    }

    (u32::from_ne_bytes(tmp_buffer), start_pos + 4)
}

//Basic Function to read a f64 from the buffer
fn read_f64(buffer: &Vec<u8>, start_pos: usize) -> (f64, usize) {
    let mut tmp_buffer: [u8; 8] = [0; 8];

    let mut n = 0;
    for i in start_pos..start_pos+8 {
        tmp_buffer[n] = buffer[i];
        n+=1;
    }

    (f64::from_ne_bytes(tmp_buffer), start_pos + 8)
}

//Basic Function to read the input Identifier from the Buffer
fn read_identifier(buffer: &Vec<u8>, start_pos: usize, num: u32) -> (Vec<Identifier>, usize) {
    let mut next_pos = start_pos;
    let mut identifier_list: Vec<Identifier> = Vec::with_capacity(num as usize);
    
    for _ in 0..num as usize {
        let (identifier_len, mut _next_pos) = read_u32(buffer, next_pos);

        let mut string_buffer: Vec<u8> = Vec::with_capacity(identifier_len as usize);
        for __ in 0..identifier_len {
            let (identifier_char, __next_pos) = read_u8(buffer, _next_pos);
            string_buffer.push(identifier_char);

            _next_pos = __next_pos;
        }
        let identifier = std::string::String::from_utf8(string_buffer).unwrap();

        let (mem_loc, _next_pos) = read_u8(buffer, _next_pos);


        identifier_list.push(Identifier{identifier, mem_loc});
        next_pos = _next_pos;
    }

    (identifier_list, next_pos)
}

//Basic Function to read the Immediate values of the programm from the buffer
fn read_immediates(buffer: &Vec<u8>, start_pos: usize, num: u32) -> (Vec<Immediate>, usize) {
    let mut next_pos = start_pos;
    let mut immediate_list: Vec<Immediate> = Vec::with_capacity(num as usize);

    for _ in 0..num {
        let (val, _next_pos) = read_f64(&buffer, next_pos);
        let (mem_loc, _next_pos) = read_u8(&buffer, _next_pos);
        immediate_list.push(Immediate {val, mem_loc});

        next_pos = _next_pos;
    }

    (immediate_list ,next_pos)
}

//Basic Function to read the Instructions from the buffer
fn read_instructions(buffer: &Vec<u8>, start_pos: usize, num: u32) -> (Vec<Instruction>, usize) {
    let mut next_pos = start_pos;
    let mut instructions: Vec<Instruction> = Vec::with_capacity(num as usize);

    for _ in 0..num {
        let (zero, _next_pos) = read_u8(&buffer, next_pos);
        let (data2, _next_pos) = read_u8(&buffer, _next_pos);
        let (data1, _next_pos) = read_u8(&buffer, _next_pos);
        let (opcode_code, _next_pos) = read_u8(&buffer, _next_pos);
        let opcode: Opcode = Opcode::try_from(opcode_code).unwrap();
        
        if zero != 0 {
            println!("Something is very about this file {}", zero);
            exit(1);
        }

        instructions.push(Instruction {opcode, data1, data2});
        next_pos = _next_pos;
    }
    (instructions, next_pos)
}