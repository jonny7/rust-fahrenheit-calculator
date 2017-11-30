use std::io;

fn main() {

    println!("This is my fahrenheit calculator");

    // initially I forgot to make this mut
    // and the following line I forget to include "&mut" in the read_line() call
    // without the above line, I didn't pass a mutable reference and this wouldn't compile
    let mut current_celcius = String::new();

    io::stdin().read_line(&mut current_celcius)
        .expect("Failed to read line");

    let current_celcius: f64 = current_celcius.trim().parse()
        .expect("Please type a decimal");

    let fahrenheit = convert_celcius_to_fahrenheit(current_celcius);

    println!("{} celcius in fahrenheit is: {}", current_celcius, fahrenheit);
}

fn convert_celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}
