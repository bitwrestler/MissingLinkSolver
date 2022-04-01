#[doc(inline)]
use super::ml_data::{SIZE_COLUMN, SIZE_TOTAL,BLANK_IDX};

//#[cfg(windows)]
//pub const LINE_ENDING: &'static str = "\r\n";
//#[cfg(not(windows))]
pub const LINE_ENDING: &'static str = "\n";

    pub const TILES : &[&str] = &["Tr","Ty","Tg","Tw","Mr","My","Mg","Mw","Br","By","Bg","Bw","__"];

    pub struct MLDisplay
    {
        //2-dimensional array columns/rows
        cols : [[usize;4];4]
    }

    impl MLDisplay
    {
       pub fn new(data : &super::ml_data::MLData)  -> Self
       {
           let mut d_struct = MLDisplay::default();
           for i in 0..SIZE_COLUMN {
            let mut it : usize = i;
                for j in 0..SIZE_COLUMN {
                    d_struct.cols[i][j] = data.posit[it];
                    it+=4;
                }
           }
           return d_struct;
       }

       pub fn display(&self)
       {
            print!("{}",self.dump_fmt());
       }

       pub fn dump_fmt(&self) -> String
       {
        let mut s : String = "".to_owned();
        for j in 0..SIZE_COLUMN{
            for i in 0..SIZE_COLUMN {
                s.push_str(TILES[ self.cols[i][j] ]);
                if i < SIZE_COLUMN - 1{
                    s.push_str("  ");
                }
            }
            s.push_str(LINE_ENDING);
        } 
       return s;
       }

       pub fn dump_str(&self) -> String
       {
            let mut s : String = "".to_owned();
            for i in 0..SIZE_COLUMN{
                 for j in 0..SIZE_COLUMN {
                     s.push_str(TILES[ self.cols[i][j] ] );
                 }
            }
            return s;
       }

       pub fn dump_raw(&self) -> [usize;SIZE_TOTAL]
       {
        let mut ret : [usize;SIZE_TOTAL] = [BLANK_IDX;SIZE_TOTAL];
        let mut pos : usize = 0;
        for i in 0..SIZE_COLUMN{
            for j in 0..SIZE_COLUMN {
                ret[pos] = self.cols[i][j];
                pos+=1;
            }
        }
        return ret;
       }

       pub fn dump_col_raw(&self, idx : usize) -> [usize;SIZE_COLUMN]
       {
        let mut ret : [usize;SIZE_COLUMN] = [BLANK_IDX;SIZE_COLUMN];
        let mut pos : usize = 0;
        for j in 0..SIZE_COLUMN {
            ret[pos] = self.cols[idx][j];
            pos+=1;
        }
        return ret;
       }

       pub fn set_value(&mut self, col_idx : usize, row_idx : usize, value : String)
        {
        self.cols[col_idx][row_idx] = MLDisplay::find_display_pos(value);
        }

        pub fn find_display_pos(value : String) -> usize
        {
            let mut idx : usize = 0;
            for adisp in TILES
            {
                if value.eq(adisp) { return idx; }
                idx+=1;
            }
            panic!("Can not find tile {}",value);
        }

        pub fn rotate_next_display(pair : (usize,usize)) -> (usize,usize)
        {
            const SC : usize = crate::missing_link_solver::ml_data::SIZE_COLUMN;

            let mut new_col = pair.0;
            let mut new_row = pair.1;
            
            if new_col >= SC
            {
                panic!("Column exceeds limit");
            }

            if new_row >= SC
            {
                panic!("Row exceeds limit");
            }

            new_row+=1;

            if new_row == SC
            {
                new_row = 0;
                new_col +=1;
            }

            if new_col == SC
            {
                new_row = 0;
                new_col = 0;
            }

            return (new_col,new_row);
        }
    }

    impl Default for MLDisplay
    {
        fn default () -> MLDisplay
        {
            let initdata : [[usize;SIZE_COLUMN];SIZE_COLUMN] = [[BLANK_IDX;SIZE_COLUMN];SIZE_COLUMN];
            let d_struct =  MLDisplay{
                cols: initdata
             };
             return d_struct;
        }
    }

    impl From<String> for MLDisplay
    {
        fn from(item : String) -> Self
        {
            let mut disp = MLDisplay::default();
            let mut buf_pos : u8 = 0;
            let mut col_row = (0usize,0usize);

            let mut tmp_buf : [char;2] = ['\0','\0'];
            for achar in item.chars()
            {
                if achar != ';'
                {   
                    if buf_pos == 1
                    {
                        tmp_buf[1] = achar;
                        let this_tile_str = tmp_buf.iter().collect();
                        let tile_idx = MLDisplay::find_display_pos(this_tile_str);
                        disp.cols[col_row.0][col_row.1] = tile_idx;
                        buf_pos = 0;
                        col_row = MLDisplay::rotate_next_display( col_row );
                    } else {
                        tmp_buf[0] = achar;
                        buf_pos = 1;
                    }
                }                
            }
            return disp;
        }
    }

    impl Into<super::ml_data::MLData> for MLDisplay
    {
        fn into(self) -> super::ml_data::MLData
        {
            let mut d_struct = super::ml_data::MLData::default();
            let mut idx : usize = 0;
            for i in 0..SIZE_COLUMN { //cols
                    for j in 0..SIZE_COLUMN { //rows
                       d_struct.posit[idx] = self.cols[j][i];
                       
                       if d_struct.posit[idx] == BLANK_IDX
                       {
                            d_struct.blank_y = i;
                            d_struct.blank_x = j;
                       }
                       idx+=1;
                    }
               }
            return d_struct;
        }
    }