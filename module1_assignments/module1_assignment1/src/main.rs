const FREEZING: f64 = 32.0;
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f-FREEZING) * (5.0/9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 5.0/9.0) + FREEZING
}

fn main() {
    let mut fahrenheit: f64 = FREEZING;
    let mut num = 0;
    println!("Fahrenheit {} to Celsius {:.2}", fahrenheit, fahrenheit_to_celsius(fahrenheit));
    for i in 1..=5 {
        let f = fahrenheit + i as f64;
        println!("Fahrenheit {} to Celsius {:.2}", f, fahrenheit_to_celsius(f));
    }
}
