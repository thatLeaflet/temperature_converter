use std::io;

fn main() {

    // Prompts the user to enter a number,
    // ensures that the number is a float,
    // and stores value in original_value
    println!("Enter the value to be converted.");
    let original_value = {
        loop {
            let mut original_value = String::new();

            io::stdin()
                .read_line(&mut original_value)
                .expect("Failed to read line.");

            let original_value: f64 = match original_value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number. Try again.");
                    continue;
                }
            };
            break original_value;
        }
    };
    //println!("Original_value: {}", original_value);

    // Prompts the user to enter the new unit,
    // ensures that the input is a character,
    // and stores the unit in new_unit
    println!("Enter the unit to be converted to.");
    let new_unit = {
        loop {
            let mut new_unit = String::new();

            io::stdin()
                .read_line(&mut new_unit)
                .expect("Failed to read line.");

            let new_unit: char = match new_unit.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not an option. Try again.");
                    continue;
                }
            };
        break new_unit;
        }
    };
    //println!("New_unit: {}", new_unit);
    
    // Performs and prints the conversion
    match new_unit {
        'c' => println!("Converted from F to C: {}", convert_to_c(original_value)),
        'f' => println!("Converted from C to F: {}", convert_to_f(original_value)),
        _ => println!("Catch-all"), // this does nothing since we are already ensuring that it is 'c' or 'f'
    }
}

// Converts from Celsius to Fahrenheit
fn convert_to_f(x: f64) -> f64 {
    (x * 9.0/5.0) + 32.0
}

// Converts from Fahrenheit to Celsius
fn convert_to_c(x: f64) -> f64 {
    (x - 32.0) * 5.0/9.0
}
