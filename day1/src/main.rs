
fn main() {
    
    let input: Vec<i32> = include_str!("input").lines().map(|l| l.parse::<i32>().unwrap()).collect();

    let res = input.iter().fold(
        (-1, 0),
        |(sum, last_depth), depth| {
            match depth > &last_depth {
                true => (sum + 1, *depth),
                false => (sum, *depth)
            }
        },
    );

    dbg!("First part", res.1);
    

    // Boring one, iterating in tuples would be cooler
    let mut prev = 0;
    let mut acc = -1;
    for i in 0..input.len() {
        if i == input.len()-2 {
            break;
        }
        let sum = input[i] + input[i+1] + input[i+2];
        if sum > prev {
            acc +=1;
        }
        prev = sum;
    }
    dbg!(acc);


}
