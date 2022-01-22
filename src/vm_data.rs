use std::process::exit;
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

use std::{fs::File, path::Path, error::Error, io::{Read}, convert::TryInto, fmt};

struct InputIdentifer {
    identifier: String,
    mem_loc: u8
}

struct OutputIdentifer {
    identifier: String,
    mem_loc: u8
}

struct Immediate {
    val: f64,
    mem_loc: u8
}

struct Instruction {
    opcode: u8,
    data1: u8,
    data2: u8,
    zero: u8
}

#[derive(Debug)]
struct Header {
    magic_word: u32,
    version: u32,
    mem_len: u8,
}

pub struct Data {
    header: Header,

    num_input_identifier: u32,
    input_identifier: Vec<InputIdentifer>,

    num_output_identifier: u32,
    output_identifier: Vec<OutputIdentifer>,

    num_immediates: u32,
    immediates: Vec<Immediate>,

    num_intructions: u32,
    instructions: Instruction,
}

impl Data {
    pub fn new(file_path: &Path) -> () {
        read_file(file_path);
    }
}

const MAGIC_WORD: u32 = 0xF00D4;
fn read_file(file_path: &Path) -> () {
    let mut file = match File::open(file_path) {
        Err(e) => panic!("Couldnt open {}: {}", file_path.display(), e),
        Ok(file) => file,
    };

    let mut rsbf_data: Vec<u8> = Vec::new();

    match file.read_to_end(&mut rsbf_data) {
        Err(e) => panic!("Couldnt read {}: {}", file_path.display(), e),
        Ok(bytes) => println!("Successfully read {} bytes from {}", bytes, file_path.display()),
    }

    let header = read_header(&rsbf_data);

    println!("{:?}", header);

    if header.magic_word != MAGIC_WORD {
        println!("[ERROR] Magic Word did not match");
        exit(0x0100); // idk took some random code :)
    }


}

fn read_header(buffer: &Vec<u8>) -> Header {
    let magic_word: u32 = read_u32(&buffer, 0, 3);
    let version: u32 = read_u32(&buffer, 4, 7);
    let mem_len: u8 = read_u8(&buffer, 8);

    Header {magic_word, version, mem_len}
}

fn read_u8(buffer: &Vec<u8>, pos: usize) -> u8 {
    buffer[pos]
}

fn read_u32(buffer: &Vec<u8>, start_pos: usize, end_pos: usize) -> u32 {
    let mut tmp_buffer: [u8; 4] = [0; 4];

    let mut n = 0;
    for i in start_pos..=end_pos {
        tmp_buffer[n] = buffer[i];
        n+=1;
    }

    u32::from_ne_bytes(tmp_buffer)
}