pub fn day5() {
    part1();
    part2();
}

fn part1() -> usize {
    let (rules, updates) = get_input();
    let mut total = 0;
    for update in updates {
        if is_update_correct(&update, &rules).is_ok() {
            let len = update.len() as f32 / 2.;
            let len = len as usize;
            total += update[len];
        }
    }
    println!("DAY5 PART1: {total}");
    total
}

fn part2() -> usize {
    let (rules, updates) = get_input();
    let mut total = 0;
    for mut update in updates {
        if is_update_correct(&update, &rules).is_ok() {
            continue;
        }
        loop {
            match is_update_correct(&update, &rules) {
                Ok(_) => {
                    let len = update.len() as f32 / 2.;
                    let len = len as usize;
                    total += update[len];
                    break;
                }
                Err((page_idx, check_idx)) => {
                    let elem = update.remove(page_idx);
                    update.insert(check_idx, elem);
                }
            }
        }
    }
    println!("DAY5 PART2: {total}");
    total
}

fn is_update_correct(update: &[usize], rules: &[(usize, usize)]) -> Result<(), (usize, usize)> {
    for (page_idx, page) in update.iter().enumerate() {
        let page = *page;
        let rules = rules
            .iter()
            .filter(|(a, _b)| *a == page)
            .map(|(_a, b)| *b)
            .collect::<Vec<_>>();
        for (check_idx, check) in update.iter().enumerate().take(page_idx) {
            if rules.contains(check) {
                return Err((page_idx, check_idx));
            }
        }
    }
    Ok(())
}

fn get_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let position = INPUT.lines().position(|x| x.is_empty()).unwrap();
    let lines = INPUT.lines().collect::<Vec<_>>();
    let rules = lines[..position]
        .iter()
        .filter_map(|rule| {
            let mut rule = rule.split("|");
            let lhs = rule.next()?.parse::<usize>().ok()?;
            let rhs = rule.next()?.parse::<usize>().ok()?;
            Some((lhs, rhs))
        })
        .collect::<Vec<_>>();

    let updates = lines[position..]
        .iter()
        .filter_map(|pages| {
            pages
                .split(",")
                .map(|x| x.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}
const INPUT: &str = include_str!("../inputs/day5.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(5509, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(4407, part2());
    }
}
