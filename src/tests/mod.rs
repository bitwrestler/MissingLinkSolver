#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::missing_link_solver::*;
    use crate::missing_link_solver::ml_data::{MLData, SIZE_COLUMN, SIZE_TOTAL};
    use crate::missing_link_solver::ml_display::{MLDisplay, BLANK_IDX};

    #[test]
    pub fn initbrd_expect_default()
    {
        let sample = initbrd();
        assert_eq!(3,sample.blank_x);
        assert_eq!(0,sample.blank_y);
        assert_eq!(ml_data::MODE_NORMAL,sample.mode);
        assert!(sample.solved());
    }

    fn assertColumn(sample : &MLData, expected : [usize;SIZE_COLUMN], idx : usize)
    {
        let disp = MLDisplay::new(sample);
        let ret = disp.dump_col_raw(idx);
        for i in 0..SIZE_COLUMN{
            assert_eq!(expected[i],ret[i])
        }
    }

    #[test]
    pub fn MLData_domove_downup()
    {
        let expected : [usize;SIZE_COLUMN] = [3,BLANK_IDX,7,11];
        let expected2 : [usize;SIZE_COLUMN] = [BLANK_IDX,3,7,11];
        let mut sample = initbrd();
        
        sample.domove(1);
        assertColumn(&sample, expected, 3);

        sample.domove(0);
        assertColumn(&sample, expected2, 3);
    }

    #[test]
    pub fn MLData_push_domove_downup()
    {
        let expected : [usize;SIZE_COLUMN] = [3,BLANK_IDX,7,11];
        let expected2 : [usize;SIZE_COLUMN] = [BLANK_IDX,3,7,11];
        let mut sample = initbrd();
        
        sample.push(&[1]);
        assertColumn(&sample, expected, 3);

        sample.push(&[-1]);
        assertColumn(&sample, expected2, 3);

        assert_eq!(sample.seq.len(), 0);
    }

    #[test]
    pub fn MLData_doleft_topRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [0,3,7,11];
        let mut sample = initbrd();
        
        sample.doleft(0);
        assertColumn(&sample, expected2, 3);
    }

    #[test]
    pub fn MLData_doleft_bottomRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [BLANK_IDX,3,7,8];
        let mut sample = initbrd();
        
        sample.doleft(1);
        assertColumn(&sample, expected2, 3);
    }

    #[test]
    pub fn MLData_push_doleft_topRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [0,3,7,11];
        let mut sample = initbrd();
        sample.push(&[4]);
        assertColumn(&sample,expected2,3);
        assert_eq!(sample.seq.len(), 1);
        assert_eq!(sample.seq[0],4);
    }

    #[test]
    pub fn MLData_push_doleft_bottomRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [BLANK_IDX,3,7,8];
        let mut sample = initbrd();
        sample.push(&[5]);
        assertColumn(&sample,expected2,3);
        assert_eq!(sample.seq.len(), 1);
        assert_eq!(sample.seq[0],5);
    }

    #[test]
    pub fn MLData_doright_topRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [2,3,7,11];
        let mut sample = initbrd();
        
        sample.doright(0);
        assertColumn(&sample, expected2, 3);
    }

    #[test]
    pub fn MLData_doright_bottomRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [BLANK_IDX,3,7,10];
        let mut sample = initbrd();
        
        sample.doright(1);
        assertColumn(&sample, expected2, 3);
    }

    #[test]
    pub fn MLData_push_doright_topRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [2,3,7,11];
        let mut sample = initbrd();
        sample.push(&[6]);
        assertColumn(&sample,expected2,3);
    }

    #[test]
    pub fn MLData_push_doright_bottomRow()
    {
        let expected2 : [usize;SIZE_COLUMN] = [BLANK_IDX,3,7,10];
        let mut sample = initbrd();
        sample.push(&[7]);
        assertColumn(&sample,expected2,3);
    }

    #[test]
    pub fn MLData_find_found()
    {
        let sample = initbrd();
        let ret = sample.find(BLANK_IDX);
        assert_eq!(ret,3);
    }

    #[test]
    pub fn MLData_find_notfound()
    {
        let sample = initbrd();
        let ret = sample.find(88);
        assert_eq!(ret,SIZE_TOTAL-1);
    }

    #[test]
    pub fn MLDisplay_dump_fmt()
    {
        let sample = initbrd();
        let disp = MLDisplay::new(&sample);
        let ret = disp.dump_fmt();
        let expected = 
"Tr  Ty  Tg  _
Mr  My  Mg  Tw
Mr  My  Mg  Mw
Br  By  Bg  Bw
";

        assert_eq!(ret,expected);
    }

    #[test]
    pub fn MLDisplay_dump_value()
    {
        let sample = initbrd();
        let disp = MLDisplay::new(&sample);

        let ret = disp.dump_str();

        assert_eq!("TrMrMrBrTyMyMyByTgMgMgBg_TwMwBw",ret);
    }

    #[test]
    pub fn MLDisplay_dumpraw_value()
    {
        let sample = initbrd();
        let disp = MLDisplay::new(&sample);

        let expected : [usize;16] = [0,4,4,8,1,5,5,9,2,6,6,10,BLANK_IDX,3,7,11];
        let ret = disp.dump_raw();

        for i in 0..SIZE_TOTAL{
            assert_eq!(expected[i],ret[i])
       }
    }

    #[test]
    pub fn MLDisplay_dumpcolraw_value()
    {
        let sample = initbrd();
        let disp = MLDisplay::new(&sample);

        let expected : [usize;SIZE_COLUMN] = [0,4,4,8];
        let ret = disp.dump_col_raw(0);

        for i in 0..SIZE_COLUMN{
            assert_eq!(expected[i],ret[i])
       }
    }
}