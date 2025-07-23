//The prime factors of 13195 are 5, 7, 13 and 29
//What is the largest prime factor of the number 600851475143?
//
//

use crate::num::Primes;
fn largest_prime_factor(num: usize) -> usize {
    let sqrt = (num as f64).sqrt() as usize;
    let mut p = Primes::new();
    let mut l = 0;
    while let Some(p) = p.next()
        && p <= sqrt
    {
        if num % p == 0 {
            l = p;
        }
    }

    if l == 0 {
        num
    } else {
        l
    }
}

pub fn print_result() {
    println!(
        "the problem 3 result is {}",
        largest_prime_factor(600_851_475_143)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_working() {
        assert_eq!(largest_prime_factor(13195), 29);
    }
}
