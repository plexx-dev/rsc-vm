extern crate num_enum;
extern crate libm;

use libm::{tgamma, tgammaf};
use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Mov = 0, //move a into b (a -> b)

    //arithmetic operators
    Add = 1, //add b to a (a + b)
    Sub = 2, //subtract b from a (a - b)
    Mul = 3, //multiply a by b (a * b)
    Div = 4, //divide a by b (a / b)
    Mag = 5, //take the magnitude of a (a|)
    Fac = 6, //compute the factorial of a (a!)
    Pow = 7, //compute a to the power of b (a^b)
    Inc = 8, //increment a by b (a += b)
    Dec = 9, //decrement a by b (a -= b)
    Mas = 10, //multiply-assign a and b (a *= b)
    Das = 11, //divide-assign a and b (a /= b)
    Pas = 12, //power-assign a and b (a ^= b)

    //comparison
    Clt = 13, //set flag if a is less than b (a < b)
    Cgt = 14, //set flag if a is greater than b (a > b)
    Ceq = 15, //set flag if a is equal to b (a == b)
    Cne = 16, //set flag if a is not equal to b (a != b)

    //control flow
    Jpz = 17, //jump if flag is not set
    Jmp = 18, //jump unconditionally
    Hlt = 19, //halt execution
}

pub fn opcode_handling(ram_buffer: &mut [f64; 255], opcode: &Opcode, data1: u8, data2: u8) -> () {
    match opcode {
        Opcode::Mov => handle_mov(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Add => handle_add(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Sub => handle_sub(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Mul => handle_mul(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Div => handle_div(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Mag => handle_mag(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Fac => handle_fac(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Pow => handle_pow(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Inc => handle_inc(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Dec => handle_dec(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Mas => handle_mas(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Das => handle_das(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Pas => handle_pas(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Clt => handle_clt(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Cgt => handle_cgt(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Ceq => handle_ceq(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Cne => handle_cne(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Jpz => handle_jpz(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Jmp => handle_jmp(ram_buffer, data1 as usize, data2 as usize),
        Opcode::Hlt => handle_hlt(ram_buffer, data1 as usize, data2 as usize),
    }
}

fn handle_mov(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data2] = ram_buffer[data1];
}

// Arithmetic Operators

fn handle_add(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] += ram_buffer[data2];
}

fn handle_sub(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] -= ram_buffer[data2];
}

fn handle_mul(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] *= ram_buffer[data2];
}

fn handle_div(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] /= ram_buffer[data2];
}

fn handle_mag(ram_buffer: &mut [f64; 255], data1: usize, _data2: usize) {
    ram_buffer[data1] = ram_buffer[data1].abs();
}

fn handle_fac(ram_buffer: &mut [f64; 255], data1: usize, _data2: usize) {
    let val = ram_buffer[data1];

    if val <= 0.0 || val == 1.0 {
        ram_buffer[data1] = 1.0;
    } else {
        ram_buffer[data1] = tgamma(ram_buffer[data1] + 1.0);
    }
}

fn handle_pow(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] = ram_buffer[data1].powf(ram_buffer[data2]);
}

fn handle_inc(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] += ram_buffer[data2];
}

fn handle_dec(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] -= ram_buffer[data2];
}

fn handle_mas(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] *= ram_buffer[data2];
}

fn handle_das(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] /= ram_buffer[data2];
}

fn handle_pas(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    ram_buffer[data1] /= ram_buffer[data2];
}

// Comparison

fn handle_clt(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    if ram_buffer[data1] < ram_buffer[data2] {
        ram_buffer[0] = 1.0;
    } else {
        ram_buffer[0] = 0.0;
    }
}

fn handle_cgt(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    if ram_buffer[data1] > ram_buffer[data2] {
        ram_buffer[0] = 1.0;
    } else {
        ram_buffer[0] = 0.0;
    }
}

fn handle_ceq(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    if ram_buffer[data1] == ram_buffer[data2] {
        ram_buffer[0] = 1.0;
    } else {
        ram_buffer[0] = 0.0;
    }
}

fn handle_cne(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    if ram_buffer[data1] != ram_buffer[data2] {
        ram_buffer[0] = 1.0;
    } else {
        ram_buffer[0] = 0.0;
    }
}

// Control Flow

fn handle_jpz(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    todo!();
}

fn handle_jmp(ram_buffer: &mut [f64; 255], data1: usize, data2: usize) {
    todo!();
}

fn handle_hlt(ram_buffer: &mut [f64; 255], _data1: usize, _data2: usize) {
    todo!();
}