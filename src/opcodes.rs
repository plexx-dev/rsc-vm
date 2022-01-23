extern crate num_enum;
extern crate libm;

use libm::{tgamma};
use num_enum::TryFromPrimitive;

use crate::vm::VmState;

#[derive(Debug, TryFromPrimitive, Clone)]
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

pub fn opcode_handling(state: &mut VmState) -> () {
    let instruction = &state.instructions[state.pos as usize];

    let opcode = &instruction.opcode;

    let data1 = instruction.data1;
    let data2 = instruction.data2;

    //println!("{:?} ${} ${}", opcode, data1, data2);

    match opcode {
        Opcode::Mov => handle_mov(state, data1 as usize, data2 as usize),
        Opcode::Add => handle_add(state, data1 as usize, data2 as usize),
        Opcode::Sub => handle_sub(state, data1 as usize, data2 as usize),
        Opcode::Mul => handle_mul(state, data1 as usize, data2 as usize),
        Opcode::Div => handle_div(state, data1 as usize, data2 as usize),
        Opcode::Mag => handle_mag(state, data1 as usize, data2 as usize),
        Opcode::Fac => handle_fac(state, data1 as usize, data2 as usize),
        Opcode::Pow => handle_pow(state, data1 as usize, data2 as usize),
        Opcode::Inc => handle_inc(state, data1 as usize, data2 as usize),
        Opcode::Dec => handle_dec(state, data1 as usize, data2 as usize),
        Opcode::Mas => handle_mas(state, data1 as usize, data2 as usize),
        Opcode::Das => handle_das(state, data1 as usize, data2 as usize),
        Opcode::Pas => handle_pas(state, data1 as usize, data2 as usize),
        Opcode::Clt => handle_clt(state, data1 as usize, data2 as usize),
        Opcode::Cgt => handle_cgt(state, data1 as usize, data2 as usize),
        Opcode::Ceq => handle_ceq(state, data1 as usize, data2 as usize),
        Opcode::Cne => handle_cne(state, data1 as usize, data2 as usize),
        Opcode::Jpz => handle_jpz(state, data1 as usize, data2 as usize),
        Opcode::Jmp => handle_jmp(state, data1 as usize, data2 as usize),
        Opcode::Hlt => handle_hlt(state, data1 as usize, data2 as usize),
    }
}

fn handle_mov(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[data2] = state.ram_buffer[data1];
}

// Arithmetic Operators

fn handle_add(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[0] = state.ram_buffer[data1] + state.ram_buffer[data2];
}

fn handle_sub(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[0] = state.ram_buffer[data1] - state.ram_buffer[data2];
}

fn handle_mul(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[0] = state.ram_buffer[data1] * state.ram_buffer[data2];
}

fn handle_div(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[0] = state.ram_buffer[data1] / state.ram_buffer[data2];
}

fn handle_mag(state: &mut VmState, data1: usize, _data2: usize) {
    state.ram_buffer[0] = state.ram_buffer[data1].abs();
}

fn handle_fac(state: &mut VmState, data1: usize, _data2: usize) {
    let val = state.ram_buffer[data1];

    if val == 0.0 || val == 1.0 {
        state.ram_buffer[0] = 1.0;
    } else {
        state.ram_buffer[0] = tgamma(state.ram_buffer[data1] + 1.0);
    }
}

fn handle_pow(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[0] = state.ram_buffer[data1].powf(state.ram_buffer[data2]);
}

fn handle_inc(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[data1] += state.ram_buffer[data2];
}

fn handle_dec(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[data1] -= state.ram_buffer[data2];
}

fn handle_mas(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[data1] *= state.ram_buffer[data2];
}

fn handle_das(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[data1] /= state.ram_buffer[data2];
}

fn handle_pas(state: &mut VmState, data1: usize, data2: usize) {
    state.ram_buffer[data1] = state.ram_buffer[data1].powf(state.ram_buffer[data2]);
}

// Comparison

fn handle_clt(state: &mut VmState, data1: usize, data2: usize) {
    if state.ram_buffer[data1] < state.ram_buffer[data2] {
        state.flag = true;
    }
}

fn handle_cgt(state: &mut VmState, data1: usize, data2: usize) {
    if state.ram_buffer[data1] > state.ram_buffer[data2] {
        state.flag = true;
    }
}

fn handle_ceq(state: &mut VmState, data1: usize, data2: usize) {
    if state.ram_buffer[data1] == state.ram_buffer[data2] {
        state.flag = true;
    }
}

fn handle_cne(state: &mut VmState, data1: usize, data2: usize) {
    if state.ram_buffer[data1] != state.ram_buffer[data2] {
        state.flag = true;
    }
}

// Control Flow

fn handle_jpz(state: &mut VmState, _data1: usize, _data2: usize) {
    if !state.flag {
        state.pos = (_data1 - 1) as u32;
    }
}

fn handle_jmp(state: &mut VmState, _data1: usize, _data2: usize) {
    state.pos = (_data1 - 1) as u32;
}

fn handle_hlt(state: &mut VmState, _data1: usize, _data2: usize) {
    state.hlt_flag = true;
}