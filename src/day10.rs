use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use encryption::include_encrypted_string;

pub fn day10() {
    part1();
    part2();
}

fn part1() -> usize {
    let (height_map, map_size) = get_input(INPUT);

    let trailheads = height_map
        .iter()
        .filter(|(_, val)| **val == 0)
        .map(|(coord, _)| *coord)
        .collect::<Vec<_>>();

    let total = trailheads
        .into_iter()
        .map(|start| {
            climb(start, &height_map, map_size)
                .iter()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("DAY10 PART1: {total}");
    total
}

fn part2() -> usize {
    let (height_map, map_size) = get_input(INPUT);

    let trailheads = height_map
        .iter()
        .filter(|(_, val)| **val == 0)
        .map(|(coord, _)| *coord)
        .collect::<Vec<_>>();

    let total = trailheads
        .into_iter()
        .map(|start| climb(start, &height_map, map_size).len())
        .sum();
    println!("DAY10 PART2: {total}");
    total
}

fn climb(coord: Coord, height_map: &HashMap<Coord, usize>, map_size: MapSize) -> Vec<Coord> {
    let Some(coord_height) = height_map.get(&coord).copied() else {
        return Vec::new();
    };

    let neighbours = get_neighbours(coord, height_map, map_size);

    if coord_height == 8 {
        let summits = neighbours
            .iter()
            .filter(|(_, val)| **val == 9)
            .map(|(coord, _)| *coord)
            .collect::<Vec<_>>();
        if !summits.is_empty() {
            return summits;
        }
    }

    neighbours
        .iter()
        .filter(|(_, val)| **val == coord_height + 1)
        .flat_map(|(coord, _)| climb(*coord, height_map, map_size))
        .collect::<Vec<_>>()
}

fn get_neighbours(
    coord: Coord,
    height_map: &HashMap<Coord, usize>,
    map_size: MapSize,
) -> HashMap<Coord, usize> {
    let mut neighbours = HashMap::new();

    let mut add_neighbour = |coord| {
        if let Some(val) = height_map.get(&coord) {
            neighbours.insert(coord, *val);
        }
    };

    if coord.0 > 0 {
        add_neighbour(Coord(coord.0 - 1, coord.1));
    }
    if coord.1 > 0 {
        add_neighbour(Coord(coord.0, coord.1 - 1));
    }
    if coord.0 < map_size.0 - 1 {
        add_neighbour(Coord(coord.0 + 1, coord.1))
    }
    if coord.1 < map_size.1 - 1 {
        add_neighbour(Coord(coord.0, coord.1 + 1))
    }

    neighbours
}

const INPUT: &str = include_encrypted_string!("inputs/day10.enc");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord(usize, usize);

#[derive(Clone, Copy)]
struct MapSize(usize, usize);

fn get_input(input: &str) -> (HashMap<Coord, usize>, MapSize) {
    let mut height_map = HashMap::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, height) in line.chars().enumerate() {
            height_map.insert(
                Coord(row_idx, col_idx),
                height.to_digit(10).unwrap() as usize,
            );
        }
    }

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let map_size = MapSize(rows, cols);

    (height_map, map_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(607, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(1384, part2());
    }
}
