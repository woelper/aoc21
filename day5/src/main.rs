use std::{collections::HashMap, str::FromStr};
#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn fill_straight(&self) -> Vec<(i32, i32)> {
        if self.start.0 == self.end.0 {
            // horizontal

            let r = if self.start.1 < self.end.1 { self.start.1..=self.end.1 } else { self.end.1..=self.start.1 };
            (r)
                .map(|x| (self.start.0, x))
                .collect()
        } else if self.start.1 == self.end.1 {
            // println!("y1 == y2 {:?} {:?}", self.start, self.end);
            let r = if self.start.0 < self.end.0 { self.start.0..=self.end.0 } else { self.end.0..=self.start.0 };

            (r)
                .map(|x| (x, self.start.1))
                .collect()
        } else {
            vec![]
        }
    }

    fn fill_diagonal(&self) -> Vec<(i32, i32)> {
        if self.start.0 == self.end.0 && self.start.1 != self.end.1{
            // horizontal
            println!("Horz line");
            let r = if self.start.1 < self.end.1 { self.start.1..=self.end.1 } else { self.end.1..=self.start.1 };
            (r)
                .map(|x| (self.start.0, x))
                .collect()
        } else if self.start.1 == self.end.1 && self.start.0 != self.end.0{
            println!("Vert line");

            let r = if self.start.0 < self.end.0 { self.start.0..=self.end.0 } else { self.end.0..=self.start.0 };

            (r)
                .map(|x| (x, self.start.1))
                .collect()
        } else {
            println!("Diag Line {:?} {:?}", self.start, self.end);

            // let r = if self.start.0 < self.end.0 { self.start.0..=self.end.0 } else { self.end.0..=self.start.0 };
            // let r2 = if self.start.1 < self.end.1 { self.start.1..=self.end.1 } else { self.end.1..=self.start.1 };
            // if self.start.1 < self.end.1 {
            //     (r).zip(r2.rev())
            //     .collect()
            // } else {
            //     (r).zip(r2)
            //     .collect()

            // }

            let mut xvec = vec![];
            let mut yvec = vec![];
            let mut res = vec![];
            
                if self.start.0 < self.end.0 {
                    println!("x smaller");
                    xvec = (self.start.0..=self.end.0).collect::<Vec<i32>>();
                } else {
                    println!("x larger");

                    xvec = (self.end.0..=self.start.0).rev().collect::<Vec<i32>>();
                }
                if self.start.1 < self.end.1 {
                    println!("y smaller");

                    yvec = (self.start.1..=self.end.1).collect::<Vec<i32>>();
                } else {
                    println!("y larger");

                    yvec = (self.end.1..=self.start.1).rev().collect::<Vec<i32>>();
                }
  
                for (x,i) in xvec.iter().enumerate() {
                    res.push((*i, yvec[x]))
                }

            res
        }
    }
}

impl FromStr for Line {
    type Err = ();
    // parse a string into the enum
    fn from_str(s: &str) -> Result<Line, ()> {
        // take the last val and turn it into an int
        let s1: i32 = s
            .split(" ")
            .nth(0)
            .unwrap()
            .split(",")
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        let s2: i32 = s
            .split(" ")
            .nth(0)
            .unwrap()
            .split(",")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let e1: i32 = s
            .split(" ")
            .nth(2)
            .unwrap()
            .split(",")
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        let e2: i32 = s
            .split(" ")
            .nth(2)
            .unwrap()
            .split(",")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let start = if s1 > e1 || s2 > e2{
            (e1,e2)
        } else {(s1,s2)};

        let end = if s1 > e1 || s2 > e2{
            (s1,s2)
        } else {(e1,e2)};

        // let start = (s1,s2);
        // let end = (e1,e2);
        Ok(Line {
            start,
            end,
        })
    }
}

fn main() {
    let input: Vec<Line> = include_str!("input")
        .lines()
        .map(|l| l.parse::<_>().unwrap())
        .collect();

    // let mut map: HashMap<(i32, i32), i32> = HashMap::default();

    // for line in &input {
    //     // println!("{:?}{:?}, {:?}", line.start, line.end, line.fill_straight());
    //     for coord in line.fill_straight() {
    //         println!("{:?}", coord);
    //         *map.entry(coord).or_default() += 1
    //     }
    // }

    // let c = map.values().filter(|v| **v > 1).count();
    // dbg!("first part", c);


    let mut map: HashMap<(i32, i32), i32> = HashMap::default();

    for line in &input {
        println!("{:?}", line);

        let diag = line.fill_diagonal();
        println!("{:?}{:?} => {:?}", line.start, line.end, diag);
        for coord in diag {
            // println!("{:?}", coord);
            *map.entry(coord).or_default() += 1
        }
    }

    let c = map.values().filter(|v| **v > 1).count();
    dbg!("second part", c);
}
