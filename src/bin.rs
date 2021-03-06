extern crate missing_link_solver;
use missing_link_solver::missing_link_solver::ml_display::MLDisplay;
use std::env;
use std::process;

fn main()
{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2
    {
        usage(String::from("Missing starting pattern!"));
        process::exit(1);
    }

    let thearg = args[1].clone();
    let disp = MLDisplay::from(thearg);

    missing_link_solver::missing_link_solver::solver(disp);
}

fn _usage()
{
    eprintln!("Missing Link Solver 1.0");
    eprintln!("Usage: missing_link_solver_cmd.exe [starting pattern]");
    eprintln!();
    eprintln!("Example: missing_link_solver_cmd.exe \"Tw__MwBw;TyMyMyBy;TgMgMgBg;TrMrMrBr\" ");
}

fn usage(message : String)
{
    eprintln!("{}",message);
    _usage();
}