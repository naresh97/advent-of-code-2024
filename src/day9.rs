#![warn(dead_code)]
use encryption::include_encrypted_string;

pub fn day9() {
    part1();
    //part2();
}

fn part1() -> usize {
    let mut input = get_input(INPUT);
    while !is_dense(&input) {
        shift(&mut input);
    }
    let total: usize = input
        .iter()
        .enumerate()
        .filter_map(|(idx, elem)| {
            let Val::Val(val) = elem else {
                return None;
            };
            Some(idx * val)
        })
        .sum();
    println!("DAY9 PART1: {total}");
    total
}

fn is_dense(vals: &[Val]) -> bool {
    !vals.windows(2).any(|chunks| {
        matches!(chunks.first(), Some(Val::Empty)) && matches!(chunks.get(1), Some(Val::Val(..)))
    })
}

fn shift(vals: &mut [Val]) {
    let mut to_shift = Val::Empty;
    for x in vals.iter_mut().rev() {
        if matches!(x, Val::Empty) {
            continue;
        }
        to_shift = x.clone();
        *x = Val::Empty;
        break;
    }
    for x in vals.iter_mut() {
        if !matches!(x, Val::Empty) {
            continue;
        }
        *x = to_shift.clone();
        break;
    }
}

fn part2() {
    todo!()
}

#[derive(Debug, Clone)]
enum Val {
    Val(usize),
    Empty,
}

fn get_input(input: &str) -> Vec<Val> {
    let input = input.trim().chars().collect::<Vec<_>>();
    input
        .chunks(2)
        .enumerate()
        .flat_map(|(idx, chunk)| {
            let mut result = Vec::new();

            let a = chunk[0].to_digit(10).unwrap();
            let mut a = vec![Val::Val(idx); a as usize];
            result.append(&mut a);

            if let Some(b) = chunk.get(1).and_then(|x| x.to_digit(10)) {
                let mut b = vec![Val::Empty; b as usize];
                result.append(&mut b);
            }

            result
        })
        .collect::<Vec<_>>()
}

const INPUT: &str = include_encrypted_string!("inputs/day9.enc");

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(1928, part1());
    }

    #[test]
    fn part2_test() {}
}
