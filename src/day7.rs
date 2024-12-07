use std::cmp::Ordering;

use encryption::include_encrypted_string;

pub fn day7() {
    part1();
    part2();
}

fn part1() -> usize {
    let equations = get_inputs(INPUT);
    let mut total = 0;
    'equation_loop: for equation in equations {
        let mut operators = gen_operators(equation.operands.len() - 1, 2);
        loop {
            if matches!(check_target(&operators, &equation), Ordering::Equal) {
                total += equation.target;
                continue 'equation_loop;
            }
            if is_fully_reduced(&operators) {
                break;
            }
            reduce_operators(&mut operators);
        }
    }
    println!("DAY7 PART1: {total}");
    total
}
fn part2() -> usize {
    let equations = get_inputs(INPUT);
    let mut total = 0;
    'equation_loop: for equation in equations {
        let mut operators = gen_operators(equation.operands.len() - 1, 3);
        loop {
            if matches!(check_target(&operators, &equation), Ordering::Equal) {
                total += equation.target;
                continue 'equation_loop;
            }
            if is_fully_reduced(&operators) {
                break;
            }
            reduce_operators(&mut operators);
        }
    }
    println!("DAY7 PART2: {total}");
    total
}

struct Operators {
    len: usize,
    dim: usize,
    val: usize,
}
impl Operators {
    fn gen(len: usize, dim: usize) -> Self {
        let val = dim.pow(len as u32) - 1;
        Self { len, dim, val }
    }
    fn sub(&mut self) {
        self.val -= 1;
    }
    fn get_iter(&self) -> impl Iterator<Item = usize> {
        let mut x = vec![0; self.len];
        let mut tmp = self.val;
        let mut i = 0;
        while tmp != 0 {
            let rem = tmp % self.dim;
            x[i] = rem;
            tmp /= self.dim;
            i += 1;
        }
        x.reverse();
        x.into_iter()
    }
}

fn gen_operators(len: usize, dim: usize) -> Operators {
    Operators::gen(len, dim)
}

fn is_fully_reduced(operators: &Operators) -> bool {
    operators.val == 0
}

fn reduce_operators(operators: &mut Operators) {
    operators.sub();
}

fn check_target(operators: &Operators, equation: &Equation) -> Ordering {
    let Equation { target, operands } = equation;
    let mut operands = operands.iter();

    let mut total = *operands.next().unwrap();

    let mut operators = operators.get_iter();

    for next_operand in operands {
        let sum = match operators.next().unwrap() {
            0 => total + *next_operand,
            1 => total * *next_operand,
            2 => concat(total, *next_operand),
            _ => unreachable!(),
        };
        if sum > *target {
            return Ordering::Greater;
        }
        total = sum;
    }
    total.cmp(target)
}

fn concat(lhs: usize, rhs: usize) -> usize {
    let lhs = 10_usize.pow(digits(rhs) as u32) * lhs;
    lhs + rhs
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
        assert_eq!(11387, part2());
    }

    #[test]
    fn digits_test() {
        assert_eq!(3, digits(123));
        assert_eq!(5, digits(55555));
    }

    #[test]
    fn concat_test() {
        assert_eq!(12345, concat(123, 45));
        assert_eq!(2286, concat(2, 286));
    }
}
