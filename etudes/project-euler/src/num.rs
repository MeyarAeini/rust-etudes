pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcm_rang(a: usize, b: usize) -> usize {
    let mut l = 1;
    let a = if a > 1 { a } else { 1 };
    for x in a..=b {
        l = lcm(x, l);
    }

    l
}
//The prime factors of 13195 are 5, 7, 13 and 29
//What is the largest prime factor of the number 600851475143?
//

pub struct Primes {
    values: Vec<usize>,
    current: usize,
}

impl Primes {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            current: 1,
        }
    }
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
