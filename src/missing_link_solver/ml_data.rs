

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

    fn push_single(&mut self, i : isize)
    {
        self.push(&[i]);
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

    pub fn find(&self, tl : usize) -> usize
    {
        for aidx in 0..SIZE_TOTAL
        {
            if self.posit[aidx] == tl { return aidx; }
        }
        return SIZE_TOTAL - 1; //that doesn't seem right; not handling if doesn't find what it's looking for
    }

    pub fn solvetile(&mut self, tl : usize, cl : usize)
    {
        //Solves tile tl by putting it at top of column cl, and if not last piece then moving down the column
        
        //gap to top. Assumes gap is not below partially solved column
        self.push_single(util::negative_of(self.blank_y));

        let mut tx_idx : usize = 0;
        for ty in 0..SIZE_COLUMN
        {
            for tx in cl+1..SIZE_COLUMN
            {
                tx_idx = tx;
                if self.posit[ty*4+tx]==tl { break; }
            }
            if tx_idx < 4 {break;}
        }
        let mut ty_idx : usize = 0;
        if tx_idx>4
        {
            for tx in (0..=cl).rev()
            {
                for ty in (0..=3).rev()
                {
                    ty_idx = ty;
                    if self.posit[ty*4+tx]==tl {break;}
                }
                if ty_idx >= 0 {break;} //this is really suspect... see orignal codde... this expression is always true
            }
        }

        if ty_idx==0 && cl==0 
        {
            while tx_idx > 0
            {
                tx_idx -= 1;
                self.push_single(4);
            }
        } else {
            if ty_idx==0
            {
                //Move tile down to second row
                if cl==self.blank_x
                { 
                    self.push(&[6,1,4]); 
                } else {
                    self.push_single(1); //gap tp second row
                }
                //move tile above gap
                let mut a: isize = util::subtract(tx_idx,self.blank_x);
                let mut b = a;
                while a>0 {self.push_single(4); a -= 1;}
                while a<0 {self.push_single(6); a +=1;}
                //tile down
                self.push(&[-1]);
                //Move top column back
                tx_idx=self.blank_x;
                ty_idx += 1;
                while b>0 {self.push_single(6); b-=1; }
                while b<0 {self.push_single(4); b+=1; }
            } else {
                //Move tile up to second row
                //Special case when lies at bottom of partially solved column
                if ty_idx==3 && tx_idx==cl && tl != 8 && tl != 9
                {
                    if cl == 0
                    {
                        self.push_single(7);
                        tx_idx +=1;
                    } else {
                        if self.blank_x == cl { self.push(&[6,1,4]); }
                        //tile to column with blank
                        let mut a = util::subtract(self.blank_x, cl);
                        while a>0 { self.push_single(7); a -=1; }
                        tx_idx = self.blank_x;
                        //Move column upwards
                        self.push_single(util::subtract(3,self.blank_y));
                        ty_idx-=1;
                        //move some unsolved column down, so gap at top again
                        if util::subtract(self.blank_x, cl) > 1 { 
                            self.push(&[5,-3]); 
                        } else {
                            self.push(&[7,-3]);
                        }
                        //move bottom row back again
                        a = util::subtract(self.blank_x, cl);
                        while a>0 {self.push_single(5); a-=1; }
                    }
                }
                while ty_idx>1
                {
                    //move blank in top row over to column with the tile
                    let mut a = util::subtract(tx_idx, self.blank_x);
                    let mut b = a;
                    while a>0 { self.push_single(6); a-=1; }
                    while a<0{ self.push_single(4); a+=1; }
                    //move column with tile up
                    self.push_single(3);
                    ty_idx-=1;
                    //restore top row into position
                    if cl != 0
                    {
                        while b>0 { self.push_single(4); b-=1; }
                        while b<0 { self.push_single(6); b+=1; }
                    }
                    //Move adjacent column down
                    let cltl_condition = cl != 0 || tl == 0;
                    if tx_idx == 3
                    {
                        self.push(&[5,-3]);
                        if cltl_condition { self.push_single(7); }
                    } else{
                        self.push(&[7,-3]);
                        if cltl_condition { self.push_single(5); }
                    }
                }
            }

            //Move tile into top of column cl. Tile is in second row now, gap somewhere in top row.


        }

        //move down column if necessary

    }
} //end impl

mod util
{
    use std::convert::{TryFrom, TryInto};

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

    pub fn subtract(x : usize, y : usize) -> isize
    {
        let xx : isize = x.try_into().unwrap();
        let yy : isize = y.try_into().unwrap();
        return xx - yy;
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

    pub fn negative_of(val : usize) -> isize
    {
        return isize::try_from(val).ok().unwrap() * -1;
    }
}