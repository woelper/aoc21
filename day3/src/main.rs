fn o2gen(input: Vec<&str>, pos: usize) -> Vec<&str> {
    let column_count = input.iter()
        .map(|l| l.chars().nth(pos).unwrap().to_digit(10).unwrap())
        .filter(|x| x == &1)
        .count();
    // dbg!(&input, input.len(), &pos, &column_count, );
    let most_common = if column_count > input.len()/2 || column_count == input.len()/2 {
        1
    } else  {0};
        // .collect::<Vec<_>>();

    let filter = input.into_iter()
        .filter(|l| l.chars().nth(pos).unwrap().to_digit(10).unwrap() == most_common)
        .collect::<Vec<_>>();
    filter
}

fn co2gen(input: Vec<&str>, pos: usize) -> Vec<&str> {
    let column_count = input.iter()
        .map(|l| l.chars().nth(pos).unwrap().to_digit(10).unwrap())
        .filter(|x| x == &1)
        .count();
    // dbg!(&input, input.len(), &pos, &column_count, );
    let most_common = if column_count < input.len()/2 || column_count != input.len()/2 {
        1
    } else  {0};
        // .collect::<Vec<_>>();

    let filter = input.into_iter()
        .filter(|l| l.chars().nth(pos).unwrap().to_digit(10).unwrap() == most_common)
        .collect::<Vec<_>>();
    filter
}

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

    // dbg!(max_dec);
    // dbg!(min_dec);
    // dbg!(min_dec*max_dec);

    let mut input = include_str!("test_input")
        .lines()
        // .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();


    // for i in 0..input[0].chars().count() {
    //     let o2 = o2gen(input.clone(), i);
    //     dbg!(o2);

    // }

    let mut i = 0;
    let mut o2_rating = 0;
    loop {
        input = o2gen(input.clone(), i);
        i += 1;
        if input.len() == 1 {
            dbg!(&input);
            
            o2_rating = isize::from_str_radix(input[0], 2).unwrap();
            break;
        }
        
    }

    dbg!(o2_rating);

    let mut co2_rating = 0;

    let mut input = include_str!("test_input")
        .lines()
        // .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;
    loop {
        input = co2gen(input.clone(), i);
        dbg!(&input, i);
        i += 1;
        if input.len() == 1 {

            co2_rating = isize::from_str_radix(input[0], 2).unwrap();
            break;
        }
    }

    dbg!(co2_rating);

}
