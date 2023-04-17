use std::fmt::Debug;
use std::ops::{IndexMut, Index};

#[derive(Clone, Copy)]
pub struct RAM<T, const SIZE: usize>{
    data: [T; SIZE], 
}

impl<T, const SIZE: usize> RAM<T, SIZE>
where T: Default + Copy,{
    pub fn new() -> Self{Self{data: [T::default(); SIZE]}}
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

macro_rules! impl_index_uint {
    ($type: ident) => {
        impl<T, const SIZE: usize> Index<$type> for RAM<T, SIZE>{
            type Output = T;
        
            fn index(&self, index: $type) -> &Self::Output {
                &self.data[index as usize]
            }
        }

        impl<T, const SIZE: usize> IndexMut<$type> for RAM<T, SIZE>{
        
            fn index_mut(&mut self, index: $type) -> &mut T{
                &mut self.data[index as usize]
            }
        }

    };
}

impl_index_uint!(u8);
impl_index_uint!(u16);
impl_index_uint!(u32);
impl_index_uint!(u64);
impl_index_uint!(u128);
impl_index_uint!(usize);

