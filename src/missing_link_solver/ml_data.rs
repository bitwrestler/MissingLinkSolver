    pub const MODE_NORMAL: usize = 0;
    pub const MODE_SOLVING_SCRAMBLED: usize = 1;
    pub const MODE_EDIT: usize = 2;
    pub const MODE_SOLVING: usize = 3;

pub struct MLData {
    pub posit: [usize; 16],
    pub blank_x: usize,
    pub blank_y: usize,
    pub mode: usize
}