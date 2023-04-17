mod ram;
mod stack;
mod processor;

use processor::Pu8;

fn main() {
    let mut machine = Pu8::new();

    // how the asm should kinda be
    let instructions = [
        // mov val 0xf0 to reg 0x00
        0x50, 0x20, 0x00, 0xf0,
        // cmp adr at reg 0x00 and val 0x00
        0x40, 0x23, 0x00, 0x00,
        // add from val 0x01 to reg 0x00
        0x00, 0x20, 0x00, 0x01,
        // jne to 0x04
        0x21, 0x04,

        0x01, 0x20, 0x00, 0xf0,
    
        0xf0, // End
    ];

    //starts at 0xf0
    let memory = [
        // fake string
        0x01, 0x02, 0x03, 0x4, 0x00,
    ];

    for i in 0..instructions.len() {
        machine.ram[i] = instructions[i];
    }

    for i in 0..memory.len() {
        let p = i + 0xf0;
        println!("{} {} {}", p, 0xf0, 0x0f);
        machine.ram[p] = memory[i];
    }

    while machine.ram[machine.inst_ptr] != 0xf0{
        // println!("{:?}", machine);
        // let mut buffer = String::new();
        // std::io::stdin().read_line(&mut buffer).unwrap();

        machine.step();
    }
    println!("{:?}", machine);
}
