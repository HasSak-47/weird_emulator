use super::drive::Block;

use std::fmt::Debug;

impl Debug for Block{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for i in 0..16{
            write!(f, "{i:02x} ")?;
        }
        write!(f, "\n00|")?;

        let mut i = 0;
        let mut j = 0;
        for a in self.0{
            if i == 16 { write!(f, "\n{j:02x}|")?; i = 0; j+=1;}
            write!(f, "{a:02x} ")?;
            i += 1;
        }
        Ok(())
    }
}
