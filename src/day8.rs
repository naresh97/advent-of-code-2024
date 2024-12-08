use std::collections::HashSet;

use encryption::include_encrypted_string;

pub fn day8() {
    part1();
    part2();
}

fn part1() -> usize {
    let (ants, map_size) = get_input(INPUT);
    let mut antinodes = HashSet::new();
    for (i, ant) in ants.iter().enumerate() {
        let mut it = ants.iter().enumerate();
        it.nth(i);
        for (j, other) in it {
            if i == j || ant.1 != other.1 {
                continue;
            }
            if let Some(antinode) = ant.0.extend(&other.0, &map_size) {
                antinodes.insert(antinode);
            }
            if let Some(antinode) = other.0.extend(&ant.0, &map_size) {
                antinodes.insert(antinode);
            }
        }
    }
    let total = antinodes.len();

    println!("DAY8 PART1: {total}");

    total
}

fn part2() -> usize {
    let (ants, map_size) = get_input(INPUT);
    let mut antinodes = HashSet::new();
    for (i, ant) in ants.iter().enumerate() {
        let mut it = ants.iter().enumerate();
        it.nth(i);
        for (j, other) in it {
            if i == j || ant.1 != other.1 {
                continue;
            }
            for x in ant.0.all_extends(&other.0, &map_size) {
                antinodes.insert(x);
            }
            for x in other.0.all_extends(&ant.0, &map_size) {
                antinodes.insert(x);
            }
        }
    }
    let total = antinodes.len();
    println!("DAY8 PART2: {total}");
    total
}

impl Coord {
    fn all_extends(&self, other: &Self, map_size: &MapSize) -> Vec<Coord> {
        let mut a = *self;
        let mut b = *other;

        let mut result = Vec::new();

        if a.0 < map_size.rows && a.1 < map_size.cols {
            result.push(a);
        }

        while let Some(next_node) = a.extend(&b, map_size) {
            result.push(next_node);
            a = b;
            b = next_node;
        }
        result
    }
    fn extend(&self, other: &Self, map_size: &MapSize) -> Option<Coord> {
        let row = other.0 as i64 - self.0 as i64;
        let col = other.1 as i64 - self.1 as i64;
        let row = other.0 as i64 + row;
        let col = other.1 as i64 + col;
        let row: usize = row.try_into().ok()?;
        let col: usize = col.try_into().ok()?;
        if row >= map_size.rows || col >= map_size.cols {
            return None;
        }
        Some(Coord(row, col))
    }
}

const INPUT: &str = include_encrypted_string!("inputs/day8.enc");

struct MapSize {
    rows: usize,
    cols: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Coord(usize, usize);
#[derive(Debug)]
struct Ant(Coord, char);

fn get_input(input: &str) -> (Vec<Ant>, MapSize) {
    let mut ants = Vec::new();
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, elem) in row.chars().enumerate() {
            if elem != '.' {
                let coord = Coord(row_idx, col_idx);
                ants.push(Ant(coord, elem));
            }
        }
    }
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let map_size = MapSize { rows, cols };
    ants.sort_by_key(|x| x.0);
    (ants, map_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(398, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(1333, part2());
    }
}
