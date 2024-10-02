#[allow(dead_code)]
mod machine;
mod utils;

use machine::memory_device::MemoryDevice;



fn main() {
    let mut a = machine::drive::Drive::new(10);

    let _b : &mut i8 = a.get_mut(0);


    for b in a.blocks{
        println!("{b:?}");
    }
}
