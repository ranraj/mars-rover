#[derive(Debug)]
pub enum Command {
    LEFT,
    RIGHT,
    MOVE,
}
impl Command {
    pub fn get_command(command: char) -> Self {
        match command {
            'L' => Command::LEFT,
            'R' => Command::RIGHT,
            'M' => Command::MOVE,
            _ => panic!("Not a valid command"),
        }
    }
}

#[derive(Debug)]
pub struct Position(pub i32, pub i32);

#[derive(Debug)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
impl Direction {
    pub fn get_direction(direction: String) -> Self {
        match direction.as_str() {
            "N" => Direction::NORTH,
            "S" => Direction::SOUTH,
            "E" => Direction::EAST,
            "W" => Direction::WEST,
            _ => panic!("Not a valid direction"),
        }
    }
}
#[derive(Debug)]
pub struct Rover {
    position: Position,
    direction: Direction,
}
impl Rover {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
    /* Execute sequence of commands */
    pub fn actions(&mut self, commands: Vec<Command>) {
        commands.iter().for_each(|a| {
            self.action(a);
        });
    }
    /* Execute single of command and update the current state in Plateau */
    fn action(&mut self, command: &Command) {
        self.change_direction_for_command(command);
        // Step forward after Direction has confirmed
        self.step_forward();
    }
    fn step_forward(&mut self) {
        let new_position = match self.direction {
            Direction::NORTH => Position(self.position.0, self.position.1 + 1),
            Direction::EAST => Position(self.position.0 + 1, self.position.1),
            Direction::SOUTH => Position(self.position.0, self.position.1 - 1),
            Direction::WEST => Position(self.position.0 - 1, self.position.1),
        };
        self.position = new_position;
    }

    /* Set the Direction for Right and Left command */
    fn change_direction_for_command(&mut self, command: &Command) {
        let direction_option = match command {
            Command::LEFT => Some(match self.direction {
                Direction::NORTH => Direction::WEST,
                Direction::WEST => Direction::SOUTH,
                Direction::SOUTH => Direction::EAST,
                Direction::EAST => Direction::NORTH,
            }),
            Command::RIGHT => Some(match self.direction {
                Direction::NORTH => Direction::EAST,
                Direction::EAST => Direction::SOUTH,
                Direction::SOUTH => Direction::WEST,
                Direction::WEST => Direction::NORTH,
            }),
            // Move command does not have change only step forward
            Command::MOVE => None,
        };
        if let Some(direction) = direction_option {
            self.direction = direction;
        };
    }
}
#[derive(Debug)]
pub struct Plateau {
    row: i32,
    col: i32,
    pub rover: Rover,
}

impl Plateau {
    pub fn new(row: i32, col: i32, rover: Rover) -> Self {
        Self { row, col, rover }
    }
    pub fn display(&self) {
        println!("{:?}  {:?}", self.rover.position, self.rover.direction);
    }
}
