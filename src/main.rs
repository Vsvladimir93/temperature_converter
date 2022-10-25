mod converter;
mod input;

fn main() {
    print_menu();
}

fn print_menu() {
    println!("Select the type of temperature unit to convert:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let input = input::read_int();

    fn read_temperature() -> f32 {
        println!("Enter temperature value:");
        input::read_float()
    }

    match input {
        1 => {
            let temperature = read_temperature();
            let far = converter::convert_celsius_to_fahrenheit(temperature);
            println!("{temperature} °C is equals to {far} °F")
        }
        2 => {
            let temperature = read_temperature();
            let cel = converter::convert_fahrenheit_to_celsius(temperature);
            println!("{temperature} °F is equals to {cel} °C")
        }
        _ => {
            println!("Wrong input number.");
        }
    }
}


