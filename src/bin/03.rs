advent_of_code::solution!(3);

fn find_max_joltage(bank: Vec<u32>) -> u64 {
    // println!("-- Finding for bank: {:?}", bank);

    struct Digits {
        tens: u32,
        ones: u32,
        last_seen: u32,
    }

    fn num(tens: u32, ones: u32) -> u32 {
        tens * 10 + ones
    }

    let mut bank_iter = bank.iter().copied();
    let tens = bank_iter.next().unwrap();
    let ones = bank_iter.next().unwrap();
    let initial_digits = Digits {
        tens,
        ones,
        last_seen: ones,
    };

    let digits = bank_iter.fold(initial_digits, |state, n| {
        let current = num(state.tens, state.ones);
        let visiting = num(state.tens, n);
        let previous = num(state.last_seen, n);

        if previous > current {
            Digits {
                ones: n,
                tens: state.last_seen,
                last_seen: n,
            }
        } else if visiting > current {
            Digits {
                ones: n,
                tens: state.tens,
                last_seen: n,
            }
        } else {
            Digits {
                last_seen: n,
                ..state
            }
        }
    });

    let result = num(digits.tens, digits.ones);
    // println!("  Result: {}", result);

    result as u64
}

fn find_max_joltage_p2<const N: usize>(bank: Vec<u32>) -> u64 {
    // println!("Finding for bank: {:?}", bank);

    fn num(digits: &[u32]) -> u64 {
        digits
            .iter()
            .rev()
            .enumerate()
            .fold(0u64, |sum, (power, digit)| {
                sum + (*digit as u64) * 10u64.pow(power as u32)
            })
    }

    let mut working_digits = bank.iter().take(N).copied().collect::<Vec<u32>>();

    for digit in bank.iter().skip(N) {
        let place_value_shift_candidate = working_digits
            .windows(2)
            .enumerate()
            .find(|(_, window)| window[0] < window[1]);
        if let Some(item) = place_value_shift_candidate {
            working_digits.remove(item.0);
            working_digits.push(*digit);
        } else {
            let first_lowest_digit = working_digits
                .iter()
                .enumerate()
                .min_by_key(|(_, i)| **i)
                .expect("digits to not be empty");
            if *digit >= *first_lowest_digit.1 {
                working_digits.remove(first_lowest_digit.0);
                working_digits.push(*digit);
            }
        }
    }

    let result = num(&working_digits);
    // println!("  = {} ({})", result, working_digits.len());

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(0, |acc, bank| acc + find_max_joltage(bank));

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(0, |acc, bank| acc + find_max_joltage_p2::<12>(bank));

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
