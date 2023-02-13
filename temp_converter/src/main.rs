use std::io;

fn to_fahrenheit(temp_in_celsius: f32) -> f32 {
    temp_in_celsius*(9.0/5.0) + 32.0
}

fn to_celsius(temp_in_fahrenheit: f32) -> f32 {
    (5.0/9.0)*(temp_in_fahrenheit - 32.0)
}

fn main() {
    println!("Temperature Converter!!");
    
    let mut temp_with_unit = String::new();
    println!("Temperature: ");
    io::stdin()
        .read_line(&mut temp_with_unit)
        .expect("Failed to read line");

    let mut temp_parsed = temp_with_unit.split_whitespace();
    let temp = temp_parsed.next().unwrap();
    
    let temp: f32 = temp.parse().expect("Not a number");
    match temp_parsed.next() {
        Some("C") => println!("In Fahrenheit: {}",to_fahrenheit(temp)),
        Some("F") => println!("In Celsius: {}",to_celsius(temp)),
        _ => println!("Invalid unit"),
    }
    //print!("temperature: {temp}");
}
