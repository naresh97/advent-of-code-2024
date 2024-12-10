use encryption::include_encrypted_string;

pub fn day7() {
    part1();
    part2();
}

#[derive(Clone, Copy)]
struct UseConcat(bool);

fn part1() -> usize {
    let equations = get_inputs(INPUT);
    let mut total = 0;
    for equation in equations {
        let operands = equation.operands.iter().rev().copied();
        let target = equation.target;
        if check(operands.clone(), target, UseConcat(false)) {
            total += equation.target;
        }
    }
    println!("DAY7 PART2: {total}");
    total
}

fn part2() -> usize {
    let equations = get_inputs(INPUT);
    let mut total = 0;
    for equation in equations {
        let operands = equation.operands.iter().rev().copied();
        let target = equation.target;
        if check(operands.clone(), target, UseConcat(true)) {
            total += equation.target;
        }
    }
    println!("DAY7 PART2: {total}");
    total
}

fn check(
    ori_operands: impl Iterator<Item = usize> + Clone,
    mut target: usize,
    use_concat: UseConcat,
) -> bool {
    let operands = ori_operands.clone();
    let len = ori_operands.clone().count();
    for (i, current_operand) in operands.enumerate() {
        if target == current_operand && i + 1 == len {
            return true;
        }
        let is_multiple = target % current_operand == 0;
        let is_concat = ends_with(target, current_operand) && use_concat.0;

        if is_multiple {
            let mut operands = ori_operands.clone();
            let current_operand = operands.nth(i).unwrap();
            let new_target = target / current_operand;
            if check(operands.clone(), new_target, use_concat) {
                return true;
            }
        }
        if is_concat {
            let mut operands = ori_operands.clone();
            let current_operand = operands.nth(i).unwrap();
            let new_target = deconcat(target, current_operand);
            if check(operands.clone(), new_target, use_concat) {
                return true;
            }
        }

        target = match target.checked_sub(current_operand) {
            Some(t) => t,
            None => return false,
        };
    }
    false
}

fn deconcat(mut lhs: usize, rhs: usize) -> usize {
    lhs /= 10_usize.pow(digits(rhs) as u32);
    lhs
}

fn ends_with(lhs: usize, rhs: usize) -> bool {
    lhs % 10_usize.pow(digits(rhs) as u32) == rhs
}

fn digits(mut num: usize) -> usize {
    let mut digits = 0;
    while num != 0 {
        digits += 1;
        num /= 10;
    }
    digits
}

#[derive(Debug)]
struct Equation {
    target: usize,
    operands: Vec<usize>,
}

fn get_inputs(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(":");
            let target = split.next()?;
            let target = target.parse::<usize>().ok()?;
            let operands = split.next()?;
            let operands = operands
                .split_ascii_whitespace()
                .filter_map(|op| op.parse::<usize>().ok())
                .collect::<Vec<_>>();
            Some(Equation { target, operands })
        })
        .collect::<Vec<_>>()
}

const INPUT: &str = include_encrypted_string!("inputs/day7.enc");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(303766880536, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(337041851384440, part2());
    }

    #[test]
    fn digits_test() {
        assert_eq!(3, digits(123));
        assert_eq!(5, digits(55555));
    }

    #[test]
    fn ends_with_test() {
        assert!(ends_with(12345, 45));
        assert!(ends_with(12345, 5));
        assert!(ends_with(12345, 345));
        assert!(!ends_with(12345, 111));
    }
}
