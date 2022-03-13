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
           let mut it : usize = 0;
           for i in 0..=3{
               it = i;
                for j in 0..=3 {
                    d_struct.cols[i][j] = data.posit[it];
                    it+=4;
                }
           }
           return d_struct;
       }

       pub fn display(&self)
       {

        for j in 0..=3{
                for i in 0..=3 {
                    print!("{}", TILES[ self.cols[i][j] ]);
                    print!("  ");
                }
                println!();
           }
       }

       pub fn dump(&self) -> String
       {
            let mut s : String = "".to_owned();
            for i in 0..=3{
                 for j in 0..=3 {
                     s.push_str(TILES[ self.cols[i][j] ] );
                 }
            }
            return s;
       }

       pub fn dump_raw(&self) -> [usize;16]
       {
        let mut ret : [usize;16] = [BLANK_IDX;16];
        let mut pos : usize = 0;
        for i in 0..=3{
            for j in 0..=3 {
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
            let initdata : [[usize;4];4] = [[BLANK_IDX;4];4];
            let d_struct =  MLDisplay{
                cols: initdata
             };
             return d_struct;
        }
    }