
#[derive(Clone, Copy, Default)]
pub enum CmpFlag{
    Equal,
    Small,
    Bigger,
    #[default]
    Diferent,
}

#[repr(u8)]
pub enum Instructions{
    Int  = 0x00,
    Call = 0x01,
    Jmp  = 0x02,
    Je   = 0x03,
    Jne  = 0x04,
    Jb   = 0x05,
    Js   = 0x06,
    Ret  = 0x07,

    Add  = 0x11,
    Sub  = 0x12,
    Mul  = 0x13,
    Div  = 0x14,

    Pop  = 0x21,
    Push = 0x22,

    End  = 0xf0,
    ERR  = 0xff,
}

pub enum PushOpts{
    Ra = 0x0a,
    Rb = 0x0b,
    Rc = 0x0c,
    Rd = 0x0d,

    Rm = 0x00,
    Rp = 0x01,
}

pub enum PopOpts{
    Ra = 0x0a,
    Rb = 0x0b,
    Rc = 0x0c,
    Rd = 0x0d,

    Rn = 0x00,
}

pub enum AddOpts{

}

impl From<u8> for Instructions{
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Int ,
            0x01 => Self::Call,
            0x02 => Self::Jmp ,
            0x03 => Self::Je  ,
            0x04 => Self::Jne ,
            0x05 => Self::Jb  ,
            0x06 => Self::Js  ,
            0x07 => Self::Ret ,
            0x11 => Self::Add ,
            0x12 => Self::Sub ,
            0x13 => Self::Mul ,
            0x14 => Self::Div ,
            0x21 => Self::Pop ,
            0x22 => Self::Push,
            0xf0 => Self::End ,
            _ => Self::ERR ,

        }
    }
}


#[derive(Clone, Copy)]
pub struct Machine{
    pub cmp_flag: CmpFlag, 
    pub reg     : [u8; 4],

    pub stack  : [u8; 256],
    pub memory : [u8; 256],
    pub stc_ptr: usize,
    pub mem_ptr: usize,
}

impl Machine{
    fn new() -> Self{
        Machine{
            cmp_flag: CmpFlag::Diferent,
            reg  : [0, 0, 0, 0],
            stack: [0; 256],
            memory: [0; 256],
            mem_ptr: 0,
            stc_ptr: 0,
        }
    }

    fn step(&mut self) -> &mut Self{
        let inst : Instructions = self.memory[self.mem_ptr].into();
        match inst{
            Instructions::Int  => {},
            Instructions::Call => {},
            Instructions::Jmp  => {},
            Instructions::Je   => {},
            Instructions::Jne  => {},
            Instructions::Jb   => {},
            Instructions::Js   => {},
            Instructions::Ret  => {},
            Instructions::Add  => {},
            Instructions::Sub  => {},
            Instructions::Mul  => {},
            Instructions::Div  => {},
            Instructions::Pop  => {},
            Instructions::Push => {},
            Instructions::End  => {},
            Instructions::ERR  => {},
        }
        self.mem_ptr += 1;
        self
    }
}
