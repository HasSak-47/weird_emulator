use std::fmt::Debug;
use std::ops::{IndexMut, Index};

#[derive(Clone, Debug)]
pub struct RAM<T, const SIZE: usize>{
    data: [T; SIZE], 
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

