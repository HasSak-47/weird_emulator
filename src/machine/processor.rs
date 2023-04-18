use std::fmt::{Display, Debug};
use super::{stack::Stack, ram::RAM, reg::Reg};

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
    // byte size dependent 
    // first nibble is the len of the arguments, the second is the instruction itself
    Add  = 0x00,
    Sub  = 0x01,
    Mul  = 0x02,
    Div  = 0x03,
    Mod  = 0x04,

    And  = 0x05,
    Or   = 0x06,
    Xor  = 0x07,
    Not  = 0x08,

    Cmp  = 0x09,

    Push = 0x0a,
    Pop  = 0x0b,

    Mov  = 0x0c,

    //the rule set above no longer applies
    // jmp instructions
    Jmp = 0xe0,
    JNE = 0xe1,
    JE  = 0xe2,
    JL  = 0xe3,
    JB  = 0xe4,
    JEL = 0xe5,
    JEB = 0xe6,


    // calls
    Int  = 0xf1,//interrupt
    Call = 0xf2, 
    Rel  = 0xf3,// rel calls the emulator
    Ret  = 0xf4,// return
    End  = 0xfe,

    // Other
    #[default]
    FUL  = 0xff,
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

// unsafe code goes brrrrr
impl Into<u8> for Instruction{
    fn into(self) -> u8{
        unsafe{
            let ptr: *const Instruction= &self;

            *(ptr as *const u8)
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
    fn set_bytelen(&mut self, len: u8);

    fn step(&mut self) {
        let inst = self.get_inst();
        let val : u8 = inst.clone().into();
        let len = val / 16;

        // fucking 7th grade code
        let inst = if len <= 3 {
            self.set_bytelen(len);
            Instruction::from(0x0f & val)
        }
        else{inst};

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

            Cmp  => self.cmp(),

            Mov  => self.mov(),

            Call => self.call(),
            Ret  => self.ret(),
            End  => self.end(),

            _ => {},
        }
    }

}

pub struct Processor<const RAM_LEN: usize, const STACK_LEN: usize>{
    pub cmp_flag: CmpFlag, 

    // memory
    pub reg8 : Reg<u8, 4>,
    pub reg16: Reg<u16, 4>,
    pub reg32: Reg<u32, 4>,
    pub reg64: Reg<u64, 4>,
    pub stk: Stack<STACK_LEN>,
    pub ram:   RAM<u8, RAM_LEN>,

    pub inst_ptr: usize,
}

