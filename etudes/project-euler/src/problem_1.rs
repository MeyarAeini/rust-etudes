//<p>If we list all the natural numbers below $10$ that are multiples of $3$ or $5$, we get $3, 5, 6$ and $9$. The sum of these multiples is $23$.</p>
//<p>Find the sum of all the multiples of $3$ or $5$ below $1000$.</p>
//

fn get_sum_multiples_3_or_5_less_than(limit: usize) -> usize {
    let mut sum: usize = 0;

    for x in 3..limit {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    sum
}

pub fn print_result() {
    println!(
        "problem 1 result is {}",
        get_sum_multiples_3_or_5_less_than(1000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_for_less_than_10() {
        assert_eq!(get_sum_multiples_3_or_5_less_than(10), 23);
    }
}
