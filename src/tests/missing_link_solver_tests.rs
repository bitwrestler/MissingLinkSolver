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

#[test]
pub fn solver_full_scenario()
{
    let disp = MLDisplay::from(String::from("TwMwBw__;TyMyMyBy;MgTrMrBg;TgMgMrBr"));
    let iterations = solver(disp);
    assert_eq!(375,iterations);
}

#[test]
#[ignore = "debugging test"]
pub fn solve_endlessloop_debug()
{
    println!("starting rust version:");
    let mut data = MLData::default(); //MLDisplay::from(String::from("TwMwBw__;TyMyMyBy;MgTrMrBg;TgMgMrBr")).into();
    data.blank_x = 0;
    data.blank_y = 1;
    data.posit = [3,1,2,0,12,5,6,4,7,5,6,4,11,9,10,8];

    data.solve();
    
    let mut idx = 0;
    println!("seq debug:");
    loop{
        let aele = data.seq.pop_front();
        if aele == None { break; }
        println!("{}: {}", idx, aele.unwrap());
        idx +=1;
    }
}

mod rust_assumption_tests
{
    #[test]
    pub fn for_range_start_gt_doesnotrun()
    {
        let mut run_check : bool = false;
        for _aele in 4..3
        {
            run_check=true;
        }
        assert_eq!(false,run_check);
    }
}
