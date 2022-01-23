extern crate num_enum;

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