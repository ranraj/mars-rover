use super::rover::{Plateau,Command,Rover,Position,Direction};

/* Input handlers */
pub fn handle_input() -> (Plateau, Vec<Command>) {
    let (row, col) = get_plateau(1, 2);
    let possition = get_current_rover_possition(4, 5);
    let direction = get_rovers_current_direction(6);
    let rover = Rover::new( possition, direction);
    let plateau = Plateau::new(row, col,rover);
    (plateau, get_rovers_action_commands(3))
}

fn get_plateau(i: usize, j: usize) -> (i32, i32) {
    let row: i32 = std::env::args()
        .nth(i)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        })
        .parse::<i32>()
        .ok()
        .expect("I wasn't given an int for row in the first arg!");

    let col: i32 = std::env::args()
        .nth(j)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        })
        .parse::<i32>()
        .ok()
        .expect("I wasn't given an int for col in the second arg!");
    (row, col)
}

fn get_current_rover_possition(i: usize, j: usize) -> Position {
    let position_x: i32 = std::env::args()
        .nth(i)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        })
        .parse::<i32>()
        .ok()
        .expect("I wasn't given an int for current possition - x in the thrid arg!");

    let position_y: i32 = std::env::args()
        .nth(j)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        })
        .parse::<i32>()
        .ok()
        .expect("I wasn't given an int for current possition - y in the thrid arg!");
        Position(position_x, position_y)
}

fn get_rovers_action_commands(i: usize) -> Vec<Command> {
    let commands_str: String = std::env::args().nth(i).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    commands_str.chars().map(Command::get_command).collect()
}

fn get_rovers_current_direction(i: usize) -> Direction {
    let current_direction_str: String = std::env::args().nth(i).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    Direction::get_direction(current_direction_str)
}
