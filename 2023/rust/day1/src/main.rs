use std::io::{self, BufRead};

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let mut input = Vec::new();
    
    for line in stdin.lock().lines() {
        input.push(line.unwrap());      // we want it to panic
    }

    return input
}

fn get_first_num(input: &String) -> Result<u32, &'static str> { 
    for c in input.chars() {
        if c.is_digit(10) {
            return Ok(c.to_digit(10).unwrap() as u32);
        }
    }    

    return Err("No digit found");
}

fn get_last_num(input: &String) -> Result<u32, &'static str> { 
    for c in input.chars().rev() {
        if c.is_digit(10) {
            return Ok(c.to_digit(10).unwrap() as u32);
        }
    }    

    return Err("No digit found");
} 

fn get_coordinates(input_str: &String) -> Result<u32, &'static str> {
    let first_digit = get_first_num(input_str);
    let last_digit = get_last_num(input_str);
    
    return first_digit.and_then(|x| last_digit.map(|y| 10 * x + y));  
}

fn main() {
    let input: Vec<String> = get_input();
    let mut sum: u32 = 0;
    
    input.iter().for_each(|x| { _ = get_coordinates(x).map(|y| sum += y); } );  
    
    println!("Sum: {}", sum);
}
