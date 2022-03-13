//From javascript code orignially from  https://www.jaapsch.net/puzzles/missing.htm
//https://www.jaapsch.net/puzzles/javascript/missingj.htm

const MODE_NORMAL: isize = 0;
const MODE_SOLVING_SCRAMBLED: isize = 1;
const MODE_EDIT: isize = 2;
const MODE_SOLVING: isize = 3;

struct MLData {
    posit: [isize; 16],
    blank_x: isize,
    blank_y: isize,
    mode: isize
}

fn initbrd() -> MLData {
    MLData {
        posit: [0,1,2,12,4,5,6,3,4,5,6,7,8,9,10,11],
        blank_x: 3,
        blank_y: 0,
        mode: MODE_NORMAL
    }
}