use std::io;

pub fn read_int() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(n) => n,
        _ => {
            println!("Failed to read number! Enter number again: ");
            read_int()
        }
    }
}

pub fn read_float() -> f32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<f32>() {
        Ok(n) => n,
        _ => {
            println!("Failed to read float number! Enter float number again:");
            read_float()
        }
    }
}