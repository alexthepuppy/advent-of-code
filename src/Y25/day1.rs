static INPUT_FILE: &str = include_str!("../../datafiles/Y25/day01-1.txt");

fn get_lines() -> Vec<&'static str> {
    INPUT_FILE.lines().collect()
}

// A Struct to represent the dial and its rotation, and encapuslate its logic
struct DialRotation {
    rotation: u16,              // The position of the dial; should be constrained to 0-99 inclusivly.
    pub zero_crossings: usize   // The number of times the dial LANDS ON ZERO at the end of the spinning operation.
}

impl DialRotation {
    // Basic ahh constructor because habit
    pub fn new(rotation: u16, zero_crossings: usize) -> Self {
        Self { rotation, zero_crossings }
    }

    // The actual rotation logic
    pub fn rotate(&mut self, value: i16) {
        println!("[[RT]]");
        println!("[DBG::INTL_V] {}", self.rotation);
        println!("[DBG::VALUE] {}", value);
        let nv = {
            if value < 0 {
                // Rotate left
                let abs_rot = value.abs(); // MUST be between 0-99
                println!("[DBG::ROT_ABS] {}", abs_rot);

                // Apply the transformation
                let mut rv = (self.rotation as i16) - abs_rot;

                // If less than zero after transforming snap around
                if rv < 0 {
                    rv = 100 - rv.abs();
                } 

                (rv % 100) as u16
            } else {
                // Rotate right
                let abs_rot = value.abs(); // MUST be between 0-99
                println!("[DBG::ROT_ABS] {}", abs_rot);

                // Apply the transformation
                let mut rv = ((self.rotation as i16) + abs_rot).abs();

                // If less than zero after transforming snap around
                if rv > 99 {
                    rv = rv - 99;
                } 

                (rv % 100) as u16
            }
        };
        println!("[DBG::NEW_V] {}", nv); // Print out the dial value for debugging

        if nv == 0 { // ONLY IF LANDING AT ZERO, increment the crossing count
            self.zero_crossings += 1;
        }
        println!("[DBG::ZC] {}", self.zero_crossings);

        // Update our rotation
        self.rotation = nv;
    }
}

pub fn solve() -> usize {
    let lines = get_lines();
    let mut dial: DialRotation = DialRotation::new(0, 0);

    for line in lines {
        let direction_sym = line.get(..1).unwrap();
        let direction_val_str = line.get(1..).unwrap();
        let direction_val: u16 = direction_val_str.parse().expect("Unable to parse rotation");

        let mut mdv: i16 = direction_val as i16;
        
        match direction_sym {
            "L" => mdv *= -1,
            _ => {}
        }

        dial.rotate(mdv);
    }

    dial.zero_crossings
}

pub fn main() {
    let result = solve();
    println!("Zero crossings: {}", result)
}