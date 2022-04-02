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
"Tr  Ty  Tg  __
Mr  My  Mg  Tw
Mr  My  Mg  Mw
Br  By  Bg  Bw
";

    assert_eq!(ret,expected);
}

#[test]
pub fn MLDisplay_into_MLData()
{
    let sample = initbrd();
    let disp = MLDisplay::new(&sample);

    let sample2 : ml_data::MLData = disp.into();

    for i in 0..SIZE_TOTAL
    {
        assert_eq!(sample.posit[i],sample2.posit[i]);
    }
    assert_eq!(sample.blank_x,sample2.blank_x);
    assert_eq!(sample.blank_y,sample2.blank_y);
}

#[test]
pub fn MLDisplay_dump_value()
{
    let sample = initbrd();
    let disp = MLDisplay::new(&sample);

    let ret = disp.dump_str();

    assert_eq!("TrMrMrBrTyMyMyByTgMgMgBg__TwMwBw",ret);
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

#[test]
pub fn find_display_pos_found()
{
    let sample = String::from("Bw");
    let ret = MLDisplay::find_display_pos(sample);
    assert_eq!(11,ret);
}

#[test]
#[should_panic]
pub fn find_display_pos_notfound()
{
    let sample = String::from("JJ");
    MLDisplay::find_display_pos(sample);
}

#[test]
pub fn display_util_rotate_next_display_00_01()
{
    let col_row = (0usize,0usize);
    let rotated = MLDisplay::rotate_next_display(col_row);
    assert_eq!(0,rotated.0);
    assert_eq!(1,rotated.1);
}

#[test]
pub fn display_util_rotate_next_display_01_02()
{
    let col_row = (0usize,1usize);
    let rotated = MLDisplay::rotate_next_display(col_row);
    assert_eq!(0,rotated.0);
    assert_eq!(2,rotated.1);
}

#[test]
pub fn display_util_rotate_next_display_02_03()
{
    let col_row = (0usize,2usize);
    let rotated = MLDisplay::rotate_next_display(col_row);
    assert_eq!(0,rotated.0);
    assert_eq!(3,rotated.1);
}

#[test]
pub fn display_util_rotate_next_display_03_10()
{
    let col_row = (0usize,3usize);
    let rotated = MLDisplay::rotate_next_display(col_row);
    assert_eq!(1,rotated.0);
    assert_eq!(0,rotated.1);
}

#[test]
#[should_panic]
pub fn display_util_rotate_next_display_40_panic()
{
    let col_row = (4usize,0usize);
    MLDisplay::rotate_next_display(col_row);
}

#[test]
#[should_panic]
pub fn display_util_rotate_next_display_04_panic()
{
    let col_row = (0usize,4usize);
    MLDisplay::rotate_next_display(col_row);
}

#[test]
pub fn from_string_noseparators_expected()
{
    let sample = String::from("TwMwBw__TyMyMyByMgTrMrBgTgMgMrBr");
    let disp = MLDisplay::from(sample);

    let ret = disp.dump_fmt();
    let expected = 
"Tw  Ty  Mg  Tg
Mw  My  Tr  Mg
Bw  My  Mr  Mr
__  By  Bg  Br
";
    assert_eq!(ret,expected);
}

#[test]
pub fn from_string_separators_expected()
{
    let sample = String::from("TwMwBw__;TyMyMyBy;MgTrMrBg;TgMgMrBr");
    let disp = MLDisplay::from(sample);

    let ret = disp.dump_fmt();
    let expected = 
"Tw  Ty  Mg  Tg
Mw  My  Tr  Mg
Bw  My  Mr  Mr
__  By  Bg  Br
";
    assert_eq!(ret,expected);
}

#[test]
pub fn is_valid_valid_returns_true()
{
    let disp = MLDisplay::from(String::from("TwMwBw__;TyMyMyBy;MgTrMrBg;TgMgMrBr"));
    assert!(disp.is_valid());
}


#[test]
#[should_panic]
pub fn is_valid_invalid_blank_panics()
{
    let _ret = MLDisplay::from(String::from("__MwBw__;TyMyMyBy;MgTrMrBg;TgMgMrBr"));
}
