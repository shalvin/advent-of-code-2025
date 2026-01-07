advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines_iter = input.lines();

    let ranges: Vec<(u64, u64)> = lines_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(begin, end)| (begin.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .collect();

    let fresh_ids = lines_iter
        .skip_while(|line| line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|(begin, end)| id >= begin && id <= end))
        .count();

    Some(fresh_ids as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(begin, end)| (begin.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .collect();

    ranges.sort_unstable_by(|(a_lower, _), (b_lower, _)| a_lower.cmp(b_lower));

    let mut index = 0;
    let mut count = 0;
    for (lower, upper) in ranges.into_iter() {
        if index <= lower {
            count += (upper - lower) + 1;
            index = upper;
        } else if index > lower && index < upper {
            count += upper - index;
            index = upper;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
