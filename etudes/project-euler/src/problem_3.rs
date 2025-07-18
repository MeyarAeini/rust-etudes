//The prime factors of 13195 are 5, 7, 13 and 29
//What is the largest prime factor of the number 600851475143?
//

struct Primes {
    values: Vec<usize>,
    current: usize,
}

impl Primes {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            current: 1,
        }
    }
}

fn largest_prime_factor(num: usize) -> usize {
    let sqrt = (num as f64).sqrt() as usize;
    let mut p = Primes::new();
    let mut l = 0;
    while p.current <= sqrt {
        if num % p.current == 0 {
            l = p.current;
        }
        p.next();
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

use std::f64;

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current == 2 {
            self.values.push(self.current);
            return Some(self.current);
        } else {
            loop {
                let sqrt = (self.current as f64).sqrt() as usize;
                let mut is_prime = true;
                for item in &self.values {
                    if item > &sqrt {
                        break;
                    }

                    if self.current % item == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    self.values.push(self.current);
                    return Some(self.current);
                }

                self.current += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let mut p = Primes::new();

        assert_eq!(p.next(), Some(2));
        assert_eq!(p.next(), Some(3));
        assert_eq!(p.next(), Some(5));
        assert_eq!(p.next(), Some(7));
        assert_eq!(p.next(), Some(11));
        assert_eq!(p.next(), Some(13));
        assert_eq!(p.next(), Some(17));
        assert_eq!(p.next(), Some(19));
        assert_eq!(p.next(), Some(23));
    }
}
