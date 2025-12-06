
use std::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../../input.txt")).unwrap();

    let i32_inp = input
    .lines()
    .map(|line| {
        line.parse::<i32>()
            .expect("")
    })
    .collect::<Vec<_>>();
    let sums: Vec<i32> = i32_inp
        .windows(3)
        .map(|vals| vals.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    
    let sum: i32 = sums.windows(2).map(|vals| (vals[0] < vals[1]) as i32).sum();

    println!("The result is: {:?}", sum)
}
