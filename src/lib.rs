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
    let delta: u64 = (n as f64).sqrt() as u64;
    dbg!(delta);
    let mut lo = 2;
    let mut hi = lo + delta;
    let mut s = vec![true; delta as usize];
    let mut primes = sieve_original(hi);
    lo += delta;
    hi += delta;
    while hi <= n + 1 {
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
        lo += delta;
        hi += delta;
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

#[cfg(test)]
mod tests {
    #[test]
    fn primes_up_to_100() {
        let n = 100;
        let primes = super::sieve_segmented(n);
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert_eq!(primes, expected);
    }
    #[test]
    fn num_primes_10_million() {
        let n = 10_000_000;
        let expected_num = 664_579;
        let primes = super::sieve_segmented(n);
        assert_eq!(primes.len(), expected_num);
        let primes = super::sieve_original(n);
        assert_eq!(primes.len(), expected_num);
    }
    #[test]
    fn num_primes_1_billion() {
        let n = 1_000_000_000;
        let expected_num = 50_847_534;
        let primes = super::sieve_segmented(n);
        assert_eq!(primes.len(), expected_num);
    }
}
