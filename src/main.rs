use sieve_of_eratosthenes::sieves::segmented::sieve_segmented;

fn main() {
    let n = 1_000_000_000;
    println!(
        "Number of primes up to {}: {:?}",
        n,
        sieve_segmented(n).len()
    );
}
