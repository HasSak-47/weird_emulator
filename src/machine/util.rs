
macro_rules! impl_index{
    ($name: ty, $type: ty) => {
        impl<T, const SIZE: usize> Index<$type> for $name{
            type Output = T;
        
            fn index(&self, index: $type) -> &Self::Output {
                &self.data[index as usize]
            }
        }

        impl<T, const SIZE: usize> IndexMut<$type> for $name{
        
            fn index_mut(&mut self, index: $type) -> &mut T{
                &mut self.data[index as usize]
            }
        }

    };
}

pub(super) use impl_index;
