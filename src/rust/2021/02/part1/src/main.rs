
use std::str;

struct Operation {
    pub command: String,
    pub distance: i32
}

fn convert_to_struct(s: &str) -> Operation {
    let split_string = s.split_whitespace().collect::<Vec<&str>>();
    let x = Operation { 
        command: split_string[0].to_string(),
        distance: split_string[1].parse::<i32>().unwrap()
    };
    return x;
}

fn main() {
    let input = str::from_utf8(include_bytes!("../../input.txt")).unwrap();
    
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;

    for i in input.lines() {
        let op_code: Operation = convert_to_struct(i);
        // println!("We are moving {:?} a dist of {:?}", op_code.command, op_code.distance);
        match &op_code.command as &str {
            "forward" => x_pos += op_code.distance,
            "down"    => y_pos += op_code.distance,
            "up"      => y_pos -= op_code.distance,
            _ => println!("Sheit you fucked it.")
        }
    }
    
    println!("{}", x_pos * y_pos);
}
