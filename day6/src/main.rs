use std::{fmt::Debug, str::FromStr};

#[derive(Debug, Clone)]
struct LanternFish {
    timer: i32,
}

impl Default for LanternFish {
    fn default() -> Self {
        Self { timer: 8 }
    }
}

impl LanternFish {
    fn process(&self) -> Vec<LanternFish> {
        if self.timer == 0 {
            vec![Self { timer: 6 }, Self::default()]
        } else {
            vec![Self {
                timer: self.timer - 1,
            }]
        }
    }
}

fn main() {
    let mut input: Vec<LanternFish> = include_str!("test_input")
        .split(",")
        .map(|l| LanternFish {
            timer: l.parse().unwrap(),
        })
        .collect();

    dbg!("second part", input);

    for _i in 0..10 {
        let x  = input.clone()
            .iter()
            .map(|f| f.process())
            .flat_map(|f| f)
            .collect::<Vec<LanternFish>>();
    }
}
