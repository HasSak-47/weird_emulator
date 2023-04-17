use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Stack<T, const SIZE: usize>{
    data: [T; SIZE], 
    index: usize,
}

impl<T, const SIZE: usize> Stack<T, SIZE>
where T: Default + Copy,{
    pub fn new() -> Self{Self{data: [T::default(); SIZE], index: 0}}
}

impl<T, const SIZE: usize> Stack<T, SIZE>
where 
    T : Clone,
{
    pub fn push(&mut self, t: T){
        self.data[self.index] = t;
        self.index += 1;
    }
    pub fn pop(&mut self) -> T{
        let t = self.data[self.index].clone();
        self.index -= 1;

        t
    }

} 
