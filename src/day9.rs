#![warn(dead_code)]
use std::borrow::BorrowMut;

use encryption::include_encrypted_string;

pub fn day9() {
    part1();
    //part2();
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

    total
}

fn part2() {
    let vals = get_input(EXAMPLE);
    let mut empty_blocks = vals
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .chunk_by(|a, b| a.1 == b.1)
        .filter(|x| matches!(x.first().map(|x| { x.1 }), Some(Val::Empty)))
        .map(|x| {
            let len = x.len();
            let idx = x.first().unwrap().0;
            (idx, len)
        })
        .collect::<Vec<_>>();
    empty_blocks.sort();
    let vals = vals.iter().enumerate().collect::<Vec<_>>();
    let chunks = vals
        .chunk_by(|a, b| a.1 == b.1)
        .collect::<Vec<_>>()
        .into_iter()
        .skip(1)
        .rev()
        .filter(|x| !matches!(x.first().map(|x| x.1), Some(Val::Empty)));
    let mut new_vals = Vec::new();
    for chunk in chunks {
        let mut chunk_len = chunk.len();
        let (chunk_idx, Val::Val(val)) = chunk.first().unwrap() else {
            panic!()
        };
        let (mut chunk_idx, val) = (*chunk_idx, *val);
        let Some((idx, (block_idx, block_len))) =
            empty_blocks
                .iter()
                .copied()
                .enumerate()
                .find(|(_idx, (block_idx, block_len))| {
                    *block_len >= chunk_len && *block_idx < chunk_idx
                })
        else {
            continue;
        };
        println!("Chunk {val} len {chunk_len} inserting into {block_idx}");
        empty_blocks.remove(idx);
        println!("Removing empty block {block_idx} len {block_len}");
        if block_len - chunk_len > 0 {
            empty_blocks.insert(idx, (block_idx + chunk_len, block_len - chunk_len));
            println!(
                "Inserting block {} len {}",
                block_idx + chunk_len,
                block_len - chunk_len
            );
        }

        let mut extended = false;

        if let Some((idx, len)) = empty_blocks
            .iter_mut()
            .find(|(idx, len)| (*idx..=*idx + *len).contains(&chunk_idx))
        {
            println!("Extending {idx},{len} to {idx},{}", *len + chunk_len);
            extended = true;
            *len += chunk_len;
            chunk_len = *len;
            chunk_idx = *idx;
        }

        if let Some((idx, (a_idx, a_len))) =
            empty_blocks
                .iter()
                .copied()
                .enumerate()
                .find(|(_, (idx, _len))| {
                    (chunk_idx..=chunk_idx + chunk_len).contains(idx) && *idx != chunk_idx
                })
        {
            extended = true;
            if let Some(x) = empty_blocks.get_mut(idx - 1) {
                if x.0 == chunk_idx {
                    x.1 += a_len;
                    empty_blocks.remove(idx);
                    println!(
                        "Extending {chunk_idx},{chunk_len} to {chunk_idx},{}",
                        chunk_len + a_len
                    );
                }
            } else {
                empty_blocks.remove(idx);
                empty_blocks.insert(idx, (chunk_idx, chunk_len + a_len));
                println!(
                    "Replacing {a_idx},{a_len} with {chunk_idx},{}",
                    chunk_len + a_len
                );
            }
        }
        if !extended {
            println!("Adding {chunk_idx},{chunk_len}");
            empty_blocks.push((chunk_idx, chunk_len));
        }
        empty_blocks.sort();
    }
    println!("{empty_blocks:?}");
    todo!()
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
const EXAMPLE: &str = "2333133121414131402";

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(6242766523059, part1());
    }

    #[test]
    fn part2_test() {
        part2();
    }
}
