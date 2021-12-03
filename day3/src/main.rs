use std::str::FromStr;

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Instruction {
    type Err = ();
    // parse a string into the enum
    fn from_str(s: &str) -> Result<Instruction, ()> {
        // take the last val and turn it into an int
        let val: i32 = s.split(" ").last().unwrap().parse().unwrap();

        if s.starts_with("forward") {
            Ok(Instruction::Forward(val))
        } else if s.starts_with("up") {
            Ok(Instruction::Up(val))
        } else if s.starts_with("down") {
            Ok(Instruction::Down(val))
        } else {
            Err(())
        }
    }
}

fn main() {
    let input: Vec<Instruction> = include_str!("input")
        .lines()
        .map(|l| l.parse::<_>().unwrap())
        .collect();

    let res = input
        .iter()
        .fold((0, 0), |(x, y), instruction| match instruction {
            Instruction::Forward(val) => (x + val, y),
            Instruction::Up(val) => (x, y - val),
            Instruction::Down(val) => (x, y + val),
        });

    dbg!("First part", res.0*res.1);


    let res = input
    .iter()
    .fold((0, 0, 0), |(x, y, aim), instruction| match instruction {
        Instruction::Forward(val) => (x + val, y+aim*val, aim),
        Instruction::Up(val) => (x, y, aim-val),
        Instruction::Down(val) => (x, y, aim+val),
    });

    dbg!("second part", res.0*res.1);
}
