#![allow(non_snake_case)]
#[cfg(test)]

use crate::missing_link_solver::*;
use crate::missing_link_solver::ml_data::{SIZE_COLUMN, SIZE_TOTAL, BLANK_IDX};
use crate::missing_link_solver::ml_display::{MLDisplay};


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
