    pub const MODE_NORMAL: usize = 0;
    pub const MODE_SOLVING_SCRAMBLED: usize = 1;
    pub const MODE_EDIT: usize = 2;
    pub const MODE_SOLVING: usize = 3;

    pub const SIZE_TOTAL : usize = 16;
    pub const SIZE_COLUMN : usize = 4;

pub struct MLData {
    pub posit: [usize; 16],
    pub blank_x: usize,
    pub blank_y: usize,
    pub mode: usize
}

impl MLData {
    pub fn solved(&self) -> bool
    {
        for i in 0..SIZE_COLUMN
        {
            let mut c = i;
            if self.posit[c] == 12 {c+=4;}
            if self.posit[c] > 3 {return false;}
            let mut d : usize = i + 12;
            if self.posit[d] == 12 {d-=4;}
            if self.posit[d] != self.posit[c]+8 {return false;}
            d -= 4;
            while c!=d
            {
                if self.posit[d]!=self.posit[c]+4 {return false;}
                d -= 4;
            }
        }
        return true;
    }
}