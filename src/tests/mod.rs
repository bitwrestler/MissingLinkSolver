#[cfg(test)]
mod tests {
    use crate::missing_link_solver::*;
    use crate::missing_link_solver::ml_display::{MLDisplay, BLANK_IDX};

    #[test]
    pub fn initbrd_expect_default()
    {
        let sample = initbrd();
        assert_eq!(3,sample.blank_x);
        assert_eq!(0,sample.blank_y);
        assert_eq!(ml_data::MODE_NORMAL,sample.mode);
    }

    #[test]
    pub fn MLDisplay_constructor_expected_dump_value()
    {
        let sample = initbrd();
        let disp = MLDisplay::new(sample);

        let ret = disp.dump();

        assert_eq!("TrMrMrBrTyMyMyByTgMgMgBg_TwMwBw",ret);
    }

    #[test]
    pub fn MLDisplay_constructor_expected_dumpraw_value()
    {
        let sample = initbrd();
        let disp = MLDisplay::new(sample);

        let expected : [usize;16] = [0,4,4,8,1,5,5,9,2,6,6,10,BLANK_IDX,3,7,11];
        let ret = disp.dump_raw();

        for i in 0..15{
            assert_eq!(expected[i],ret[i])
       }
    }
}