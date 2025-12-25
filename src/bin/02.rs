advent_of_code::solution!(2);

fn is_sequence_repeated_twice(num: u64) -> bool {
    if num / 10 == 0 {
        return false;
    }

    let num = num.to_string();

    // False if odd length
    if num.len() % 2 == 1 {
        return false;
    }

    let split = num.split_at(num.len() / 2);

    split.0 == split.1
}

fn is_repeating_sequence(num: u64) -> bool {
    if num / 10 == 0 {
        return false;
    }

    let num = num.to_string();

    let input_len = num.len();
    let sample_size_range = 1..((input_len / 2) + 1);

    // println!("- Testing number ({}): {}", input_len, num);

    for sample_size in sample_size_range
        .into_iter()
        .filter(|size| input_len.is_multiple_of(*size))
    {
        let sample = &num[0..sample_size];

        let test_indexes = (sample_size..(input_len - sample_size + 1)).step_by(sample_size);
        // println!(
        //     "--- indexes: {:?}",
        //     test_indexes.clone().collect::<Vec<usize>>()
        // );
        let is_repeating = test_indexes.into_iter().all(|index| {
            let test_sample = &num[index..(index + sample_size)];

            // println!(
            //     "For {} Testing: {} == {} [{}, {}]",
            //     num, sample, test_sample, index, sample_size
            // );

            sample == test_sample
        });

        if is_repeating {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_ranges: Vec<(u64, u64)> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|item| item.split_once("-").unwrap())
        .map(|split| {
            (
                split.0.parse::<u64>().unwrap(),
                split.1.parse::<u64>().unwrap(),
            )
        })
        .collect();

    let sum = id_ranges.into_iter().fold(0u64, |acc, e| {
        acc + (e.0..=e.1).fold(0u64, |acc, e| {
            let is_repeating = is_sequence_repeated_twice(e);
            // println!("!!! {} = {}", e, is_repeating);
            acc + match is_repeating {
                true => e,
                false => 0,
            }
        })
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_ranges: Vec<(u64, u64)> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|item| item.split_once("-").unwrap())
        .map(|split| {
            (
                split.0.parse::<u64>().unwrap(),
                split.1.parse::<u64>().unwrap(),
            )
        })
        .collect();

    let sum = id_ranges.into_iter().fold(0u64, |acc, e| {
        acc + (e.0..=e.1).fold(0u64, |acc, e| {
            let is_repeating = is_repeating_sequence(e);
            if is_repeating {
                // println!("!!! {}", e);
            }
            // println!("!!! {} = {}", e, is_repeating);
            acc + match is_repeating {
                true => e,
                false => 0,
            }
        })
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
