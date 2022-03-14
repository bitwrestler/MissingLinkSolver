use super::ml_data::{SIZE_COLUMN, SIZE_TOTAL};

    pub const BLANK_IDX : usize = 12;
    pub const TILES : &[&str] = &["Tr","Ty","Tg","Tw","Mr","My","Mg","Mw","Br","By","Bg","Bw","_"];

    pub struct MLDisplay
    {
        cols : [[usize;4];4]
    }

    impl MLDisplay
    {
       pub fn new(data : super::ml_data::MLData)  -> MLDisplay
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

        for j in 0..SIZE_COLUMN{
                for i in 0..SIZE_COLUMN {
                    print!("{}", TILES[ self.cols[i][j] ]);
                    print!("  ");
                }
                println!();
           }
       }

       pub fn dump(&self) -> String
       {
            let mut s : String = "".to_owned();
            for i in 0..SIZE_COLUMN{
                 for j in 0..SIZE_COLUMN {
                     s.push_str(TILES[ self.cols[i][j] ] );
                 }
            }
            return s;
       }

       pub fn dump_raw(&self) -> [usize;16]
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