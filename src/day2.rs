use std::cmp::Ordering;

pub fn day2() {
    part1();
    part2();
}

fn part1() -> usize {
    let input = get_input();

    let safe_reports = input
        .iter()
        .filter(|report| is_record_safe(report.as_slice()).is_ok())
        .count();

    println!("DAY2 PART1: {safe_reports}");

    safe_reports
}

fn part2() -> usize {
    let input = get_input();

    let mut count = 0;
    for record in input {
        let is_safe = is_record_safe(&record);
        match is_safe {
            Ok(_) => count += 1,
            Err(idx) => {
                for i in 0..=idx {
                    let mut record = record.clone();
                    record.remove(i);
                    if is_record_safe(&record).is_ok() {
                        count += 1;
                        break;
                    }
                }
            }
        };
    }

    println!("DAY2 PART2: {count}");

    count
}

#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn is_record_safe(record: &[i32]) -> Result<(), usize> {
    let mut direction: Option<Direction> = None;
    let mut last_level: Option<i32> = None;
    for (idx, current_level) in record.iter().enumerate() {
        let Some(last_level) = &mut last_level else {
            last_level = Some(*current_level);
            continue;
        };
        let current_direction = match &current_level.cmp(last_level) {
            Ordering::Less => Direction::Descending,
            Ordering::Greater => Direction::Ascending,
            Ordering::Equal => return Err(idx),
        };
        if let Some(last_direction) = &direction {
            if *last_direction != current_direction {
                return Err(idx);
            }
        } else {
            direction = Some(current_direction);
        }

        let difference = i32::abs(*current_level - *last_level);
        if (1..=3).contains(&difference) {
            *last_level = *current_level;
            continue;
        }
        return Err(idx);
    }
    Ok(())
}

fn get_input() -> Vec<Vec<i32>> {
    const INPUT: &str = include_str!("../inputs/day2.txt");
    INPUT
        .split("\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|x| !x.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    #[test]
    fn part1_test() {
        assert_eq!(591, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(621, part2());
    }
}
