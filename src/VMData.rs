extern crate byte;

use byte::*;
use byte::ctx::*;

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

pub struct Header {
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
    instructions: Vec<u32>
}

impl Data {
    pub fn new(filename: &str) -> () {
        read_file(filename)
    }
}

fn read_file(filename: &str) -> Data {

}