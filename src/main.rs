use sieve_of_eratosthenes::sieve_original;

fn main() {
    let n = 10000;
    println!("Primes up to {}: {:?}", n, sieve_original(n));
}
