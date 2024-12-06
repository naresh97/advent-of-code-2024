use encryption::include_encrypted_string;

pub fn day1() {
    part1();
    part2();
}

fn part1() -> i32 {
    let (mut a, mut b) = get_input();
    a.sort();
    b.sort();
    let total_distance: i32 = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| {
            let distance = a - b;
            i32::abs(distance)
        })
        .sum();
    println!("DAY1 PART1: {total_distance}");
    total_distance
}

fn part2() -> usize {
    let (a, b) = get_input();
    let similarity: usize = a
        .iter()
        .map(|a| b.iter().filter(|b| **b == *a).count() * *a as usize)
        .sum();
    println!("DAY1 PART2: {similarity}");
    similarity
}

fn get_input() -> (Vec<i32>, Vec<i32>) {
    const INPUT: &str = include_encrypted_string!("inputs/day1.enc");
    let input: (Vec<_>, Vec<_>) = INPUT
        .lines()
        .filter_map(|lines| {
            let mut lines = lines.split_ascii_whitespace();
            let a = lines.next().and_then(|a| a.parse::<i32>().ok())?;
            let b = lines.next().and_then(|b| b.parse::<i32>().ok())?;
            Some((a, b))
        })
        .unzip();
    input
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!(2264607, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(19457120, part2());
    }
}
