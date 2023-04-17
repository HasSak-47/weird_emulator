use std::{fmt::{Display, Debug}, num::Wrapping};
use super::stack::Stack;
use super::ram::RAM;

#[repr(u8)]
#[allow(dead_code)]
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
    Rel  = 0x62,

    // Other
    #[default]
    End  = 0xf0,
    ERR  = 0xff,
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
pub struct Processor<T: 'static>{
    pub cmp_flag: CmpFlag, 

    // memory
    pub reg:      [T; 4],
    pub stk: Stack<T, 256>,
    pub ram:   RAM<T, 256>,

    pub inst_ptr: usize,
}

pub type Pu8 = Processor<u8>;

impl Pu8{
    pub fn new() -> Pu8{ Pu8 { cmp_flag: CmpFlag::Equal, reg: [0; 4], stk: Stack::new(), ram: RAM::new(), inst_ptr: 0, }}

    fn get_next(&self) -> u8{
        self.ram[self.inst_ptr + 1]
    }

    fn get_val(&self, modf: u8, val: u8) -> u8{
        let ind = val as usize;
        match modf{
            0x0 => self.reg[ind],
            0x1 => self.ram[ind],
            0x2 => val as u8,
            0x3 => self.ram[self.reg[ind] as usize],
            0x4 => self.ram[self.ram[ind] as usize],

            _ => {panic!("unknown src/dst flag")}
        }
    }

    fn get_operant(&mut self) -> u8 {
        let mods = self.ram[self.inst_ptr + 1];
        let dst  = self.ram[self.inst_ptr + 2];
        let dst_mod = mods % 16;

        self.get_val(dst_mod, dst)
    }


    fn get_operants(&self) -> (u8, u8){
        let mods = self.ram[self.inst_ptr + 1];
        let dst  = self.ram[self.inst_ptr + 2];
        let src  = self.ram[self.inst_ptr + 3];

        let dst_mod = mods % 16;
        let src_mod = mods / 16;

        let src_val = self.get_val(src_mod, src);
        let dst_val = self.get_val(dst_mod, dst);
        // println!("{dst_mod:x} {dst:x} {dst_val:x}");

        (dst_val, src_val)
    }

    fn get_oper(&mut self) -> (u8, u8, &mut u8) {
        let operants = self.get_operants();
        let dst_mod = self.ram[self.inst_ptr + 1] % 16;

        let ind = self.ram[self.inst_ptr + 2];
        let dst_ref = match dst_mod{
            0x00 => { &mut self.reg[ind as usize]},
            0x01 => { &mut self.ram[ind] },
            _ => panic!("unknown dst ref"),
        };

        (operants.0, operants.1, dst_ref)
    }


    fn operation(&mut self, oper: fn(Wrapping<u8>, Wrapping<u8>) -> u8) {
        let op = self.get_oper();
        // println!("{op:x?}");
        *op.2 = oper(Wrapping(op.0), Wrapping(op.1));

        self.inst_ptr += 4;
    }

    fn add(&mut self) {self.operation(|a, b| (a + b).0);}
    fn sub(&mut self) {self.operation(|a, b| (a - b).0);}
    fn mul(&mut self) {self.operation(|a, b| (a * b).0);}
    fn div(&mut self) {self.operation(|a, b| (a / b).0);}

    fn and(&mut self) {self.operation(|a, b| (a & b).0);}
    fn or (&mut self) {self.operation(|a, b| (a | b).0);}
    fn not(&mut self) {}

    fn jmp(&mut self){
        let operant = self.get_next();
        self.inst_ptr = operant as usize;
    }

    fn con_jump(&mut self, flag: CmpFlag) {
        let operant = self.get_next();
        if self.cmp_flag == flag{
            self.inst_ptr = operant as usize;
            return;
        }
        self.inst_ptr += 2;
    }

    fn jne(&mut self) { self.con_jump(CmpFlag::Different) }
    fn  je(&mut self) { self.con_jump(CmpFlag::Equal) }
    fn  jl(&mut self) { self.con_jump(CmpFlag::Lesser) }

    fn push(&mut self) {
        let operant = self.get_operant();
        self.stk.push(operant)
    }
    fn  pop(&mut self) {
        // let operant = self.get_operant();
        // self.stc[self.stc_ptr] = operant;
        // self.stc_ptr += 1;
    }

    fn cmp(&mut self) {
        let operants = self.get_operants();
        self.cmp_flag =
            if operants.0 == operants.1 {CmpFlag::Equal}
            else if operants.0 < operants.1 {CmpFlag::Lesser}
            else {CmpFlag::Different};
        self.inst_ptr += 4;
    }

    fn mov(&mut self) {
        let operants = self.get_oper();
        *operants.2 = operants.1;
        self.inst_ptr += 4;

    }

    fn  int(&mut self) {}
    fn call(&mut self) {}
    fn  err(&mut self) {}
    fn end(&mut self) {}

    pub fn step(&mut self) {
        let inst = self.ram[self.inst_ptr];
        use Instruction::*;
        match Instruction::from(inst){
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



            _ => {},
        }
    }
}

impl Display for Pu8{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
"reg: 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
cmp: {}
index: 0x{:x}
next instruction: {} [x{:x}]
",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3],
            self.cmp_flag,
            self.inst_ptr,
            Instruction::from(self.ram[self.inst_ptr]), self.ram[self.inst_ptr],
        )
    }
}

impl Debug for Pu8{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}\nram: {:?}",self, self.ram,)
    }
}
