#![allow(non_snake_case)]

// --- STD Imports ---
use std::fs;


fn getPrimes(count: usize) -> Vec<u128>
{
    let mut primes: Vec<u128> = Default::default();
    if 0 < count {
        primes.push(2);
        'N: for n in 3.. {
            if primes.len() == count {
                break;
            } else {
                for p in primes.iter() {
                    if n % p == 0 {
                        continue 'N;
                    } // if p is not prime
                } // for p in primes
                primes.push(n);
            } // primes.len() < count
        } // for n
    } // 0 < count
    return primes;
}


fn isNotUnique(n: u128, primes: &Vec<u128>) -> bool
{
    for p in primes {
        if (n % p == 0) && ((n / p) % p == 0) {
            return true;
        }
    }
    return false;
}


fn main()
{
    // Oh how much easier this would be with regex, but of course Rust has
    // to go all hipster and drop support of lookarounds ...

    let primes = getPrimes(26);
    let base: u128 = 'a' as u128;
    let idSize: usize = 14;

    match fs::read_to_string("input") {
        Ok(message) => {
            let mut hash: u128 = 1;
            for c in message.chars().take(idSize) {
                match primes.iter().nth((c as u128 - base) as usize) {
                    Some(prime) => hash *= prime,
                    None => {
                        println!("Prime associated with '{}' not found", c);
                        break;
                    },
                } // maybePrime
            } // for first couple chars in message
            for (index, (front, back)) in message.chars().zip(message.chars().skip(idSize)).enumerate() {
                if !isNotUnique(hash, &primes) {
                    println!("{}", index + idSize);
                    break;
                }

                match primes.iter().nth((front as u128 - base) as usize) {
                    Some(prime) => hash /= prime,
                    None => {
                        println!("Prime associated with '{}' not found", front);
                        break
                    },
                }

                match primes.iter().nth((back as u128 - base) as usize) {
                    Some(prime) => {
                            hash *= prime;
                    },
                    None => {
                        println!("Prime associated with '{}' not found", back);
                        break;
                    }
                } // maybePrime
            } // for index, (front, back) in enumerate(zip(message, offsetMessage))
        },
        Err(error) => {
            println!("Failed to parse input: {}", error.to_string());
        } // maybeMessage: Err
    } // maybeMessage
}
