use std::collections::HashMap;

use encryption::include_encrypted_string;

pub fn day11() {
    part1();
    part2();
}

fn part1() -> usize {
    let input = get_input(INPUT);
    let mut cache = HashMap::new();
    let total = input.iter().map(|num| blink(*num, 0, 25, &mut cache)).sum();
    println!("DAY11 PART1: {total}");
    total
}

fn part2() -> usize {
    let input = get_input(INPUT);
    let mut cache = HashMap::new();
    let total = input.iter().map(|num| blink(*num, 0, 75, &mut cache)).sum();
    println!("DAY11 PART2: {total}");
    total
}

fn blink(
    input: usize,
    current: usize,
    until: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if current == until {
        return 1;
    }
    if let Some(total) = cache.get(&(input, until - current)) {
        return *total;
    }
    if input == 0 {
        let total = blink(1, current + 1, until, cache);
        cache.insert((input, until - current), total);
        total
    } else if let Some((left, right)) = split_digits(input) {
        let left_total = blink(left, current + 1, until, cache);
        let right_total = blink(right, current + 1, until, cache);
        cache.insert((input, until - current), left_total + right_total);
        left_total + right_total
    } else {
        let total = blink(input * 2024, current + 1, until, cache);
        cache.insert((input, until - current), total);
        total
    }
}

fn split_digits(num: usize) -> Option<(usize, usize)> {
    let digits = digits(num);
    if digits % 2 != 0 {
        return None;
    }

    let factor = 10_usize.pow((digits / 2) as u32);

    let left_side = num / factor;
    let right_side = num % factor;

    Some((left_side, right_side))
}

fn digits(mut num: usize) -> usize {
    let mut digits = 0;
    while num != 0 {
        digits += 1;
        num /= 10;
    }
    digits
}

fn get_input(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<_>>()
}

const INPUT: &str = include_encrypted_string!("inputs/day11.enc");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        assert_eq!(235850, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(279903140844645, part2());
    }

    #[test]
    fn split_digits_test() {
        assert_eq!(Some((123, 456)), split_digits(123456));
        assert_eq!(None, split_digits(12345));
    }
}
