use std::io;
// C° -> F°: (x°C × 9/5) + 32 = y°F
// F° -> C°: (x°F − 32) × 5/9 = y°C
fn main() {
    println!("Please input your temperature in Farenheit degrees.");

    // Creates a new mutable variable of type `String`
    let mut farenheit = String::new();

    // Reads user input and stores it in the `farenheit` variable
    io::stdin()
        .read_line(&mut farenheit)
        .expect("Failed to read line");

    // Tries to parse the string as a number
    // Stores it again in the same variable, thanks to shadowing,
    // but this time it's immutable
    let farenheit: f64 = farenheit.trim().parse().expect("Failed to parse input");

    // Convert Farenheit to Celsius
    let celsius = (farenheit - 32.0) * 5.0 / 9.0;

    // Prints it to standard output
    println!("{:.2}°F == {:.2}°C", farenheit, celsius);
}
