fn to_fahrenheit(celsius: i32) -> i32 {
    return (celsius * 9 / 5) + 32
}

fn to_celsius(fahrenheit: i32) -> i32 {
    return (fahrenheit - 32) * 5 / 9
}

fn main() {
    const TEMPERATURE: i32 = 10;

    println!("Convert 10C to F: {}", to_fahrenheit(TEMPERATURE));
    println!("Convert 10F to C: {}", to_celsius(TEMPERATURE));
}
