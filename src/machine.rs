use std::fmt::{Display, Debug};
use std::num::Wrapping;

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CmpFlag{
    #[default]
    Diferent = 0,

    Equal    = 1,
    Lesser   = 2,
}

impl std::fmt::Display for CmpFlag{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Equal    => "equal",
            Self::Lesser   => "less",
            Self::Diferent => "diff",
        })
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Instruction{
    // arithmetic instructions
    Add = 0x00,
    Sub = 0x01,
    Mul = 0x02,
    Div = 0x03,
    // bitwise instructions
    And = 0x10,
    Or  = 0x11,
    Not = 0x12,
    // jmp instructions
    Jmp = 0x20,
    JNE = 0x21,
    JE  = 0x22,
    JL  = 0x23,
    // stack instructions
    Push = 0x30,
    Pop  = 0x31,
    // cmp instructions
    Cmp  = 0x40,
    // move instructions
    Mov  = 0x50,
    // calls
    Int  = 0x60,
    Call = 0x61,

    // Other
    #[default]
    End  = 0xf0,
    ERR  = 0xf1,
}

impl Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        write!(f, "{}", match self{
            Add  => "Add", Sub  => "Sub", Mul  => "Mul", Div  => "Div",
            And  => "And", Or   => "Or ", Not  => "Not",
            Jmp  => "Jmp", JNE  => "JNE", JE   => "JE ", JL   => "JL ",
            Push => "Push", Pop  => "Pop",
            Cmp  => "Cmp",
            Mov  => "Mov",
            Int  => "Int", Call => "Call",
            End  => "End", ERR  => "ERR",
        })
    }
}

