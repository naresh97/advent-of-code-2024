use encryption::include_encrypted_string;

pub fn day3() {
    part1();
    part2();
}

fn part1() -> usize {
    let chars = INPUT.chars().collect::<Vec<_>>();
    let (total_enabled, total_disabled) = parse(&chars);
    let total = total_enabled + total_disabled;
    println!("DAY3 PART1: {total}");
    total
}

fn part2() -> usize {
    let chars = INPUT.chars().collect::<Vec<_>>();
    let (total, _) = parse(&chars);
    println!("DAY3 PART2: {total}");
    total
}

fn parse(chars: &[char]) -> (usize, usize) {
    let mut current = 0;

    let mut total_enabled = 0;
    let mut total_disabled = 0;

    enum InstructionState {
        Enabled,
        Disabled,
    }
    let mut current_state = InstructionState::Enabled;

    while current != chars.len() {
        let Some(peek) = chars
            .get(current..current + 3)
            .map(|c| c.iter().collect::<String>())
        else {
            break;
        };
        if peek == "mul" {
            current += 3;
            if chars[current] == '(' {
                current += 1;
                let start = current;
                while chars[current].is_ascii_digit() {
                    current += 1;
                }
                if current == start {
                    continue;
                }
                let Ok(lhs) = chars[start..current]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                else {
                    continue;
                };
                if chars[current] != ',' {
                    current += 1;
                    continue;
                }
                current += 1;
                let start = current;
                while chars[current].is_ascii_digit() {
                    current += 1;
                }
                if current == start {
                    continue;
                }
                let Ok(rhs) = chars[start..current]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                else {
                    continue;
                };
                if chars[current] != ')' {
                    current += 1;
                    continue;
                }
                current += 1;
                match current_state {
                    InstructionState::Enabled => total_enabled += lhs * rhs,
                    InstructionState::Disabled => total_disabled += lhs * rhs,
                }
            }
            continue;
        }

        let Some(peek) = chars
            .get(current..current + 4)
            .map(|c| c.iter().collect::<String>())
        else {
            break;
        };
        if peek == "do()" {
            current += 4;
            current_state = InstructionState::Enabled;
            continue;
        }

        let Some(peek) = chars
            .get(current..current + 7)
            .map(|c| c.iter().collect::<String>())
        else {
            break;
        };
        if peek == "don't()" {
            current += 7;
            current_state = InstructionState::Disabled;
            continue;
        }

        current += 1;
    }
    (total_enabled, total_disabled)
}

const INPUT: &str = include_encrypted_string!("inputs/day3.enc");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        assert_eq!(188192787, part1());
    }
    #[test]
    fn part2_test() {
        assert_eq!(113965544, part2());
    }
}
