use core::panic;

use encryption::include_encrypted_string;

pub fn day9() {
    part1();
    part2();
}

fn part1() -> usize {
    let vals = get_input(INPUT);
    let mut empties = vals
        .iter()
        .enumerate()
        .filter(|(_, val)| matches!(val, Val::Empty))
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    empties.sort();
    let moves_required = empties.len()
        - vals
            .iter()
            .rev()
            .take(empties.len())
            .filter(|x| matches!(x, Val::Empty))
            .count();

    let mut elems_to_move = vals
        .iter()
        .rev()
        .filter(|x| !matches!(x, Val::Empty))
        .take(moves_required);

    let mut vals = vals.clone();
    for v in vals.iter_mut() {
        if matches!(v, Val::Empty) {
            let Some(next) = elems_to_move.next() else {
                break;
            };
            *v = next.clone();
        }
    }
    let total: usize = vals
        .iter()
        .enumerate()
        .take(vals.len() - empties.len())
        .map(|(idx, x)| {
            let Val::Val(val) = x else {
                panic!();
            };
            idx * *val
        })
        .sum();

    println!("DAY9 PART1 {total}");

    total
}

fn part2() -> usize {
    let input = get_input(INPUT).into_iter().enumerate().collect::<Vec<_>>();
    let input = input
        .chunk_by(|a, b| a.1 == b.1)
        .filter_map(|vals| {
            let (idx, val) = vals.first()?;
            let len = vals.len();
            Some((*idx, len, (*val).clone()))
        })
        .collect::<Vec<_>>();
    let (mut files, mut empties): (Vec<_>, Vec<_>) = input
        .into_iter()
        .partition(|(_, _, val)| !matches!(val, Val::Empty));

    for (file_idx, file_len, _) in files.iter_mut().rev() {
        let Some((empty_idx, empty_len, _)) = empties
            .iter_mut()
            .find(|(empty_idx, empty_len, _)| empty_idx < file_idx && empty_len >= file_len)
        else {
            continue;
        };
        *file_idx = *empty_idx;
        *empty_idx = *file_idx + *file_len;
        *empty_len -= *file_len;
    }

    let total: usize = files
        .into_iter()
        .map(|(idx, len, val)| {
            let Val::Val(val) = val else { unreachable!() };
            (idx..idx + len).map(|i| i * val).sum::<usize>()
        })
        .sum();

    println!("DAY9 PART2 {total}");

    total
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Val(usize),
    Empty,
}

fn get_input(input: &str) -> Vec<Val> {
    let input = input.trim().chars().collect::<Vec<_>>();
    let vals = input
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
        .collect::<Vec<_>>();

    vals
}

const INPUT: &str = include_encrypted_string!("inputs/day9.enc");

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(6242766523059, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(2858, part2());
    }
}
