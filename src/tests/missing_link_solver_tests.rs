#![allow(unused_imports)]

#[cfg(test)]
use crate::missing_link_solver::*;
use crate::missing_link_solver::ml_display::MLDisplay;
use crate::missing_link_solver::ml_data::MLData;

#[test]
pub fn initbrd_expect_default()
{
    let sample = initbrd();
    assert_eq!(3,sample.blank_x);
    assert_eq!(0,sample.blank_y);
    assert_eq!(ml_data::MODE_NORMAL,sample.mode);
    assert!(sample.solved());
}


#[test]
pub fn test_solve_scenario_simple1movesolve()
{
    let mut dataObj : MLData = MLDisplay::into(MLDisplay::from(String::from("Tw__MwBw;TyMyMyBy;TgMgMgBg;TrMrMrBr")));
    dataObj.solve();
    let disp = MLDisplay::new(&dataObj);
    assert_eq!(disp.dump_str(),"__TwMwBwTyMyMyByTgMgMgBgTrMrMrBr")
}
