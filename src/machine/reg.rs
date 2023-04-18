use std::ops::{Index, IndexMut};
use super::util::impl_index;

pub struct Reg<T, const SIZE: usize>{
    pub data: [T; SIZE],
}

impl<T: Default + Copy, const SIZE: usize> Reg<T, SIZE>{
    pub fn new() -> Self{ Reg {data: [T::default(); SIZE]}}
}
impl_index!(Reg<T, SIZE>, u8);
impl_index!(Reg<T, SIZE>, u16);
impl_index!(Reg<T, SIZE>, u32);
impl_index!(Reg<T, SIZE>, u64);
impl_index!(Reg<T, SIZE>, u128);
impl_index!(Reg<T, SIZE>, usize);
impl_index!(Reg<T, SIZE>, i8);
impl_index!(Reg<T, SIZE>, i16);
impl_index!(Reg<T, SIZE>, i32);
impl_index!(Reg<T, SIZE>, i64);
impl_index!(Reg<T, SIZE>, i128);
impl_index!(Reg<T, SIZE>, isize);
/*
macro_rules! create_reg_deref {
    ($name: ident, $($field: ident), *) => {
        #[repr(C)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub struct $name<T>{
            $(pub $field: T,)*
        }
    };
}

create_reg_deref!(__Reg1, a);
create_reg_deref!(__Reg2, a, b);
create_reg_deref!(__Reg3, a, b, c);
create_reg_deref!(__Reg4, a, b, c, d);
*/
