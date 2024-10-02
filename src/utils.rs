pub trait Index{ fn to_usize(self) -> usize; }

macro_rules! to_usize_impl {
    ($t: ty, $bl: block, $self : tt) => {
        impl Index for $t {
            fn to_usize($self) -> usize { $bl }
        }
    };
}

to_usize_impl!(usize, {self}, self);
to_usize_impl!(u64, {self as usize}, self);
to_usize_impl!(u32, {self as usize}, self);
to_usize_impl!(u16, {self as usize}, self);
to_usize_impl!(u8 , {self as usize}, self);
to_usize_impl!(i64, {self as usize}, self);
to_usize_impl!(i32, {self as usize}, self);
to_usize_impl!(i16, {self as usize}, self);
to_usize_impl!(i8 , {self as usize}, self);
