    pub const BLANK_IDX : usize = 12;
    pub const TILES : &[&str] = &["Tr","Ty","Tg","Tw","Mr","My","Mg","Mw","Br","By","Bg","Bw","_"];

    struct MLDisplay
    {
        cols : [[usize;4];4]
    }


    impl MLDisplay
    {
       pub fn new(data : super::ml_data::MLData) -> MLDisplay
       {
            let mut initdata : [[usize;4];4] = [[BLANK_IDX;4];4];

           let mut dStruct =  MLDisplay{
              cols: initdata
           };

           let mut it : usize = 0;

           for i in 0..3{
                for j in 0..3 {
                    dStruct.cols[i][j] = data.posit[it];
                    it=it+1;
                }
           }

           return dStruct;

       }
    }