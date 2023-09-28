// Sieve of Eratosthenes algorithm implementation in Rust
// Time complexity: O(n log log n)

/*
algorithm Sieve of Eratosthenes is
    input: an integer n > 1.
    output: all prime numbers from 2 through n.

    let A be an array of Boolean values, indexed by integers 2 to n,
    initially all set to true.
    
    for i = 2, 3, 4, ..., not exceeding √n do
        if A[i] is true
            for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
                set A[j] := false

    return all i such that A[i] is true.
*/

use std::io::{self, Write};
use std::process::exit;
use std::time::Instant;

const N: usize = 100000000;

fn sieve_of_eratosthenes () -> Vec<usize> {
    let mut a = vec![true; N + 1];
    let mut primes = Vec::new();

    for i in 2..N {
        if a[i] {
            primes.push(i);

            for j in (i..N).step_by(i) {
                a[j] = false;
            }
        }
    }

    primes
}

fn main() {
    let mut iterations = String::new();

    print!("Type how many iterations to run: ");
    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut iterations)
        .expect("Failed to read line");

    let iterations: u32 = match iterations.trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
    };

    println!("\nCalculating {} prime numbers...\n", N);

    let mut runtimes = Vec::new();

    for i in 0..iterations {
        let now = Instant::now();
        
        let _primes = sieve_of_eratosthenes();

        let elapsed_time = now.elapsed();
        runtimes.push(elapsed_time.as_millis());

        println!("{}st test execution time: {}ms", i + 1, elapsed_time.as_millis());
    }

    let sum: u128 = runtimes.iter().sum();
    let count = runtimes.len() as u128;

    let mean = sum / count;

    println!("\nRuntime meantime: {}ms", mean);
}
