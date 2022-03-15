

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
    pub seq : Vec<isize>,
    pub mode: usize
}

impl MLData {
    
    pub fn doleft(&mut self, i : usize)
    {
        //i == 0 = top row; 1 = bottom row
        let c : usize = i*12;
        let d=self.posit[c];
        self.posit[c]=self.posit[c+1];
        self.posit[c+1]=self.posit[c+2];
        self.posit[c+2]=self.posit[c+3];
        self.posit[c+3]=d;
        if self.blank_y==i*3 { self.blank_x=(self.blank_x-1) & 3; }
    }

    pub fn doright(&mut self, i : usize)
    {
        //i == 0 = top row; 1 = bottom row
        let c : usize = i*12;
        let d=self.posit[c+3];
        self.posit[c+3]=self.posit[c+2];
        self.posit[c+2]=self.posit[c+1];
        self.posit[c+1]=self.posit[c];
        self.posit[c]=d;
        if self.blank_y==i*3 {self.blank_x = (self.blank_x+1) & 3; }
    }

    pub fn domove(&mut self,y : isize)
    {
        //try up/down move
        let mut c : usize = self.blank_y * 4 + self.blank_x;
        
        while util::compare(self.blank_y, y) == -1  //self.blank_y < y
        {
            self.posit[c] = self.posit[c + 4];
            c+=4;
            self.blank_y+=1;
        }
        while  util::compare(self.blank_y, y) == 1  //self.blank_y > y
        {
            self.posit[c]=self.posit[c-4];
            c-=4;
            self.blank_y-=1;
        }
        self.posit[c] = 12;
    }
    
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

    pub fn push(&mut self, i : &[isize])
    {
        for m_ptr in i
        {
            let mut m = *m_ptr;
            if m != 0
            {
                if m==4 || m==5 { self.doleft( util::cvt_int(m-4)  ); }
                else if m==6 || m==7 { self.doright(util::cvt_int(m-6)); }
                else { self.domove( util::add(self.blank_y , m) ); }
                if self.seq.len() > 0
                {
                    let last_ele = *self.seq.last().unwrap();
                    if m >= 4 && last_ele == (m^2) { self.seq.pop(); }
                    else if m<4 && last_ele < 4 {
                        m += last_ele;
                        let last_idx = self.seq.len() - 1;
                        util::change_value(&mut self.seq, last_idx, m);
                        if m == 0 { self.seq.pop(); }
                    }
                    else {
                        self.seq.push(m);    
                    }
                } else {
                    self.seq.push(m);
                }
            }
        }
    }
}

mod util
{
    use std::convert::TryFrom;

    //convert signed int to uint asserting that it is possible
    pub fn cvt_int(v : isize) -> usize
    {
        return usize::try_from(v).ok().unwrap();
    }

    pub fn add(x : usize, y : isize) -> isize
    {
        let x2 = isize::try_from(x).ok().unwrap();
        return x2 + y;
    }

    pub fn compare(x : usize, y : isize) -> isize
    {
        let x2 = isize::try_from(x).ok().unwrap();
        if x2 < y { return -1; }
        else if x2 > y {return 1; }
        else { return 0; }
    }

    pub fn change_value(list : &mut Vec<isize>, idx : usize, value : isize)
    {
        list[idx] = value;
    }
}