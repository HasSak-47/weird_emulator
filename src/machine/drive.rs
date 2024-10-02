use crate::utils::Index;

pub struct Block(pub [u8; 256]);

pub struct Drive{
    pub blocks: Vec<Box<Block>>,
    pub len: usize,
}

impl Drive{
    pub fn new(len: usize) -> Drive{
        let mut n_self = Drive {
            blocks : Vec::new(),
            len,
        };
        for _ in 0..len{
            n_self.blocks.push( Box::new(Block([0; 256])));
        }

        n_self
    }

    pub fn get_at_mut<In: Index>(&mut self, index: In) -> &mut u8{
        let index = index.to_usize();
        
        let block_index = index / 256;
        let block_adrss = index % 256;

        &mut(self.blocks[block_index]).0[block_adrss]
    }

    pub fn get_at<In: Index>(&self, index: In) -> &u8{
        let index = index.to_usize();
        
        let block_index = index / 256;
        let block_adrss = index % 256;

        &(self.blocks[block_index]).0[block_adrss]
    }
}

macro_rules!  ImplDrive {
    ($type : tt, $div: literal) => {
        
impl MemoryDevice for Drive {
    fn _get<'a, $type>(&'a self, indx: usize) -> &'a $type {
        let indx = indx / $div;
        unsafe{
            let a : *const u8 = self.get_at(indx);
            &*(a as *const $type )
        }

    } 

    fn _get_mut<'a, $type>(&'a mut self, indx: usize) -> &'a mut $type {
        let indx = indx / $div;
        unsafe{
            let a : *mut u8 = self.get_at_mut(indx);
            &mut *(a as *mut $type)
        }
    } 
}
    };
}

ImplDrive!(u8  , 1);
