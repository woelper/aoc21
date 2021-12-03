fn main() {
    let input = include_str!("input")
        .lines()
        // .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut acc: Vec<i32> = vec![0,0,0,0,0,0,0,0,0,0,0,0,];

    for i in input.iter() {
        for (ci, c) in i.chars().enumerate() {
            let d = c.to_digit(10).unwrap();
            acc[ci] += d as i32;
        }
    }

    let max = acc
        .iter()
        .map(|d| {
            if d > &(input.len() as i32 / 2) {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec<_>>()
        .join("");

    let min = acc
        .iter()
        .map(|d| {
            if d > &(input.len() as i32 / 2) {
                "0"
            } else {
                "1"
            }
        })
        .collect::<Vec<_>>().join("");

    let max_dec = isize::from_str_radix(&max, 2).unwrap();
    let min_dec = isize::from_str_radix(&min, 2).unwrap();

    dbg!(max_dec);
    dbg!(min_dec);
    dbg!(min_dec*max_dec);

}
