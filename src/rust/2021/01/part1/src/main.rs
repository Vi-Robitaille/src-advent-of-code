
use std::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../../input.txt")).unwrap();

    let i32_inp = input
    .lines()
    .map(|line| {
        line.parse::<i32>()
            .expect("")})
    .collect::<Vec<_>>();

    let mut sum: i32 = 0;
    for i in 1..=i32_inp.len() - 1 { 
        if i32_inp[i - 1] < i32_inp[i] {
            sum = sum + 1;
        }
    }
    println!("The result is: {:?}", sum)
}
