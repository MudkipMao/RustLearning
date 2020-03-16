use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit and I'll convert it to Celsius!");
    let fahrenheit_temp: u32 = loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let _input: u32 = match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let celsius_temp = (fahrenheit_temp as f32 - 32.0) * 5.0 / 9.0;
    println!("Temperature is {}˚ C!", celsius_temp);
}
