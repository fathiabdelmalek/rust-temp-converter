use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");
    println!("Enter 'C' for Celsius or 'F' for Fahrenheit!");
    let scale: String = get_input().trim().to_uppercase();
    println!("Enter the temperature you want to convert");
    let temp: f32 = get_input().trim().parse().expect("Please type a number!");
    match scale.as_str() {
        "C" => println!("{:.0}°C = {:.2}°F", temp, c_to_f(temp)),
        "F" => println!("{:.2}°F = {:.0}°C", temp, f_to_c(temp)),
        _ => println!("Invalid scale ('C' or 'F')!"),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn c_to_f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
