use num::complex::Complex;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    let a = Complex { re: 2.69, im: -6.69 };
    let b = Complex::new(69.69, -96.96);

    let result = a + b;

    println!("Complex numbers: {} + {}", result.re, result.im);

    let some_collection = [1,2,3,4,5,6,7,8,9,10];

    for item in some_collection {
        println!("{}", item);
    }

    println!("{}", some_collection[5]);

    for n in 0..10 {
        if n % 2 == 0 {
            continue;
        }
        println!("Odd Number: {}", n);
    }

    let mut samples = vec![];

    while samples.len() < 10 {
        samples.push(generate_timestamp());
    }

    println!("Collection: {:?}", samples);

}

fn generate_timestamp() -> u128 {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    return time;
}