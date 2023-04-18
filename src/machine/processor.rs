use std::fmt::{Display, Debug};
use super::stack::Stack;
use super::ram::RAM;

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CmpFlag{
    #[default]
    Different = 0b00,
    Lesser    = 0b01,
    Bigger    = 0b10,
    Equal     = 0b11,
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
    Rel  = 0x62,// rel calls the emulator
    Ret  = 0x6f,

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
    NON = 0xf,
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

#[allow(dead_code)]
pub trait Process{
    // arithmetic instructions
    fn add(&mut self);
    fn sub(&mut self);
    fn mul(&mut self);
    fn div(&mut self);
    fn mdi(&mut self);

    fn addf(&mut self);
    fn subf(&mut self);
    fn mulf(&mut self);
    fn divf(&mut self);

    // bitwise operations 
    fn and(&mut self);
    fn or (&mut self);
    fn not(&mut self);

    // jump instructions
    fn jmp(&mut self);
    fn jne(&mut self);
    fn  je(&mut self);
    fn  jl(&mut self);
    fn  jb(&mut self);
    fn jel(&mut self);
    fn jeb(&mut self);

    // stack
    fn push(&mut self);
    fn  pop(&mut self);

    //
    fn  cmp(&mut self);

    fn  mov(&mut self);

    fn  ker(&mut self);
    fn call(&mut self);
    fn  rel(&mut self);
    fn  ret(&mut self);

    fn  err(&mut self);
    fn  end(&mut self);

    fn get_inst(&self) -> Instruction;

    fn step(&mut self) {
        let inst = self.get_inst();
        use Instruction::*;
        // there must be a less bullshit way to implement this
        match inst{
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),

            And => self.and(),
            Or  => self.or(),
            Not => self.not(),

            Jmp => self.jmp(),
            JNE => self.jne(),
            JE  => self.je(),
            JL  => self.jl(),

            Push => self.push(),
            Pop  => self.pop(),

            Cmp => self.cmp(),

            Mov => self.mov(),

            Call => self.call(),
            End => self.end(),
            Ret => self.ret(),

            _ => {},
        }
    }

}


#[derive(Clone, Copy)]
pub struct Processor<MT, ST, GT, const RAM_LEN: usize, const STACK_LEN: usize>{
    pub cmp_flag: CmpFlag, 

    // memory
    pub reg:      [GT; 4],
    pub stk: Stack<ST, STACK_LEN>,
    pub ram:   RAM<MT, RAM_LEN>,

    pub inst_ptr: usize,
}

