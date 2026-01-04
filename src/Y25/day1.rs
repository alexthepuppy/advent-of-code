static INPUT_FILE: &str = include_str!("../../datafiles/Y25/day01-1.txt");

fn get_lines() -> Vec<&'static str> {
    INPUT_FILE.lines().collect()
}

// A Struct to represent the dial and its rotation, and encapuslate its logic
struct DialRotation {
    rotation: u16,              // The position of the dial; should be constrained to 0-99 inclusivly.
    pub zero_crossings: u16     // The number of times the dial LANDS ON ZERO at the end of the spinning operation.
}

impl DialRotation {
    // Basic ahh constructor because habit
    pub fn new(rotation: u16, zero_crossings: u16) -> Self {
        Self { rotation, zero_crossings }
    }

    // The actual rotation logic
    pub fn rotate(&mut self, value: i16) {
        println!("[[RT]]");
        let nv = {
            let tv1 = ((self.rotation as i16) + value) % 100; // Convert the current value (uint 0-99) to signed so we can do math with `value` (-99 to 99)
            if tv1 < 0 {
                // (THE FIX?): I had forgotten to abs the value before rotating
                // If below zero, subtract 100 to wrap it around (values are constrained to two chars so we shouldn't need to worry about wrapping around twice)
                (100 - tv1.abs()) as u16
            } else {
                // It's above zero so just return it
                tv1 as u16
            }
        };
        println!("[DBG::NV] {}", nv); // Print out the dial value for debugging

        if nv == 0 { // ONLY IF LANDING AT ZERO, increment the crossing count
            self.zero_crossings += 1;
        }
        println!("[DBG::ZC] {}", self.zero_crossings);

        // Update our rotation
        self.rotation = nv;
    }
}

pub fn solve() -> u16 {
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