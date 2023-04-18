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
}
    
// more unsafe brrrrr
impl<T, const SIZE: usize> RAM<T, SIZE>{
    pub fn get_as<T2, U: Into<usize>>(&self, index: U) -> &T2{
        let p : *const T= &self.data[index.into()];
        unsafe{ &*(p as *const T2) }
    }
    pub fn get_as_mut<T2, U: Into<usize>>(&mut self, index: U) -> &T2{
        let p : *mut T= &mut self.data[index.into()];
        unsafe{ &*(p as *mut T2) }
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

