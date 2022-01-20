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

pub struct BaseData {
    magic_word: u32,
    version: u32,
    mem_len: u8,

    num_input_identifier: u32,
    input_identifier: Vec<InputIdentifer>,

    num_output_identifier: u32,
    output_identifier: Vec<OutputIdentifer>,

    num_immediates: u32,
    immediates: Vec<Immediate>,

    num_intructions: u32,
    instructions: Vec<u32>
}

impl BaseData {
    pub fn new(filename: &String) -> BaseData {
        
    }
}