extern crate core;

use std::io::{stdin, stdout, Write, BufRead};

fn main() {
    println!("Sieve of Eratostenes");
    print!("Input inclusive bound k for the sieve to look for primes in [0, k] k: ");
    let mut input= String::new();
    let _ = stdout().flush();
    let stdin = stdin();

    match stdin.lock().read_line(&mut input) {
        Ok(result) => println!("Read result: {}", result),
        Err(message) => panic!(message),
    };

    let input_int = match input[..input.len()-1].parse::<isize>() {
        Ok(value) => value,
        Err(message) => panic!(message),
    };

    println!("Parsed int: {}", input_int.to_string());

    sieve(input_int);

}

fn sieve(bound: isize) {
    let mut definitely_not_primes = Vec::with_capacity(bound as usize);

    for number in 2..bound {
        let mut no_divisors: isize = 0;

        if definitely_not_primes.contains(& number) {
            continue;
        }

        for divisor in 1..number+1 {
            if number % divisor == 0 {
                no_divisors = no_divisors + 1;
            }
        }

        if no_divisors == 2 {
            println!("{}", number.to_string());
            let mut power = number*number;

            while power < bound {
                definitely_not_primes.push(power);
                power = power * number;
            }
        }
    }
}