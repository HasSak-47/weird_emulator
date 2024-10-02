pub mod machine;
pub mod memory_device;
pub mod drive;

mod debug;

#[repr(C)]
pub enum Precision{
    Byte  = 0x00, //  1 byte =   8bit
    Word  = 0x01, //  2 byte =  16bit
    DWord = 0x02, //  4 byte =  32bit
    QWord = 0x03, //  8 byte =  64bit
    OWord = 0x04, // 16 byte = 128bit
}

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
    And  = 0x0c,
    Or   = 0x0d,
    Xor  = 0x0e,
    Not  = 0x0f,

    // func
    Call = 0x50,
    Ret  = 0x51,
    Int  = 0x52,
    Non  = 0xfe,
    Err  = 0xff,
}
