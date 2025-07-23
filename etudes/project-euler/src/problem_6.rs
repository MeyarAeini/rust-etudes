//<p>The sum of the squares of the first ten natural numbers is,</p>
//$$1^2 + 2^2 + ... + 10^2 = 385.$$
//<p>The square of the sum of the first ten natural numbers is,</p>
//$$(1 + 2 + ... + 10)^2 = 55^2 = 3025.$$
//<p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
//<p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>

// (a+b)2 = a2+b2+2ab
// 2ab = (a+b)2-a2-b2;

pub fn print_result() {
    println!(
        "the result for problem 6 is {}",
        diff_square_sum_square(100)
    );
}

fn diff_square_sum_square(max: usize) -> usize {
    let mut result = 0;

    for i in 1..max {
        for j in (i + 1)..=max {
            result += 2 * i * j;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_working() {
        assert_eq!(diff_square_sum_square(10), 2640);
    }
}
