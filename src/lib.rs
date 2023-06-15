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
    let segment_len: u64 = (n as f64).sqrt() as u64;
    let mut lo = 2;
    // fixed-length sieve
    let mut is_prime = vec![true; segment_len as usize];
    let mut primes = sieve_original(lo + segment_len - 1);
    lo += segment_len;
    while lo <= n {
        let hi = std::cmp::min(lo + segment_len - 1, n);
        for p in &primes {
            if p * p > hi {
                break;
            }
            let mut j = (lo / p) * p;
            if j < lo {
                j += p;
            }
            while j <= hi {
                is_prime[(j - lo) as usize] = false;
                j += p;
            }
        }
        is_prime.iter().enumerate().for_each(|(i, &is_prime)| {
            let j = i as u64 + lo;
            if is_prime && j <= hi {
                primes.push(j);
            }
        });
        is_prime.iter_mut().for_each(|x| *x = true);
        lo += segment_len;
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
