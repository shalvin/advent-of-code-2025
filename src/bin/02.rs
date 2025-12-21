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
            acc + match is_sequence_repeated_twice(e) {
                true => e,
                false => 0,
            }
        })
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
