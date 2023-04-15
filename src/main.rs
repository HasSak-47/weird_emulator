#[allow(dead_code)]
mod machine;
use machine::*;

fn main() {
    let mut machine = Machine::new();

    // how the asm should kinda be
    let memory = [
        // mov val 0x11 to reg 0x00
        0x50, 0x20, 0x00, 0x11,
        // mov val 0x11 to reg 0x01
        0x50, 0x20, 0x01, 0x11,
        // cmp adr at reg 0x00 and val 0x00
        0x40, 0x23, 0x00, 0x00,
        // je to 0x10
        0x22, 0x10,
        // add from val 0x01 to reg 0x00
        0x00, 0x20, 0x00, 0x01,
        // jmp to 0x04
        0x20, 0x04,
    
        0xf0, // End
        // fake string
        0x01, 0x02, 0x03, 0x4, 0x00,
    ];

    for val in 0..memory.len() {
        machine.mem[val] = memory[val];
    }

    while machine.mem[machine.mem_ptr] != 0xf0{
        println!("{:?}", machine);
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        machine.step();
    }
}
