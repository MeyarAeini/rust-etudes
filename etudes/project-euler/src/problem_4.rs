//<p>A palindromic number reads the same both ways. The largest palindrome made from the product of two $2$-digit numbers is $9009 = 91 \times 99$.</p>
//<p>Find the largest palindrome made from the product of two $3$-digit numbers.</p>
//
//

fn find_max(f: usize, t: usize) -> usize {
    let mut max = 0;
    for x in (f..=t).rev() {
        for y in (f..=t).rev() {
            let p = x * y;
            if is_plaindromic(&p) && p > max {
                max = p;
            }
        }
    }

    max
}

fn is_plaindromic(num: &usize) -> bool {
    let mut reverse = 0;
    let mut x = num.clone();
    while x > 0 {
        reverse *= 10;
        reverse += x % 10;
        x /= 10;
    }

    return reverse == *num;
}

pub fn print_result() {
println!("the problem 4 result is {}",find_max(100,999));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(find_max(10, 99), 9009);
    }
}
