use std::{convert::{TryFrom, TryInto}, collections::VecDeque};

//convert signed int to uint asserting that it is possible
pub fn cvt_int(v : isize) -> usize
{
    return usize::try_from(v).ok().unwrap();
}

pub fn add(x : usize, y : isize) -> isize
{
    let x2 = isize::try_from(x).ok().unwrap();
    return x2 + y;
}

pub fn subtract(x : usize, y : usize) -> isize
{
    let xx : isize = x.try_into().unwrap();
    let yy : isize = y.try_into().unwrap();
    return xx - yy;
}

pub fn compare(x : usize, y : isize) -> isize
{
    let x2 = isize::try_from(x).ok().unwrap();
    if x2 < y { return -1; }
    else if x2 > y {return 1; }
    else { return 0; }
}

pub fn change_value(list : &mut VecDeque<isize>, idx : usize, value : isize)
{
    list[idx] = value;
}

pub fn negative_of(val : usize) -> isize
{
    return isize::try_from(val).ok().unwrap() * -1;
}
