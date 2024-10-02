use crate::utils::Index;

pub trait MemoryDevice {
    fn get_mut<'a, T, IndxTrait: Index>(&'a mut self, indx: IndxTrait) -> &'a mut T {
        self._get_mut(indx.to_usize())
    }
    fn get    <'a, T, IndxTrait: Index>(&'a     self, indx: IndxTrait) -> &'a T{
        self._get(indx.to_usize())
    }

    fn _get_mut<'a, T>(&'a mut self, indx: usize) -> &'a mut T;
    fn _get<'a, T>(&'a self, indx: usize) -> &'a T;
}

