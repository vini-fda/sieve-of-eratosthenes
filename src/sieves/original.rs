pub fn sieve_original(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let mut primes = vec![true; (n + 1) as usize];
    primes[0] = false;
    primes[1] = false;
    let mut i = 2;
    let mut ii = i * i;
    while ii <= n {
        if primes[i as usize] {
            let mut j = ii;
            while j <= n {
                primes[j as usize] = false;
                j += i;
            }
        }
        i += 1;
        ii = i * i;
    }
    primes
        .iter()
        .enumerate()
        .filter_map(|(p, &is_prime)| if is_prime { Some(p as u64) } else { None })
        .collect::<Vec<_>>()
}
