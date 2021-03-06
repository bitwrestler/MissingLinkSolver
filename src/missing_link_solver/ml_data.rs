use std::collections::VecDeque;

pub const MODE_NORMAL: usize = 0;
pub const MODE_SOLVING_SCRAMBLED: usize = 1;
pub const MODE_EDIT: usize = 2;
pub const MODE_SOLVING: usize = 3;

pub const SIZE_TOTAL : usize = 16;
pub const SIZE_COLUMN : usize = 4;

pub const BLANK_IDX : usize = 12;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum MoveType{
    StartingPosition=0,
    TopMoveLeft=1,
    TopMoveRight=2,
    BottomMoveLeft=4,
    BottomMoveRight=8,
    MoveUp=16,
    MoveDown=32
}

pub struct MLData {
    pub posit: [usize; 16],
    pub blank_x: usize,
    pub blank_y: usize,
    pub seq : VecDeque<isize>,
    pub mode: usize,
    pub last_move : MoveType
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
        if self.blank_y==i*3 { self.blank_x=util::cvt_int( util::subtract(self.blank_x,1) & 3); }
        self.last_move = if i == 0 { MoveType::TopMoveLeft } else { MoveType::BottomMoveLeft };
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
        self.last_move = if i == 0 { MoveType::TopMoveRight } else { MoveType::BottomMoveRight }
    }

    pub fn domove(&mut self,y : isize)
    {
        //try up/down move
        let mut c : usize = self.blank_y * 4 + self.blank_x;
        
        //move up
        while util::compare(self.blank_y, y) == -1  //self.blank_y < y
        {
            self.posit[c] = self.posit[c + 4];
            c+=4;
            self.blank_y+=1;
            self.last_move = MoveType::MoveUp;
        }
        //move down
        while  util::compare(self.blank_y, y) == 1  //self.blank_y > y
        {
            self.posit[c]=self.posit[c-4];
            c-=4;
            self.blank_y-=1;
            self.last_move = MoveType::MoveDown;
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
                    let last_ele = *self.seq.back().unwrap();
                    if m >= 4 && last_ele == (m^2) { self.seq.pop_back(); }
                    else if m<4 && last_ele < 4 {
                        m += last_ele;
                        let last_idx = self.seq.len() - 1;
                        util::change_value(&mut self.seq, last_idx, m);
                        if m == 0 { self.seq.pop_back(); }
                    }
                    else {
                        self.seq.push_back(m);    
                    }
                } else {
                    self.seq.push_back(m);
                }
            }
        }
    }

    

    pub fn solve(&mut self)
    {
        if self.mode==0 || self.mode==1
        {
            self.mode=3;
            self.seq.clear();

            //no solution set up yet. Construct it!
            //save pieces;
            let mut back : MLData = MLData::default();
            Self::copy_posit(self,&mut back);  //var back = new Array();
                                            //for(var i=0;i<16;i++) back[i]=posit[i];
                                            //back[16]=blnkx; back[17]=blnky;
            back.blank_x = self.blank_x;
            back.blank_y = self.blank_y;

            //solve first column
            if self.posit[0]!=0 || self.posit[4]!=4 || self.posit[8]!=4 || self.posit[12]!=8
            {
                self.solvetile(8, 0);
                self.solvetile(4, 0);
                self.solvetile(4, 0);
                self.solvetile(0, 0);
            }

            //solve second column
            if self.posit[1]!=1 || self.posit[5]!=5 || self.posit[9]!=5 || self.posit[13]!=9 {
                self.solvetile(9,1);
                self.solvetile(5,1);
                self.solvetile(5,1);
                self.solvetile(1,1);
            }

            //solve bottom tile of third column
            self.push_single(util::negative_of(self.blank_y)); //blank to top
            if self.blank_x==2 { self.push(&[1,4,-1,6]); } //blank to top right
            let mut t= self.find(10);
            if t==2 { self.push(&[4,1,6,-1,6,1,4,-1]); t+=4;}
            while t==6 || t==10 { self.push(&[7,3,5,4,-3,6]); t+=4;}
            while t==7 || t==11 { self.push(&[4,3,6,7,-3,5]); t+=4;}

            //solve bottom of last column.
            t=self.find(11);
            if t==2 { self.push(&[4,1,6,-1,6,1,4,-1]); t+=4;}
            while t==6 || t==10 { self.push(&[3,4,5,-3,6,7]); t+=4;}
            while t==7 || t==11 { self.push(&[4,5,3,6,7,-3]); t+=4;}

            //solve middle of last column.
            t=self.find(7);
            if t==6 { self.push(&[4,1,6,-1,6,1,4,-1]); t=7; }
            if t==7 { self.push(&[3,5,-3, 6,1,4,-1,4,3,6,7,-3]); }
            if t==10 {self.push(&[4,5,3,7,6,6,-3,4,1,4,-1, 6,3,4,5,-3,6,7]); t=2;}
            if t==2  {self.push(&[3,4,5,-3,6, 1,6,-1,4, 3, 7,-3]); }

            //solve top of last column.
            t=self.find(3);
            if t==10 { self.push(&[4,5,3,7,6,6,-3,4,1,4,-1, 6,3,4,5,-3,6,7]); t=2;}
            if t==2 { self.push(&[1,6,-1,4,1,4,-1,6]); }
            if t==6 { self.push(&[4,1,6,-1,6,1,4,-1]); }

            //solve top of third column.
            t=self.find(2);
            if t==10 { self.push(&[4,5,3,7,6,6,-3,4,1,4,-1,6,3,4,5,-3,6,7]); }
            if t==6 { self.push(&[4,5,3,7,6,-3,4,1,6,-1, 6,3,4,4,5,-3,6,7]); }

            //restore pieces;
            Self::copy_posit(&back, self);
            self.blank_x = back.blank_x;
            self.blank_y = back.blank_y;
        }

        if self.mode >= 3
        {
            //do next move of prerecorded sequence
            if self.seq.is_empty() { 
                self.mode = 0; 
            } else {
                // var c=seq.shift();
                let c = self.seq.pop_front().unwrap();   //var c=seq[0];
                                                                //for(var i=1;i<seq.length;i++) seq[i-1]=seq[i];
                                                                //seq.length--;
                if self.seq.is_empty() { self.mode = 0; }
                if c==4 || c==5 { 
                    self.doleft(util::cvt_int(c-4)); 
                } else if c==6 || c==7 {
                    self.doright(util::cvt_int(c-6));
                } else {
                    self.domove( util::add(self.blank_y, c) );
                }
            }
            //display()
        }
    }

    pub fn copy_posit(copyfrom : &MLData, copyto : &mut MLData)
    {
        let mut idx : usize = 0;
        for i in copyfrom.posit
        {
            copyto.posit[idx] = i;
            idx+=1;
        }
        copyto.blank_x = copyfrom.blank_x;
        copyto.blank_y = copyfrom.blank_y;
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

        //find tile
        let mut tx_idx : usize = 0;
        let mut ty_idx : usize = 0;

        for ty in 0..SIZE_COLUMN
        {
            ty_idx = ty;
            let tx_start = cl+1;
            tx_idx = tx_start;
            for tx in tx_start..=SIZE_COLUMN
            {
                tx_idx = tx;

                if tx_idx > SIZE_COLUMN - 1 { break; }

                let thisidx = ty*4+tx;
                if self.posit[thisidx]==tl { break; }
            }
            if tx_idx < 4 {break;}
        }
          
        if tx_idx>=4
        {
            tx_idx = cl;
            for tx in util::inclusive_reverse_range(cl) //(0..=cl).rev()
            {
                tx_idx = tx;
                ty_idx = 3;
                for ty in util::inclusive_reverse_range(ty_idx) //(0..=3).rev(
                {
                    ty_idx = ty;
                    let thisidx = ty*4+tx;
                    if self.posit[thisidx]==tl {break;}
                }
                //if ty_idx >= 0  //this condition is really suspect... see orignal code... this expression is always true
                if ty_idx > 0
                {
                    break;
                } 
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
                self.push_single(-1);
                //Move top column back
                tx_idx=self.blank_x;
                //ty_idx += 1; //this value is not used again on this path
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
                        if self.blank_x == cl { 
                            self.push(&[6,1,4]); 
                        }
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
            if cl!=0 && self.blank_x != cl
            {
                //move gap at spot of tile now at top of column cl.
                //gap to second row
                if self.blank_x == tx_idx
                {
                    if tx_idx==3 { 
                        self.push(&[4,1,6]); 
                    } else {
                        self.push(&[6,1,4]);
                    }
                } else {
                    self.push_single(1);
                }
                //columns tile above gap
                let mut a = cl;
                while a < self.blank_x { self.push_single(6); a+=1; }
                //pull down piece at top of column cl
                self.push_single(-1);
            }
            //move top row so that the blank lies above the tile
            while self.blank_x<tx_idx {self.push_single(6);}
            while self.blank_x>tx_idx {self.push_single(4);}
            //tile into top row
            self.push_single(1);
            while cl<tx_idx { self.push_single(4); tx_idx-=1; }
            //gap back on top row.
            self.push_single(-1);
        }

        //move down column if necessary
        if tl>1
        {
            self.push_single(2); //gap to third row
            if cl!=0
            {
                let mut a = self.blank_x - cl;
                while a>0 { self.push_single(7); a-=1; }
            }
            self.push_single(1); //gap to column
            let mut a = self.blank_x - cl;
            while a>0 {self.push_single(5); a-=1;} //gap to column
            self.push_single(-3); //column down
        }
    }
} //end impl

impl Default for MLData
{
    fn default() -> MLData
    {
        //1-dimensional array rows/columns
        MLData { posit: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            , blank_x: 3, blank_y: 0, seq: VecDeque::new(), mode: MODE_NORMAL, last_move: MoveType::StartingPosition }
    }
}

pub mod util;