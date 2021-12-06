use std::{fmt::Debug, str::FromStr};
use rayon::prelude::*;
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

fn sim(fish: &Vec<LanternFish>) -> Vec<LanternFish> {
    fish
    .par_iter()
    .map(|f| f.process())
    .flat_map(|f| f)
    .collect()
}

fn process_days(fish: &Vec<usize>, num_days: usize) -> usize {
    let mut fish = fish.clone();
    for _d in 0..num_days {
        let zeroes = fish[0];
        fish[0] = 0;
        fish.rotate_left(1);
        fish[6] += zeroes;
        fish[8] += zeroes;
    }

    fish.iter().sum()
}

fn main() {
    let mut input: Vec<LanternFish> = include_str!("test_input")
        .split(",")
        .map(|l| LanternFish {
            timer: l.parse().unwrap(),
        })
        .collect();

        
        for _i in 0..80 {
            input = sim(&input);
        }
        dbg!("step", &input.len());

        // part 2 - just simulating it takes too much time
        let input: Vec<usize> = include_str!("input").split(",").map(|c| c.parse().unwrap()).fold(vec![0; 9], |mut acc, v: usize| {
            acc[v] += 1;
            acc
        });

        let x = process_days(&input, 256);
        dbg!(x);
}
