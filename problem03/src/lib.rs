use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

#[derive(Debug, Clone, Copy)]
struct Snail {
    x0: i64,
    y0: i64,
    period: i64,
}

impl Snail {
    fn new(x0: i64, y0: i64) -> Self {
        let period = (y0 as usize + x0 as usize - 1) as i64;
        Self {
            x0: x0 - 1,
            y0: y0 - 1,
            period,
        }
    }

    fn position_at(&self, t: i64) -> (i64, i64) {
        let x = (self.x0 + t).rem_euclid(self.period);
        let y = (self.y0 - t).rem_euclid(self.period);
        (x + 1, y + 1)
    }
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let mut m0 = m;
    let mut y = 0;
    let mut x = 1;

    if m == 1 {
        return 0;
    }

    let mut a = a % m;
    while a > 1 {
        // q is quotient
        let q = a / m0;
        let mut t = m0;

        // m is remainder now, process same as Euclid's algorithm
        m0 = a % m0;
        a = t;
        t = y;

        y = x - q * y;
        x = t;
    }

    // Make x positive
    if x < 0 {
        x += m;
    }

    x
}

/// Return time when all the snails are at the first row.
fn crt(snails: &[Snail]) -> i64 {
    let r_list = snails.iter().map(|snail| snail.y0 % snail.period).collect::<Vec<_>>();

    let total_period = snails.iter().map(|snail| snail.period).product::<i64>();

    let mut t = 0;
    for (r_i, period_i) in r_list.iter().zip(snails.iter().map(|snail| snail.period)) {
        let m_i = total_period / period_i;
        let m_i_inv = mod_inverse(m_i, period_i);
        t += r_i * m_i * m_i_inv;
    }

    t % total_period
}

#[inline]
pub fn solve_part1() -> impl Display {
    let snails = include_str!("part1_input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(x, y)| {
            let x = x[2..].parse::<i64>().unwrap();
            let y = y[2..].parse::<i64>().unwrap();
            Snail::new(x, y)
        })
        .collect::<Vec<_>>();

    snails
        .into_iter()
        .map(|snail| snail.position_at(100))
        .map(|(x, y)| 100 * y + x)
        .sum::<i64>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let snails = include_str!("part2_input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(x, y)| {
            let x = x[2..].parse::<i64>().unwrap();
            let y = y[2..].parse::<i64>().unwrap();
            Snail::new(x, y)
        })
        .collect::<Vec<_>>();

    crt(&snails)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let snails = include_str!("part3_input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(x, y)| {
            let x = x[2..].parse::<i64>().unwrap();
            let y = y[2..].parse::<i64>().unwrap();
            Snail::new(x, y)
        })
        .collect::<Vec<_>>();

    crt(&snails)
}
