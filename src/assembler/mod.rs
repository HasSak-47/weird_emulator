// fuck yeahhh
// parsing text
// I wanna dieeee
use super::machine::processor::Instruction;
use std::fs::File;

pub struct Token {}
pub struct Statement{
    sts: Vec<Token>,
}

fn proto_statements<S : AsRef<str>>(name: S) -> Vec<Vec<String>>{
    let mut r = Vec::new(); 
    let mut f = File::open(name.as_ref()).unwrap();

    r
}
