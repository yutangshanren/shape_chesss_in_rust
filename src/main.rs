use std::env;
use std::process::exit;
use shape_chess_game::{play_game, shape};


fn main() {
    // Parse the args first.
    let mut arguments = env::args();
    let cmd_name = arguments.next();

    while let Some(arg) = arguments.next() {
        match arg.as_str() {
            "-i" =>  {
                shape::show_shape_index();
                exit(0);
            }
            "-d" =>  {
                if let Some(idx) = arguments.next() {
                    match  idx.parse::<usize>() {
                        Ok(n) => {
                            shape::show_shape_description(shape::map_from_index_to_shape(n));
                        }
                        Err(_) => {
                            show_help(cmd_name);
                        }
                    }
                }
                exit(0);
            }
            "-h" =>  {
                show_help(cmd_name);
                exit(0);
            }
            _ => {
                println!("Skip unknown options {}", arg.as_str());
            }
        }
    }

    play_game();
}

fn show_help(cmd: Option<String>) {
    if let Some(name) = cmd {
        println!("Usage:");
        println!("  {} -d n: show all sub-shape description for index n. ",name);
        println!("  or");
        println!("  {} -i: show shape indexes.",name);
        println!("  or");
        println!("  {} -h",name);
        println!("  or");
        println!("  {}",name);
    }
}




