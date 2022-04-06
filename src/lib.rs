//From javascript code orignially from  https://www.jaapsch.net/puzzles/missing.htm
//https://www.jaapsch.net/puzzles/javascript/missingj.htm


mod tests;

pub const MAX_SOLVE_ITERATIONS : usize = 10000;

pub mod missing_link_solver
{
    use std::collections::VecDeque;

    use crate::{MAX_SOLVE_ITERATIONS, missing_link_solver::ml_data::MoveType};

    pub mod ml_data;
    pub mod ml_display;
    

pub fn initbrd() -> self::ml_data::MLData {
    self::ml_data::MLData {
        posit: [0,1,2,12,4,5,6,3,4,5,6,7,8,9,10,11],
        blank_x: 3,
        blank_y: 0,
        seq: VecDeque::new(),
        mode: ml_data::MODE_NORMAL,
        last_move: ml_data::MoveType::StartingPosition
    }
}

pub fn solver(initial_setup : ml_display::MLDisplay) -> usize
{
    let mut data : ml_data::MLData = ml_display::MLDisplay::into(initial_setup);
    display_current(&data,0);
    let mut loop_count = 0;
    loop{
        data.solve();
        loop_count+=1;

        if loop_count >= MAX_SOLVE_ITERATIONS 
        {  
            panic!("too many iterations to find solution... something is wrong");
        }

        display_current(&data,loop_count);
        if data.solved() {break;}
    }
    display_final();
    return loop_count;
}

fn display_current(data : &ml_data::MLData, iteration : usize)
{
    let disp = ml_display::MLDisplay::new(&data);
    println!("Iteration: {}",iteration);
    println!("{}",display_current_wording(data.last_move));
    disp.display();
    println!("");
}

fn display_final()
{
    println!("Finished!");
}

fn display_current_wording(move_val: MoveType) -> String
{
    match move_val
    {
        MoveType::StartingPosition => String::from("Starting Position"),
        MoveType::TopMoveLeft => String::from("Rotate Top to the Left"),
        MoveType::TopMoveRight => String::from("Rotate Top to the Right"),
        MoveType::BottomMoveLeft => String::from("Rotate Bottom to the Left"),
        MoveType::BottomMoveRight => String::from("Rotate Bottom to the Right"),
        MoveType::MoveUp => String::from("Move square Up"),
        MoveType::MoveDown => String::from("Move square Down")
    }
}

}