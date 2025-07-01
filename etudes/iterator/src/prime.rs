pub struct Prime {
    found: Vec<u32>,
}

impl Prime {
    pub fn new() -> Self {
        Self { found: Vec::new() }
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found.is_empty() {
            self.found.push(2);
            return Some(2);
        }

        if self.found.len() == 1 {
            self.found.push(3);
            return Some(3);
        }

        let mut current: u32 = *self.found.last().unwrap() + 2;
        loop {
            if current == u32::MAX || current <= 2 {
                return None;
            }

            let mut is_prime = true;
            let sq = { (current as f32).sqrt() as u32 };
            for x in &self.found {
                if *x > sq {
                    break;
                }
                if current % x == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.found.push(current);
                return Some(current);
            }

            current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ten_first_prime() {
        let mut p = Prime::new();
        assert_eq!(p.next(), Some(2));
        assert_eq!(p.next(), Some(3));
        assert_eq!(p.next(), Some(5));
        assert_eq!(p.next(), Some(7));
        assert_eq!(p.next(), Some(11));
        assert_eq!(p.next(), Some(13));
        assert_eq!(p.next(), Some(17));
        assert_eq!(p.next(), Some(19));
        assert_eq!(p.next(), Some(23));
        assert_eq!(p.next(), Some(29));
    }
}
