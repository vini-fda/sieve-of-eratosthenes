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

pub fn sieve_segmented(n: u64) -> Vec<u64> {
    let delta: u64 = 10;
    let mut lo = 2;
    let mut hi = lo + delta;
    let mut s = vec![true; delta as usize];
    let mut primes = sieve_original(hi);
    lo += delta + 1;
    hi += delta + 1;
    while hi <= n {
        for p in &primes {
            let mut j = (lo / p) * p;
            if j < lo {
                j += p;
            }
            while j < hi {
                s[(j - lo) as usize] = false;
                j += p;
            }
        }
        s.iter().enumerate().for_each(|(i, &is_prime)| {
            if is_prime {
                primes.push(lo + i as u64);
            }
        });
        s.iter_mut().for_each(|x| *x = true);
        lo += delta + 1;
        hi += delta + 1;
    }
    if lo <= n {
        let mut s = vec![true; (n - lo + 1) as usize];
        for p in &primes {
            let mut j = (lo / p) * p;
            if j < lo {
                j += p;
            }
            while j <= n {
                s[(j - lo) as usize] = false;
                j += p;
            }
        }
        s.iter().enumerate().for_each(|(i, &is_prime)| {
            if is_prime {
                primes.push(lo + i as u64);
            }
        });
    }
    primes
}