impl From<u8> for Instruction{
    fn from(value: u8) -> Self {
        use Instruction::*;
        match value {
            0x00 => Add , 0x01 => Sub , 0x02 => Mul , 0x03 => Div ,
            0x10 => And , 0x11 => Or  , 0x12 => Not ,
            0x20 => Jmp , 0x21 => JNE , 0x22 => JE  , 0x23 => JL  ,
            0x30 => Push, 0x31 => Pop ,
            0x40 => Cmp ,
            0x50 => Mov ,
            0x60 => Int , 0x61 => Call,
            0xf0 => End , _    => ERR ,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Machine{
    pub cmp_flag: CmpFlag, 

    // memory
    pub reg: [u8; 4],
    pub stc: [u8; 256],
    pub mem: [u8; 256],

    pub stc_ptr: usize,
    pub mem_ptr: usize,
}

impl Default for Machine{
    fn default() -> Self {
        Machine::new()
    }
}


impl Machine{
    pub const fn new() -> Machine{ Machine { cmp_flag: CmpFlag::Equal, reg: [0; 4], stc: [0; 256], mem: [0; 256],  stc_ptr: 0, mem_ptr: 0, }}

    const FUNCTIONS : [fn(&mut Machine); 17] = [
        Self::add, Self::sub, Self::mul, Self::div,
        Self::and, Self::or , Self::not,
        Self::jmp, Self::je, Self::jl,
        Self::push, Self::pop,
        Self::cmp,
        Self::mov,
        Self::int, Self::call,
        Self::err,
    ];

    fn get_next(&mut self) -> u8{
        self.mem[self.mem_ptr + 1]
    }

    fn get_val(&mut self, modf: u8, val: u8) -> u8{
        let ind = val as usize;
        match modf{
            0x0 => self.reg[ind],
            0x1 => self.mem[ind],
            0x2 => val as u8,
            0x3 => self.mem[self.reg[ind] as usize],
            0x4 => self.mem[self.mem[ind] as usize],

            _ => {panic!("unknown src/dst flag")}
        }
    }

    fn get_operant(&mut self) -> u8 {
        let mods = self.mem[self.mem_ptr + 1];
        let dst  = self.mem[self.mem_ptr + 2];
        let dst_mod = mods % 16;

        self.get_val(dst_mod, dst)
    }


    fn get_operants(&mut self) -> (u8, u8){
        let mods = self.mem[self.mem_ptr + 1];
        let dst  = self.mem[self.mem_ptr + 2];
        let src  = self.mem[self.mem_ptr + 3];

        let dst_mod = mods % 16;
        let src_mod = mods / 16;

        let src_val = self.get_val(src_mod, src);
        let dst_val = self.get_val(dst_mod, dst);
        // println!("{dst_mod:x} {dst:x} {dst_val:x}");

        (dst_val, src_val)
    }

    fn get_oper(&mut self) -> (u8, u8, &mut u8) {
        let operants = self.get_operants();
        let dst_mod = self.mem[self.mem_ptr + 1] % 16;

        let dst_ref = match dst_mod{
            0x00 => {
                &mut self.reg[self.mem[self.mem_ptr + 2] as usize]
            },
            0x01 => {&mut self.mem[self.mem[self.mem_ptr + 2] as usize]},
            _ => panic!("unknown dst ref"),
        };

        (operants.0, operants.1, dst_ref)
    }


    fn operation(&mut self, oper: fn(Wrapping<u8>, Wrapping<u8>) -> u8) {
        let op = self.get_oper();
        // println!("{op:x?}");
        *op.2 = oper(Wrapping(op.0), Wrapping(op.1));

        self.mem_ptr += 4;
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
        self.mem_ptr = operant as usize;
    }

    fn con_jump(&mut self, flag: CmpFlag) {
        let operant = self.get_next();
        if self.cmp_flag == flag{
            self.mem_ptr = operant as usize;
            return;
        }
        self.mem_ptr += 2;
    }

    fn jne(&mut self) { self.con_jump(CmpFlag::Diferent) }
    fn  je(&mut self) { self.con_jump(CmpFlag::Equal) }
    fn  jl(&mut self) { self.con_jump(CmpFlag::Lesser) }

    fn push(&mut self) {
        let operant = self.get_operant();
        self.stc[self.stc_ptr] = operant;
        self.stc_ptr += 1;
    }
    fn  pop(&mut self) {
        let operant = self.get_operant();
        self.stc[self.stc_ptr] = operant;
        self.stc_ptr += 1;
    }

    fn cmp(&mut self) {
        let operants = self.get_operants();
        self.cmp_flag =
            if operants.0 == operants.1 {CmpFlag::Equal}
            else if operants.0 < operants.1 {CmpFlag::Lesser}
            else {CmpFlag::Diferent};
        self.mem_ptr += 4;
    }

    fn mov(&mut self) {
        let operants = self.get_oper();
        *operants.2 = operants.1;
        self.mem_ptr += 4;

    }

    fn  int(&mut self) {}
    fn call(&mut self) {}
    fn  err(&mut self) {}
    fn end(&mut self) {}

    pub fn step(&mut self) {
        let inst = self.mem[self.mem_ptr];
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

            Int  => self.int(),
            Call => self.call(),

            End => self.end(),



            _ => {},
        }
    }
}

impl Display for Machine{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
"reg: 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
cmp: {}
mem_ptr: x{:x}
stc_ptr: x{:x}
next instruction: {} [x{:x}]
",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3],
            self.cmp_flag,
            self.mem_ptr,
            self.stc_ptr,
            Instruction::from(self.mem[self.mem_ptr]),
            self.mem[self.mem_ptr],
        )
    }
}

impl Debug for Machine{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n     ", self)?;

        // print x line
        for i in 0..8 { write!(f, "0{:x} ", i)?; }
        write!(f, " ")?;
        for i in 8..16 { write!(f, "0{:x} ", i)?; }

        write!(f, "\n")?;

        for j in 0..16{
            write!(f, "{:02x}:  ", j)?;

            // print x line
            for i in 0..8 { write!(f, "{:02x} ", self.mem[j * 16 + i])?; }
            write!(f, " ")?;
            for i in 8..16{ write!(f, "{:02x} ", self.mem[j * 16 + i])?; }

            write!(f, "\n")?;
        }
        Ok(())
    }
}
