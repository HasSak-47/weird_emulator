use std::{fmt::{Display, Debug}, default};
use super::stack::Stack;
use super::ram::RAM;

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CmpFlag{
    #[default]
    Different = 0,

    Equal    = 1,
    Lesser   = 2,
    Bigger   = 3,
}

impl std::fmt::Display for CmpFlag{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Different=> "diff",
            Self::Equal    => "equal",
            Self::Lesser   => "less",
            Self::Bigger   => "Big",
        })
    }
}


#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug)]
pub enum Instruction{
    // arithmetic instructions
    Add = 0x00,
    Sub = 0x01,
    Mul = 0x02,
    Div = 0x03,
    Mod = 0x04,
        //float
    FAdd = 0x05,
    FSub = 0x06,
    FMul = 0x07,
    FDiv = 0x08,

    // bitwise instructions
    And = 0x10,
    Or  = 0x11,
    Not = 0x12,

    // jmp instructions
    Jmp = 0x20,
    JNE = 0x21,
    JE  = 0x22,
    JL  = 0x23,
    JB  = 0x24,
    JEL = 0x25,
    JEB = 0x26,

    // stack instructions
    Push = 0x30,
    Pop  = 0x31,
    // cmp instructions
    Cmp  = 0x40,
    // move instructions
    Mov  = 0x50,
    // calls
    Ker  = 0x60,
    Call = 0x61,
    // rel calls the emulator
    Rel  = 0x62,

    // Other
    #[default]
    End  = 0xf0,
    ERR  = 0xff,
}


#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug)]
enum ReadFlag{
    #[default]
    REG = 0x0,
    RAM = 0x1,
    VAL = 0x2,
    ARG = 0x3,
    ARM = 0x4,
}

impl Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

// unsafe code goes brrrrr
impl From<u8> for Instruction{
    fn from(value: u8) -> Self {
        unsafe{
            let ptr: *const u8= &value;

            *(ptr as *const Instruction)
        }
    }
}


#[derive(Clone, Copy)]
pub struct Processor<T: 'static, const RAM_LEN: usize, const STACK_LEN: usize>{
    pub cmp_flag: CmpFlag, 

    // memory
    pub reg:      [T; 4],
    pub stk: Stack<T, STACK_LEN>,
    pub ram:   RAM<T, RAM_LEN>,

    pub inst_ptr: usize,
}

