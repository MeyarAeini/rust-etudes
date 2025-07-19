//<p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
//<p>What is the smallest positive number that is <strong class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></strong> by all of the numbers from $1$ to $20$?</p>

fn smallest_divisible_by(limit: usize) -> Option<usize> {
    for x in limit..usize::MAX {
        let mut is_divisible = true;
        for y in 2..=limit {
            if x % y != 0 {
                is_divisible = false;
                break;
            }
        }

        if is_divisible {
            return Some(x);
        }
    }

    None
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
