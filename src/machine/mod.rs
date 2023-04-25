mod machine;
mod memory_device;

#[repr(C)]
pub enum Precision{
    Byte  = 0,
    Word  = 1,
    DWord = 2,
    QWord = 3,
    OWord = 4,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Instructions{
    Add  = 0x01,
    Sub  = 0x02,
    Mul  = 0x03,
    Div  = 0x04,
    Mov  = 0x05,
    Push = 0x06,
    Pop  = 0x07,
    FAdd = 0x08,
    FSub = 0x09,
    FMul = 0x0a,
    FDiv = 0x0b,

    Call = 0x50,
    Ret  = 0x51,
    Int  = 0x52,
    Non  = 0xfe,
    Err  = 0xff,
}
