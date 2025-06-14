use std::fmt::Display;

use rayon::prelude::*;

fn eni_part1(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut score = 1;
    let mut l = Vec::new();
    for _ in 0..exp {
        score *= n;
        score %= mod_;
        l.push(score);
    }
    l.reverse();
    l.into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn eni_part2(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut score = 1;
    let mut l = Vec::new();

    let mut last_seen_at = vec![usize::MAX; mod_ as usize];

    while l.len() < exp as usize {
        score *= n;
        score %= mod_;

        l.push(score);

        if last_seen_at[score as usize] != usize::MAX {
            let cycle_start = last_seen_at[score as usize];
            let cycle_length = l.len() - cycle_start - 1;

            l.pop();
            let looping_bit = l[cycle_start..].to_vec();
            let remaining = exp as usize - l.len();

            let padding = if cycle_length < 5 {
                cycle_length * (5 / cycle_length) - 1
            } else {
                0
            };

            l.clear();
            l.extend(
                looping_bit
                    .into_iter()
                    .cycle()
                    .take(padding + cycle_length + remaining % cycle_length),
            );

            break;
        }
        last_seen_at[score as usize] = l.len() - 1;
    }

    l.reverse();
    l.truncate(5);
    l.into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn eni_part3(n: u64, exp: u64, r#mod: u16) -> u64 {
    let mut score = 1;
    let mut prefix_sums = Vec::new();
    let mut sum = 0;

    let mut last_seen_at = vec![u16::MAX; r#mod as usize];

    prefix_sums.push(sum);

    while prefix_sums.len() - 1 < exp as usize {
        score *= n;
        score %= r#mod as u64;
        sum += score;

        prefix_sums.push(sum);

        if last_seen_at[score as usize] != u16::MAX {
            let cycle_start = last_seen_at[score as usize] as usize;
            let cycle_length = prefix_sums.len() - cycle_start - 2;

            let loop_sum = sum - score - prefix_sums[cycle_start];

            let remaining = exp as usize - prefix_sums.len() + 1;
            let full_cycles = remaining / cycle_length;

            sum += loop_sum * full_cycles as u64;
            sum += prefix_sums[cycle_start + remaining % cycle_length + 1]
                - prefix_sums[cycle_start]
                - score;

            break;
        }
        last_seen_at[score as usize] = prefix_sums.len() as u16 - 2;
    }

    sum
}

fn parse_kv(k: &str) -> u64 {
    k[2..].parse::<u64>().expect("Failed to parse key value")
}

struct Row {
    a: u64,
    b: u64,
    c: u64,
    x: u64,
    y: u64,
    z: u64,
    m: u64,
}

impl Row {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split_whitespace();

        let a = parse_kv(parts.next().unwrap());
        let b = parse_kv(parts.next().unwrap());
        let c = parse_kv(parts.next().unwrap());
        let x = parse_kv(parts.next().unwrap());
        let y = parse_kv(parts.next().unwrap());
        let z = parse_kv(parts.next().unwrap());
        let m = parse_kv(parts.next().unwrap());

        Row {
            a,
            b,
            c,
            x,
            y,
            z,
            m,
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();
    (part1, part2, part3)
}

#[inline]
pub fn solve_part3() -> u64 {
    let part3_input = include_str!("part3_input.txt");
    let part3 = part3_input
        .par_lines()
        .map(Row::from_line)
        .map(|row| {
            eni_part3(row.a, row.x, row.m as u16)
                + eni_part3(row.b, row.y, row.m as u16)
                + eni_part3(row.c, row.z, row.m as u16)
        })
        .max()
        .unwrap();
    part3
}

#[inline]
pub fn solve_part2() -> u64 {
    let part2_input = include_str!("part2_input.txt");
    let part2 = part2_input
        .par_lines()
        .map(Row::from_line)
        .map(|row| {
            eni_part2(row.a, row.x, row.m)
                + eni_part2(row.b, row.y, row.m)
                + eni_part2(row.c, row.z, row.m)
        })
        .max()
        .unwrap();
    part2
}

#[inline]
pub fn solve_part1() -> u64 {
    let part1_input = include_str!("part1_input.txt");
    let part1 = part1_input
        .par_lines()
        .map(Row::from_line)
        .map(|row| {
            eni_part1(row.a, row.x, row.m)
                + eni_part1(row.b, row.y, row.m)
                + eni_part1(row.c, row.z, row.m)
        })
        .max()
        .unwrap();
    part1
}
