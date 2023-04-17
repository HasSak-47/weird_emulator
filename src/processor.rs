
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
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
    Int  = 0x60,
    Call = 0x61,

    // Other
    #[default]
    End  = 0xf0,
    ERR  = 0xf1,
}
