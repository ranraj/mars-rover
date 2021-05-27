/**
 * cargo run 5 5 LMLMLMLMM 1 2 N
 * Expected : 1 3 N
 * 
 * cargo run 5 5 MMRMMRMRRM 3 3 E
 * Expected 5 1 E
 */

use marsrover::handler as input;
fn main() {
    let (mut plateau, commands) = input::handle_input();
    println!("{:?}", plateau);
    let rover = &mut plateau.rover;
    rover.actions(commands);
    plateau.display();
}

