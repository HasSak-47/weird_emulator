use std::fmt::Debug;
use std::ops::{IndexMut, Index};
use super::util::impl_index;

#[derive(Clone, Copy)]
pub struct RAM<T, const SIZE: usize>{
    data: [T; SIZE], 
}

#[allow(dead_code)]
impl<T, const SIZE: usize> RAM<T, SIZE>
where T: Default + Copy,{
    pub fn new() -> Self{Self{data: [T::default(); SIZE]}}

    pub fn get0(&self, ptr: usize) -> &T{
        &self.data[ptr]
    }
    pub fn get1(&self, ptr: usize) -> (&T, &T){
        (&self.data[ptr], &self.data[ptr + 1])
    }
    pub fn get2(&self, ptr: usize) -> (&T, &T, &T){
        (&self.data[ptr], &self.data[ptr + 1], &self.data[ptr + 2])
    }
    pub fn get3(&self, ptr: usize) -> (&T, &T, &T, &T){
        (&self.data[ptr], &self.data[ptr + 1], &self.data[ptr + 2], &self.data[ptr + 3])
    }
}

impl<T, const SIZE: usize> Debug for RAM<T, SIZE>
where 
    T: std::fmt::LowerHex
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print x line
        for i in 0..8 { write!(f, "0{:x} ", i)?; }
        write!(f, " ")?;
        for i in 8..16 { write!(f, "0{:x} ", i)?; }

        write!(f, "\n")?;

        let t = self.data.len() / 16;

        for j in 0..t{
            write!(f, "{:4x}:  ", j * 16)?;

            // print x line
            for i in 0..8 { write!(f, "{:2x} ", self.data[j * 16 + i])?; }
            write!(f, " ")?;
            for i in 8..16{ write!(f, "{:2x} ", self.data[j * 16 + i])?; }

            write!(f, "\n")?;
        }
        Ok(())
    }

}

impl_index!(RAM, u8);
impl_index!(RAM, u16);
impl_index!(RAM, u32);
impl_index!(RAM, u64);
impl_index!(RAM, u128);
impl_index!(RAM, usize);

