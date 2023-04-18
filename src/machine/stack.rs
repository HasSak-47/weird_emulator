use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Stack<const SIZE: usize>{
    data: [u64; SIZE], 
    index: usize,
}

impl<const SIZE: usize> Stack<SIZE>{
    pub fn new() -> Self{Self{data: [0; SIZE], index: 0}}
}

impl<const SIZE: usize> Stack<SIZE> {
    pub fn push(&mut self, t: u64){
        self.data[self.index] = t;
        self.index += 1;
    }
    pub fn pop(&mut self) -> u64{
        let t = self.data[self.index].clone();
        self.index -= 1;

        t
    }

} 
