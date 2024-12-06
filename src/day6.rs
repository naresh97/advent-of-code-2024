use std::{collections::HashSet, hash::Hash};

use encryption::include_encrypted_string;

pub fn day6() {
    part1();
    part2();
}

fn part1() -> usize {
    let (map, mut position) = get_input(INPUT);
    let mut positions = HashSet::new();
    loop {
        while facing_type(&map, &position) == MapItem::Obstacle {
            turn(&mut position);
        }
        if facing_type(&map, &position) == MapItem::Outside {
            break;
        }
        move_forward(&mut position);
        positions.insert((position.row, position.col));
    }
    let positions = positions.len();
    println!("DAY6 PART1: {positions}");
    positions
}

fn part2() -> usize {
    let (map, position) = get_input(INPUT);
    let mut obstacles = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, _col) in row.iter().enumerate() {
            let mut map = map.clone();
            let mut position = position.clone();
            map[row_idx][col_idx] = MapItem::Obstacle;
            let mut positions = HashSet::new();
            loop {
                while facing_type(&map, &position) == MapItem::Obstacle {
                    turn(&mut position);
                }
                if facing_type(&map, &position) == MapItem::Outside {
                    break;
                }
                move_forward(&mut position);
                if positions.contains(&position) {
                    obstacles += 1;
                    break;
                }
                positions.insert(position.clone());
            }
        }
    }
    println!("DAY2 PART2: {obstacles}");
    obstacles
}

fn move_forward(position: &mut Position) {
    let Position {
        row,
        col,
        direction,
    } = position;

    match direction {
        Direction::Up => *row = row.saturating_sub(1),
        Direction::Down => *row += 1,
        Direction::Left => *col = col.saturating_sub(1),
        Direction::Right => *col += 1,
    }
}

fn turn(position: &mut Position) {
    position.direction = match position.direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn facing_type(map: &[Vec<MapItem>], position: &Position) -> MapItem {
    let Position {
        row,
        col,
        direction,
    } = position;
    let mut row = *row as i32;
    let mut col = *col as i32;
    match direction {
        Direction::Up => {
            row -= 1;
        }
        Direction::Down => row += 1,
        Direction::Left => col -= 1,
        Direction::Right => col += 1,
    };
    if row < 0 || col < 0 {
        return MapItem::Outside;
    }
    map.get(row as usize)
        .and_then(|row| row.get(col as usize).copied())
        .unwrap_or(MapItem::Outside)
}

#[derive(Clone, Copy, PartialEq)]
enum MapItem {
    Clear,
    Obstacle,
    Outside,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

fn get_input(input: &str) -> (Vec<Vec<MapItem>>, Position) {
    let mut position = None;

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut map = Vec::<Vec<MapItem>>::new();
    map.resize_with(rows, || {
        let mut v = Vec::new();
        v.resize(cols, MapItem::Clear);
        v
    });

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    map[row_idx][col_idx] = MapItem::Obstacle;
                }
                '^' => {
                    position = Some(Position {
                        row: row_idx,
                        col: col_idx,
                        direction: Direction::Up,
                    });
                }
                '>' => {
                    position = Some(Position {
                        row: row_idx,
                        col: col_idx,
                        direction: Direction::Right,
                    });
                }
                'v' => {
                    position = Some(Position {
                        row: row_idx,
                        col: col_idx,
                        direction: Direction::Down,
                    });
                }
                '<' => {
                    position = Some(Position {
                        row: row_idx,
                        col: col_idx,
                        direction: Direction::Left,
                    });
                }
                _ => (),
            };
        }
    }

    let position = position.unwrap();
    (map, position)
}

const INPUT: &str = include_encrypted_string!("inputs/day6.enc");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        assert_eq!(5067, part1());
    }
    #[test]
    fn part2_test() {
        assert_eq!(1793, part2());
    }
}
