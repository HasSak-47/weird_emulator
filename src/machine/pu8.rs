use super::{
    processor::*,
    stack::*,
    ram::*,
    reg::*,
};

use std::{
    fmt::{Display, Debug},
    num::Wrapping,
};
pub type Pu8 = Processor<256, 256>;

#[allow(dead_code)]
impl Pu8{
    pub fn new() -> Pu8 { Pu8 {
        cmp_flag: CmpFlag::Equal,
        reg8:  Reg::new(),
        reg16: Reg::new(),
        reg32: Reg::new(),
        reg64: Reg::new(),
        stk: Stack::new(),
        ram: RAM::new(),
        inst_ptr: 0,
    }}

    fn get_next(&self) -> u8{
        self.ram[self.inst_ptr + 1]
    }

    fn get_val(&self, modf: u8, val: u8) -> u8{
        let ind = val as usize;
        match modf{
            0x0 => self.reg8[ind],
            0x1 => self.ram[ind],
            0x2 => val,
            0x3 => self.ram[self.reg8[ind]],
            0x4 => self.ram[self.ram[ind]],

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
            0x00 => { &mut self.reg8[ind as usize]},
            0x01 => { &mut self.ram[ind] },
            _ => panic!("unknown dst ref"),
        };

        (operants.0, operants.1, dst_ref)
    }


    fn operation(&mut self, oper: fn(Wrapping<u8>, Wrapping<u8>) -> u8) {
        let op = self.get_oper();
        *op.2 = oper(Wrapping(op.0), Wrapping(op.1));

        self.inst_ptr += 4;
    }

    fn con_jump(&mut self, flag: CmpFlag) {
        let operant = self.get_next();
        if self.cmp_flag == flag{
            self.inst_ptr = operant as usize;
            return;
        }
        self.inst_ptr += 2;
    }
}

impl Process for Pu8{
    fn add(&mut self) {self.operation(|a, b| (a + b).0);}
    fn sub(&mut self) {self.operation(|a, b| (a - b).0);}
    fn mul(&mut self) {self.operation(|a, b| (a * b).0);}
    fn div(&mut self) {self.operation(|a, b| (a / b).0);}
    fn mdi(&mut self) {self.operation(|a, b| (a % b).0);}

    fn addf(&mut self) {}
    fn subf(&mut self) {}
    fn mulf(&mut self) {}
    fn divf(&mut self) {}

    fn and(&mut self) {self.operation(|a, b| (a & b).0);}
    fn or (&mut self) {self.operation(|a, b| (a | b).0);}
    fn not(&mut self) {}

    fn jmp(&mut self){
        let operant = self.get_next();
        self.inst_ptr = operant as usize;
    }

    // this is fucked
    // will fix later
    fn jne(&mut self) { self.con_jump(CmpFlag::Different) }
    fn  je(&mut self) { self.con_jump(CmpFlag::Equal) }
    fn  jl(&mut self) { self.con_jump(CmpFlag::Lesser) }
    fn  jb(&mut self) { self.con_jump(CmpFlag::Bigger) }
    fn jel(&mut self) { self.con_jump(CmpFlag::Lesser) }
    fn jeb(&mut self) { self.con_jump(CmpFlag::Equal) }

    fn push(&mut self) { }
    fn  pop(&mut self) { }

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

    fn  ker(&mut self){}
    fn call(&mut self){}
    fn  rel(&mut self){}
    fn  ret(&mut self){}

    fn  err(&mut self){}
    fn  end(&mut self){}

    fn get_inst(&self) -> Instruction{
        Instruction::from(self.ram[self.inst_ptr])
    }

    fn set_bytelen(&mut self, len: u8) {
        
    }
}

impl Display for Pu8{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
"reg8: 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
cmp: {}
index: 0x{:x}
next instruction: {} [x{:x}]
",
            self.reg8[0], self.reg8[1], self.reg8[2], self.reg8[3],
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
