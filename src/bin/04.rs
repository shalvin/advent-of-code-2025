use std::{cell::RefCell, rc::Rc};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let paper_map = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect::<Vec<Vec<bool>>>();

    let height = paper_map.len();
    let width = paper_map[0].len();

    let accessible_positions = paper_map
        .iter()
        .enumerate()
        .map(|(column_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(row_index, cell)| {
                    if !cell {
                        None
                    } else {
                        Some(
                            [
                                (-1, -1),
                                (-1, 0),
                                (-1, 1),
                                (0, -1),
                                (0, 1),
                                (1, -1),
                                (1, 0),
                                (1, 1),
                            ]
                            .into_iter()
                            .fold(0, |sum, (x, y)| {
                                let x = row_index as i32 + x;
                                let y = column_index as i32 + y;

                                if y < 0 || y >= height as i32 || x < 0 || x >= width as i32 {
                                    return sum;
                                }

                                if paper_map[y as usize][x as usize] {
                                    sum + 1
                                } else {
                                    sum
                                }
                            }),
                        )
                    }
                })
                .fold(
                    0,
                    |sum, num_adjacent| if num_adjacent < 4 { sum + 1 } else { sum },
                )
        })
        .sum();

    Some(accessible_positions)
}

pub fn part_two(input: &str) -> Option<u64> {
    let paper_map = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect::<Vec<Vec<bool>>>();
    let paper_map = Rc::new(RefCell::new(paper_map));

    let (width, height) = {
        let paper_map_ref = paper_map.borrow();
        (paper_map_ref[0].len(), paper_map_ref.len())
    };

    let mut removed = 0;

    loop {
        let paper_map_ref = paper_map.borrow().clone();

        let remove_list: Vec<(usize, usize)> = paper_map
            .borrow()
            .iter()
            .enumerate()
            .flat_map(move |(column_index, row)| {
                let paper_map_ref = paper_map_ref.clone();
                row.iter()
                    .enumerate()
                    .filter_map(move |(row_index, cell)| {
                        if !cell {
                            None
                        } else {
                            let paper_map_ref = paper_map_ref.clone();
                            let num_adjacent = [
                                (-1, -1),
                                (-1, 0),
                                (-1, 1),
                                (0, -1),
                                (0, 1),
                                (1, -1),
                                (1, 0),
                                (1, 1),
                            ]
                            .into_iter()
                            .fold(0, |sum, (x, y)| {
                                let x = row_index as i32 + x;
                                let y = column_index as i32 + y;

                                if y < 0 || y >= height as i32 || x < 0 || x >= width as i32 {
                                    return sum;
                                }

                                if paper_map_ref[y as usize][x as usize] {
                                    sum + 1
                                } else {
                                    sum
                                }
                            });

                            Some(((row_index, column_index), num_adjacent))
                        }
                    })
                    .filter_map(
                        |(pos, num_adjacent)| if num_adjacent < 4 { Some(pos) } else { None },
                    )
            })
            .collect();

        if remove_list.is_empty() {
            break;
        } else {
            removed += remove_list.len();
            remove_list
                .into_iter()
                .for_each(|(x, y)| paper_map.borrow_mut()[y][x] = false);
        }
    }

    Some(removed as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
