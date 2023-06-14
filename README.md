# prime-sieves

## Mathematical Background

The approximate number of primes up to $n$ is given by the prime number theorem:

$$ \pi(n) \approx \frac{n}{\ln(n)}$$

## Algorithm complexity

| Sieve name         | Time complexity  | Space complexity |
| ------------------ | ---------------- | ---------------- |
| `sieve_original`   | $O(n \log \log n)$ | $O(n)$           |
| `sieve_segmented`  | $O(n \log \log n)$ | $O(\sqrt{n})$    |
