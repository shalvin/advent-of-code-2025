use std::cmp::max;

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mult,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "+" => Op::Add,
            "*" => Op::Mult,
            _ => unimplemented!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let first_row = input.lines().next().unwrap();
    let items = first_row.split_whitespace();
    let num_columns = items.count();

    // println!("Columns: {}", num_columns);

    let column_iters = (0..num_columns).map(|i| {
        input
            .lines()
            .rev()
            .flat_map(|line| line.split_whitespace())
            // .inspect(|i| println!("iter: '{}'", i))
            .skip(i)
            .step_by(num_columns)
    });

    let result = column_iters.fold(0, |sum, mut col| {
        let op = Op::from(col.next().unwrap());

        sum + match op {
            Op::Add => col
                // .inspect(|val| println!("add: '{}'", val))
                .fold(0u64, |acc, val| acc + val.parse::<u64>().unwrap()),
            Op::Mult => col
                // .inspect(|val| println!("mult: '{}'", val))
                .fold(1u64, |acc, val| acc * val.parse::<u64>().unwrap()),
        }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input_lines_iter = input.lines();
    let last_row_reverse: String = input_lines_iter
        .next_back()
        .unwrap()
        .chars()
        .rev()
        .collect();
    let colum_widths: Vec<(usize, Op)> = last_row_reverse
        .split_inclusive(|c: char| !c.is_whitespace())
        .map(|col| {
            let col_size = max(col.len() - 1, 1);
            let op = Op::from(col.trim());
            (col_size, op)
        })
        .rev()
        .collect();
    let num_columns = colum_widths.len();
    let num_rows = input_lines_iter.clone().count();

    // println!("Columns: {}", num_columns);

    let column_iters = (0..num_columns).map(|i| {
        input_lines_iter
            .clone()
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|split| split.parse::<u64>().unwrap())
            })
            // .inspect(|i| println!("iter: '{}'", i))
            .skip(i)
            .step_by(num_columns)
    });

    let result = column_iters.enumerate().fold(0, |sum, (i, col)| {
        let (width, op) = colum_widths[i];

        // iterate over strings instead of numbers...
        col.cycle().take(width * num_rows).max
        sum + match op {
            Op::Add => col
                .enumerate()
                // .inspect(|val| println!("add: '{}'", val))
                .fold(0u64, |acc, (i, val)| {
                    acc + (val * 10u64.pow(num_rows as u32 - 1))
                }),
            Op::Mult => col
                // .inspect(|val| println!("mult: '{}'", val))
                .fold(1u64, |acc, val| acc * val),
        }
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
