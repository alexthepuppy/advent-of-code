static INPUT_FILE: &str = include_str!("../../datafiles/Y25/day01-1.txt");

struct NobOperation {
    delta: usize,
    direction: Direction
}

enum Direction {
    Clockwise,
    Counterclockwise
}

// A Struct to represent the dial and its rotation, and encapuslate its logic
struct DialRotation {
    rotation: usize,              // The position of the dial; should be constrained to 0-99 inclusivly.
    pub zero_crossings: usize   // The number of times the dial LANDS ON ZERO at the end of the spinning operation.
}

impl DialRotation {
    // Basic ahh constructor because habit
    pub fn new(rotation: usize) -> Self {
        Self { rotation, zero_crossings: 0 }
    }

    // Check if we're at zero and increment the counter
    fn zero_check(&mut self) {
        if self.rotation == 0 {
            self.zero_crossings += 1;
        }
    }

    pub fn rotate_left(&mut self) {
        self.rotation = {
            if self.rotation == 0 { 99 } // If we're at 0, going one left means going to 99
            else { self.rotation - 1 } // If we're not at 0, we just subtract one to go left
        };
        self.zero_check(); // Check if we're at zero
    }

    pub fn rotate_right(&mut self) {
        self.rotation = {
            if self.rotation == 99 { 0 } // If we're at 99, going one right would reset us to 0
            else { self.rotation + 1 }  // If we're not at 99, we just add one to go right
        };
        self.zero_check(); // Check if we're at zero
    }

    // A helper to rotate multiple times
    pub fn turn_nob(&mut self, ops: NobOperation) {
        for _i in 0..ops.delta {
            match ops.direction {
                Direction::Clockwise => self.rotate_right(),
                Direction::Counterclockwise => self.rotate_left(),
            }
            println!("[Dial] {}", self.rotation)
        }
    }
}

fn parse_line(line: &str) -> NobOperation {
    let direction_sym = line.get(..1).unwrap();
    let direction_val_str = line.get(1..).unwrap();
    let direction_val: usize = direction_val_str.parse().expect("Unable to parse rotation");

    println!("({}){}", direction_sym, direction_val);
    
    let direction = match direction_sym {
        "L" => Direction::Counterclockwise,
        _ => Direction::Clockwise
    };

    NobOperation { delta: direction_val, direction }
}

pub fn solve(inital_rotation: usize, instructions: Vec<&str>) -> usize {
    let mut dial: DialRotation = DialRotation::new(inital_rotation);

    for line in instructions {
        let delta = parse_line(line);
        dial.turn_nob(delta);
    }

    dial.zero_crossings
}

pub fn main() {
    let lines = INPUT_FILE.lines().collect();
    let result = solve(50, lines);
    println!("Zero crossings: {}", result)
}