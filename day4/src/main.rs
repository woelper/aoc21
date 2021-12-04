use std::{io::BufRead, str::FromStr};

#[derive(Debug)]
struct Board {
    lines: Vec<Vec<(u32,bool)>>,
    completed: bool
}

impl Board {
    fn insert(&mut self, num: u32) {
        for row in 0..self.lines.len() {
            for cell in 0..self.lines[row].len() {
                if self.lines[row][cell].0 == num {
                    self.lines[row][cell].1 = true
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for row in &self.lines {
            if row.iter().all(|x| x.1) {
                return true;
            }
        }

        for column_idx in 0..self.lines[0].len() {
            if (0..self.lines.len()).all(|x| self.lines[x][column_idx].1) {
                return  true;
            }
        }
        
        false
    }

    fn unmarked(&self) -> Vec<u32>{
        let mut u = vec![];
        for l in &self.lines {
            for i in l {
                if !i.1 {
                    u.push(i.0)
                }
            }
        }
        u
    }

    fn display(&self) -> String{
        let mut out = String::default();
        for l in &self.lines {
            out = format!("{}\n{:?}", out, l.iter().map(|x| {
                if x.1 {format!("[{}]",x.0)} else {
                x.0.to_string()}
            
            }).collect::<Vec<_>>().join(" "))
        }
        out
    }
}


fn main() {
    let draw_nums = include_str!("input")
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = include_str!("input")
        .lines()
        .skip(2)
        .collect::<Vec<_>>()
        .join("\n");
    let boards = boards
        .split("\n\n")
        .map(|l| l.replace("  ", " "))
        .map(|l| l.replace("\n ", "\n"))
        .map(|l| {
            if l.starts_with(" ") {
                l.chars().skip(1).collect::<String>()
            } else {
                l
            }
        })
        .collect::<Vec<_>>();


    let mut all_boards = boards.iter()
    .map(|s| {
        s.split("\n")
        .map(|l| l.split(" ").map(|c| (c.parse::<u32>().unwrap(), false)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        
    })
    .collect::<Vec<_>>()
    .into_iter()
    .map(|x| Board {lines: x, completed: false})
    .collect::<Vec<_>>();


    for num in draw_nums {
        println!("drawing {}", num);
        for b in &mut all_boards {
            b.insert(num);
            if b.has_won() && !b.completed{
                println!("{}", b.display());
                let s = b.unmarked().iter().sum::<u32>();
                println!("This board is a winner {} ", s*num);
                b.completed = true;
                // break;
            }
        }
    }

    
}
