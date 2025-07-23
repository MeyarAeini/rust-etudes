//<p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
//<p>What is the smallest positive number that is <strong class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></strong> by all of the numbers from $1$ to $20$?</p>

use crate::num;

fn smallest_divisible_by(limit: usize) -> Option<usize> {
    Some(num::lcm_rang(1, limit))
}

pub fn print_result() {
    println!(
        "the problem 5 result is {}",
        smallest_divisible_by(20).expect("this should have result")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_work() {
        assert_eq!(smallest_divisible_by(10), Some(2520));
    }
}
