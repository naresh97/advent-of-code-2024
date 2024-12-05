pub fn day4() {
    part1();
}

fn part1() -> usize {
    let rows = INPUT
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const FORWARD: [char; 4] = ['X', 'M', 'A', 'S'];
    const BACKWARD: [char; 4] = ['S', 'A', 'M', 'X'];

    let horizontal = rows
        .iter()
        .flat_map(|row| row.windows(4))
        .filter(|x| *x == FORWARD || *x == BACKWARD)
        .count();

    let vertical = rows
        .windows(4)
        .flat_map(|blocks| {
            let iter = blocks[0].iter().zip(blocks[1].iter());
            let iter = iter.zip(blocks[2].iter());
            let iter = iter.zip(blocks[3].iter());
            iter.map(|(((a, b), c), d)| [*a, *b, *c, *d])
        })
        .filter(|x| x.eq(&FORWARD) || x.eq(&BACKWARD))
        .count();

    let backwards_diagonal = rows
        .windows(4)
        .filter_map(|blocks| {
            let mut a = blocks[0].iter();
            let mut b = blocks[1].iter();
            let mut c = blocks[2].iter();
            let mut d = blocks[3].iter();
            a.advance_by(0).ok()?;
            b.advance_by(1).ok()?;
            c.advance_by(2).ok()?;
            d.advance_by(3).ok()?;

            let iter = a.zip(b);
            let iter = iter.zip(c);
            let iter = iter.zip(d);
            Some(iter.map(|(((a, b), c), d)| [*a, *b, *c, *d]))
        })
        .flatten()
        .filter(|x: &[char; 4]| x.eq(&FORWARD) || x.eq(&BACKWARD))
        .count();

    let forward_diagonal = rows
        .windows(4)
        .filter_map(|blocks| {
            let mut a = blocks[0].iter();
            let mut b = blocks[1].iter();
            let mut c = blocks[2].iter();
            let mut d = blocks[3].iter();
            a.advance_by(3).ok()?;
            b.advance_by(2).ok()?;
            c.advance_by(1).ok()?;
            d.advance_by(0).ok()?;

            let iter = a.zip(b);
            let iter = iter.zip(c);
            let iter = iter.zip(d);
            Some(iter.map(|(((a, b), c), d)| [*a, *b, *c, *d]))
        })
        .flatten()
        .filter(|x: &[char; 4]| x.eq(&FORWARD) || x.eq(&BACKWARD))
        .count();

    let total = vertical + horizontal + forward_diagonal + backwards_diagonal;
    println!("DAY4 PART1: {total}");
    total
}

fn part2() -> usize {
    let rows = INPUT
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const FORWARD: [char; 3] = ['M', 'A', 'S'];
    const BACKWARD: [char; 3] = ['S', 'A', 'M'];

    let total = rows
        .windows(3)
        .filter_map(|blocks| {
            let mut a = blocks[0].iter();
            a.advance_by(0).ok()?;
            let mut b = blocks[1].iter();
            b.advance_by(1).ok()?;
            let mut c = blocks[2].iter();
            c.advance_by(2).ok()?;
            let forward_diagonal = a.zip(b).zip(c).map(|((a, b), c)| [*a, *b, *c]);

            let mut a = blocks[0].iter();
            a.advance_by(2).ok()?;
            let mut b = blocks[1].iter();
            b.advance_by(1).ok()?;
            let mut c = blocks[2].iter();
            c.advance_by(0).ok()?;
            let backward_diagonal = a.zip(b).zip(c).map(|((a, b), c)| [*a, *b, *c]);

            Some(forward_diagonal.zip(backward_diagonal))
        })
        .flatten()
        .filter(|(forward, backward)| {
            (forward.iter().eq(&FORWARD) || forward.iter().eq(&BACKWARD))
                && (backward.iter().eq(&FORWARD) || backward.iter().eq(&BACKWARD))
        })
        .count();

    println!("DAY4 PART2: {total}");

    total
}

const INPUT: &str = include_str!("../inputs/day4.txt");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        assert_eq!(2406, part1());
    }
    #[test]
    fn part2_test() {
        assert_eq!(1807, part2());
    }
}
