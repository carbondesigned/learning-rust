use std::io;

// create a calculator to convert between fahrenheit and celsius
fn main() {
    println!("Convert between Fahrenheit and Celsius");

    // grab the input from user 
    let mut temp_in_fahrenheit = String::new();
    io::stdin()
        .read_line(&mut temp_in_fahrenheit)
        .expect("Failed to read line.");
    
    // converts input of temp to float
    let temp_in_fahrenheit: f32 = match temp_in_fahrenheit.trim().parse() {
        Ok(temp) => temp,
        Err(_) => 0.0,
    };
    let temp_in_celsius = convert_between_fahrenheit_celsius(temp_in_fahrenheit);
    println!("{}°F is {}°C", temp_in_fahrenheit, temp_in_celsius);
}

/// This function takes a temperature in fahrenheit and returns the temperature in celsius
/// 
/// Arguments:
/// 
/// * `fahrenheit`: f32 - This is the parameter that we're passing into the function.
/// 
/// Returns:
/// 
/// the result of the calculation.
fn convert_between_fahrenheit_celsius(fahrenheit: f32) -> f32 {
    // calculation - (x°F − 32) × 5/9 = 0°C
    return (fahrenheit - 32.0) * 5.0/9.0;
}
