#![allow(dead_code)]

use crate::missing_link_solver::ml_data::{MLData, SIZE_COLUMN};
use crate::missing_link_solver::ml_display::{MLDisplay};

#[test]
pub fn MLData_domove_downup()
{
    let expected : [usize;SIZE_COLUMN] = [3,crate::missing_link_solver::ml_data::BLANK_IDX,7,11];
    let expected2 : [usize;SIZE_COLUMN] = [crate::missing_link_solver::ml_data::BLANK_IDX,3,7,11];
    let mut sample = crate::missing_link_solver::initbrd();
    
    sample.domove(1); //move down
    assert_column(&sample, expected, 3);
    assert_eq!(sample.blank_y,1);
    assert_eq!(sample.blank_x,3);

    sample.domove(0); //move up
    assert_column(&sample, expected2, 3);
    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,3);
}

#[test]
pub fn MLData_push_domove_downup()
{
    let expected : [usize;SIZE_COLUMN] = [3,crate::missing_link_solver::ml_data::BLANK_IDX,7,11];
    let expected2 : [usize;SIZE_COLUMN] = [crate::missing_link_solver::ml_data::BLANK_IDX,3,7,11];
    let mut sample = crate::missing_link_solver::initbrd();
    
    sample.push(&[1]); //move down
    assert_column(&sample, expected, 3);
    assert_eq!(sample.blank_y,1);
    assert_eq!(sample.blank_x,3);

    sample.push(&[-1]); //move up
    assert_column(&sample, expected2, 3);
    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,3);

    assert_eq!(sample.seq.len(), 0);
}

#[test]
pub fn MLData_doleft_topRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [0,3,7,11];
    let mut sample = crate::missing_link_solver::initbrd();
    
    sample.doleft(0);
    assert_column(&sample, expected2, 3);
    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,2);
}

#[test]
pub fn MLData_doleft_bottomRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [crate::missing_link_solver::ml_data::BLANK_IDX,3,7,8];
    let mut sample = crate::missing_link_solver::initbrd();
    
    sample.doleft(1);
    assert_column(&sample, expected2, 3);
    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,3);
}

#[test]
pub fn MLData_push_doleft_topRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [0,3,7,11];
    let mut sample = crate::missing_link_solver::initbrd();
    sample.push(&[4]);
    assert_column(&sample,expected2,3);
    assert_eq!(sample.seq.len(), 1);
    assert_eq!(sample.seq[0],4);

    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,2);
}

#[test]
pub fn MLData_push_doleft_bottomRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [crate::missing_link_solver::ml_data::BLANK_IDX,3,7,8];
    let mut sample = crate::missing_link_solver::initbrd();
    sample.push(&[5]);
    assert_column(&sample,expected2,3);
    assert_eq!(sample.seq.len(), 1);
    assert_eq!(sample.seq[0],5);

    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,3);
}

#[test]
pub fn MLData_doright_topRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [2,3,7,11];
    let mut sample = crate::missing_link_solver::initbrd();
    
    sample.doright(0);
    assert_column(&sample, expected2, 3);

    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,0);
}

#[test]
pub fn MLData_doright_bottomRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [crate::missing_link_solver::ml_data::BLANK_IDX,3,7,10];
    let mut sample = crate::missing_link_solver::initbrd();
    
    sample.doright(1);
    assert_column(&sample, expected2, 3);

    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,3);
}

#[test]
pub fn MLData_push_doright_topRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [2,3,7,11];
    let mut sample = crate::missing_link_solver::initbrd();
    sample.push(&[6]);
    assert_column(&sample,expected2,3);

    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,0);
}

#[test]
pub fn MLData_push_doright_bottomRow()
{
    let expected2 : [usize;SIZE_COLUMN] = [crate::missing_link_solver::ml_data::BLANK_IDX,3,7,10];
    let mut sample = crate::missing_link_solver::initbrd();
    sample.push(&[7]);
    assert_column(&sample,expected2,3);

    assert_eq!(sample.blank_y,0);
    assert_eq!(sample.blank_x,3);
}

#[test]
pub fn MLData_find_found()
{
    let sample = crate::missing_link_solver::initbrd();
    let ret = sample.find(crate::missing_link_solver::ml_data::BLANK_IDX);
    assert_eq!(ret,3);
}

#[test]
pub fn MLData_find_notfound()
{
    let sample = crate::missing_link_solver::initbrd();
    let ret = sample.find(88);
    assert_eq!(ret,crate::missing_link_solver::ml_data::SIZE_TOTAL-1);
}

fn assert_column(sample : &MLData, expected : [usize;SIZE_COLUMN], idx : usize)
{
    let disp = MLDisplay::new(sample);
    let ret = disp.dump_col_raw(idx);
    for i in 0..SIZE_COLUMN{
        assert_eq!(expected[i],ret[i])
    }
}