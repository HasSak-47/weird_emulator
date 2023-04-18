use std::ops::{Index, IndexMut};
use super::util::impl_index;

pub struct Reg<T, const TOTAL: usize>{
    pub data: [T; TOTAL],
}

impl<T: Default + Copy, const TOTAL: usize> Reg<T, TOTAL>{
    pub fn new() -> Self{ Reg {data: [T::default(); TOTAL]}}
}
impl_index!(Reg, u8);
impl_index!(Reg, u16);
impl_index!(Reg, u32);
impl_index!(Reg, u64);
impl_index!(Reg, u128);
impl_index!(Reg, usize);

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
