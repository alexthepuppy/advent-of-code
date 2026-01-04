static INPUT_FILE: &str = include_str!("../../datafiles/Y25/day01-1.txt");

fn get_lines() -> Vec<&'static str> {
    INPUT_FILE.lines().collect()
}

struct DialRotation {
    rotation: u16,
    zero_crossings: u16
}

impl DialRotation {
    pub fn new(rotation: u16, zero_crossings: u16) -> Self {
        Self { rotation, zero_crossings }
    }

    pub fn rotate(&mut self, value: i16) {
        let nv = (self.rotation as i16) + value;
        if nv == 0 || nv == 100 {
            self.zero_crossings += 1;
        }
        self.rotation = (nv % 100) as u16;
    }

    pub fn crossing_count(&self) -> u16 {
        self.zero_crossings
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

    dial.crossing_count()
}

pub fn main() {
    let result = solve();
    println!("Zero crossings: {}", result)
}