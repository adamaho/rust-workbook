use std::io;

fn main() {
    let mut cli_temp = String::new();
    let mut cli_type = String::new();

    println!("Enter the temperature: ");

    io::stdin()
        .read_line(&mut cli_temp)
        .expect("Failed to read temperature");

    println!("Enter the type of temperature to convert to (c,f): ");

    io::stdin()
        .read_line(&mut cli_type)
        .expect("Failed to read temperature to convert to");

    let temp: f32 = cli_temp.trim().parse().expect("Please type a number!");

    let converted_temp = convert_temp(temp, cli_type.trim());

    println!("The temperature is: {}", converted_temp);
}

fn convert_temp(temp: f32, desired_type: &str) -> f32 {
    println!("{} {}", temp, desired_type);

    match desired_type {
        "f" => temp * 1.8 + 32.0,
        "c" => (temp - 32.0) / 1.8,
        _ => 0.0
    }
}
