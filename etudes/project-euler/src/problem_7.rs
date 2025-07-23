//<p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
//<p>What is the $10\,001$st prime number?</p>
//
//

use crate::num::Primes;

fn nth_prime(n: usize) -> Option<usize> {
    //let mut p = Primes::new();
    for (i, p) in Primes::new().enumerate() {
        if i == n - 1 {
            return Some(p);
        }
    }

    None
}

pub fn print_result() {
    println!(
        "Problem 7 result is {}",
        nth_prime(10001).expect("this should have result")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_working() {
        assert_eq!(nth_prime(6), Some(13));
    }
}
