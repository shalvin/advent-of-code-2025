advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .fold((0, 50), |(count, heading), line| {
                let (direction, clicks) = line.split_at(1);
                let clicks = clicks.parse::<i32>().unwrap();

                let new_heading: i32 = match direction {
                    "L" => heading + (100 - clicks),
                    "R" => heading + clicks,
                    _ => unimplemented!(),
                } % 100;

                (
                    if new_heading == 0 { count + 1 } else { count },
                    new_heading,
                )
            })
            .0 as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .fold((0, 50), |(count, heading), line| {
                let (direction, clicks) = line.split_at(1);
                let clicks = clicks.parse::<i32>().unwrap();
                let mut wraps = clicks.abs() / 100;
                let clicks = clicks % 100;

                let new_heading: i32 = match direction {
                    "L" => heading + (100 - clicks),
                    "R" => heading + clicks,
                    _ => unimplemented!(),
                };

                wraps += match direction {
                    "L" if heading != 0 && (heading - clicks <= 0) => 1,
                    "R" if new_heading >= 100 => 1,
                    _ => 0,
                };

                // println!(
                //     "{}{} :: H {} H-C {} W {}",
                //     direction,
                //     clicks,
                //     new_heading % 100,
                //     heading - clicks,
                //     wraps,
                // );

                (count + wraps, new_heading % 100)
            })
            .0 as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let file = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&file);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
