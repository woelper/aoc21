fn calc_fuel(start: i32, end: i32) -> i32 {
    (start - end).abs()
}

fn calc_fuel_exp(start: i32, end: i32) -> i32 {
    let step = (start - end).abs();
    step * (step + 1) / 2
}

fn main() {
    let mut input: Vec<i32> = include_str!("input")
        .split(",")
        .map(|l| l.parse::<_>().unwrap())
        .collect();

    // so we can easily get min and max fuel
    input.sort();

    let (min, max) = (
        input.first().unwrap().clone(),
        input.last().unwrap().clone(),
    );

    let p1: i32 = (min..max)
        .into_iter()
        .map(|i| input.iter().map(|j| calc_fuel(*j, i)).sum())
        .min()
        .unwrap();

    dbg!("first part", p1);

    // same as above, different fuel calculation
    let p2: i32 = (min..max)
        .into_iter()
        .map(|i| input.iter().map(|j| calc_fuel_exp(*j, i)).sum())
        .min()
        .unwrap();

    dbg!("2nd part", p2);
}
