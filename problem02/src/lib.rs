use std::{cmp::Ordering, fmt::Display, mem::swap, num::NonZeroU16};

use scan_fmt::scan_fmt;
use slab::Slab;

const MAX_NODES: usize = 200; // Maximum number of nodes in the tree

struct Node {
    rank: u16,
    symbol: u8,
    left: Option<NonZeroU16>,
    right: Option<NonZeroU16>,
}

enum Instruction {
    Add {
        id: u16,
        left_rank: u16,
        left_symbol: u8,
        right_rank: u16,
        right_symbol: u8,
    },
    Swap {
        id: u16,
    },
}

fn do_insert(slab: &mut Slab<Node>, root: u16, rank: u16, symbol: u8) -> NonZeroU16 {
    match rank.cmp(&slab[root as usize].rank) {
        Ordering::Equal => {
            unreachable!();
        }
        Ordering::Less => {
            // Insert into the left subtree.
            if let Some(left) = slab[root as usize].left {
                do_insert(slab, left.get(), rank, symbol)
            } else {
                let idx = NonZeroU16::new(slab.insert(Node {
                    rank,
                    symbol,
                    left: None,
                    right: None,
                }) as u16)
                .unwrap();
                slab[root as usize].left = Some(idx);
                idx
            }
        }
        Ordering::Greater => {
            // Insert into the right subtree.
            if let Some(right) = slab[root as usize].right {
                do_insert(slab, right.get(), rank, symbol)
            } else {
                let idx = NonZeroU16::new(slab.insert(Node {
                    rank,
                    symbol,
                    left: None,
                    right: None,
                }) as u16)
                .unwrap();
                slab[root as usize].right = Some(idx);
                idx
            }
        }
    }
}

fn height(slab: &Slab<Node>, root: u16) -> usize {
    let left_height = slab[root as usize]
        .left
        .map_or(0, |l| height(slab, l.get()));
    let right_height = slab[root as usize]
        .right
        .map_or(0, |r| height(slab, r.get()));
    1 + left_height.max(right_height)
}

fn message(slab: &Slab<Node>, root: u16) -> String {
    let mut levels = vec![String::new(); height(slab, root)];
    let mut queue = vec![(root, 0)]; // (node index, level)
    while let Some((node, level)) = queue.pop() {
        if let Some(node_ref) = slab.get(node as usize) {
            levels[level].push(node_ref.symbol as char);
            if let Some(left) = node_ref.left {
                queue.push((left.get(), level + 1));
            }
            if let Some(right) = node_ref.right {
                queue.push((right.get(), level + 1));
            }
        }
    }
    levels
        .into_iter()
        .rev()
        .max_by_key(|s| s.len())
        .unwrap_or_default()
        .chars()
        .rev()
        .collect::<String>()
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

#[inline]
pub fn solve_part1() -> String {
    let mut left_tree_nodes = Slab::new();
    let mut right_tree_nodes = Slab::new();

    let mut lines = include_str!("part1_input.txt").lines().map(|line| {
        scan_fmt!(
            line,
            "ADD id={} left=[{},{}] right=[{},{}]",
            u16,
            u16,
            char,
            u16,
            char
        )
        .unwrap()
    });

    let (_, l_root_rank, l_root_sym, r_root_rank, r_root_sym) = lines.next().unwrap();
    let l_root = left_tree_nodes.insert(Node {
        rank: l_root_rank,
        symbol: l_root_sym as u8,
        left: None,
        right: None,
    }) as u16;
    let r_root = right_tree_nodes.insert(Node {
        rank: r_root_rank,
        symbol: r_root_sym as u8,
        left: None,
        right: None,
    }) as u16;

    for (_id, l_rank, l_sym, r_rank, r_sym) in lines {
        do_insert(&mut left_tree_nodes, l_root, l_rank, l_sym as u8);
        do_insert(&mut right_tree_nodes, r_root, r_rank, r_sym as u8);
    }

    let mut part1 = message(&left_tree_nodes, l_root);
    part1.push_str(message(&right_tree_nodes, r_root).as_str());
    part1
}

#[inline]
pub fn solve_part2() -> String {
    let mut left_tree_nodes = Slab::new();
    let mut right_tree_nodes = Slab::new();

    let mut lines = include_str!("part2_input.txt").lines().map(|line| {
        scan_fmt!(
            line,
            "ADD id={} left=[{},{}] right=[{},{}]",
            u16,
            u16,
            char,
            u16,
            char
        )
        .map(|(id, l_rank, l_sym, r_rank, r_sym)| Instruction::Add {
            id,
            left_rank: l_rank,
            left_symbol: l_sym as u8,
            right_rank: r_rank,
            right_symbol: r_sym as u8,
        })
        .or_else(|_| scan_fmt!(line, "SWAP {}", u16).map(|id| Instruction::Swap { id }))
        .unwrap()
    });

    let Instruction::Add {
        id: _,
        left_rank: l_root_rank,
        left_symbol: l_root_sym,
        right_rank: r_root_rank,
        right_symbol: r_root_sym,
    } = lines.next().unwrap()
    else {
        panic!("Expected first line to be an ADD instruction");
    };
    let l_root = left_tree_nodes.insert(Node {
        rank: l_root_rank,
        symbol: l_root_sym,
        left: None,
        right: None,
    }) as u16;
    let r_root = right_tree_nodes.insert(Node {
        rank: r_root_rank,
        symbol: r_root_sym,
        left: None,
        right: None,
    }) as u16;

    let mut nodes_by_id = [(u16::MAX, u16::MAX); MAX_NODES + 1];

    for instruction in lines {
        match instruction {
            Instruction::Add {
                id,
                left_rank: l_rank,
                left_symbol: l_sym,
                right_rank: r_rank,
                right_symbol: r_sym,
            } => {
                let (l_node, r_node) = (
                    do_insert(&mut left_tree_nodes, l_root, l_rank, l_sym),
                    do_insert(&mut right_tree_nodes, r_root, r_rank, r_sym),
                );
                nodes_by_id[id as usize] = (l_node.get(), r_node.get());
            }
            Instruction::Swap { id } => {
                let (l_node_idx, r_node_idx) = nodes_by_id[id as usize];
                if l_node_idx == u16::MAX || r_node_idx == u16::MAX {
                    continue; // Skip if either node doesn't exist
                }
                let l_node = left_tree_nodes.get_mut(l_node_idx as usize).unwrap();
                let r_node = right_tree_nodes.get_mut(r_node_idx as usize).unwrap();
                swap(&mut l_node.symbol, &mut r_node.symbol);
                swap(&mut l_node.rank, &mut r_node.rank);
            }
        }
    }

    let mut part2 = message(&left_tree_nodes, l_root);
    part2.push_str(message(&right_tree_nodes, r_root).as_str());
    part2
}

#[inline]
pub fn solve_part3() -> String {
    let mut nodes = Slab::new();

    let mut lines = include_str!("part3_input.txt").lines().map(|line| {
        scan_fmt!(
            line,
            "ADD id={} left=[{},{}] right=[{},{}]",
            u16,
            u16,
            char,
            u16,
            char
        )
        .map(|(id, l_rank, l_sym, r_rank, r_sym)| Instruction::Add {
            id,
            left_rank: l_rank,
            left_symbol: l_sym as u8,
            right_rank: r_rank,
            right_symbol: r_sym as u8,
        })
        .or_else(|_| scan_fmt!(line, "SWAP {}", u16).map(|id| Instruction::Swap { id }))
        .unwrap()
    });

    let Instruction::Add {
        id: root_id,
        left_rank: l_root_rank,
        left_symbol: l_root_sym,
        right_rank: r_root_rank,
        right_symbol: r_root_sym,
    } = lines.next().unwrap()
    else {
        panic!("Expected first line to be an ADD instruction");
    };
    let l_root = nodes.insert(Node {
        rank: l_root_rank,
        symbol: l_root_sym,
        left: None,
        right: None,
    }) as u16;
    let r_root = nodes.insert(Node {
        rank: r_root_rank,
        symbol: r_root_sym,
        left: None,
        right: None,
    }) as u16;

    let mut nodes_by_id = [(u16::MAX, u16::MAX); MAX_NODES + 1];
    nodes_by_id[root_id as usize] = (l_root, r_root);

    for instruction in lines {
        match instruction {
            Instruction::Add {
                id,
                left_rank: l_rank,
                left_symbol: l_sym,
                right_rank: r_rank,
                right_symbol: r_sym,
            } => {
                let (l_node, r_node) = (
                    do_insert(&mut nodes, l_root, l_rank, l_sym),
                    do_insert(&mut nodes, r_root, r_rank, r_sym),
                );
                nodes_by_id[id as usize] = (l_node.get(), r_node.get());
            }
            Instruction::Swap { id } => {
                let (l_node_idx, r_node_idx) = nodes_by_id[id as usize];
                let (l_node, r_node) = nodes
                    .get2_mut(l_node_idx as usize, r_node_idx as usize)
                    .expect("Both nodes should exist");
                swap(l_node, r_node);
            }
        }
    }

    let mut part3 = message(&nodes, l_root);
    part3.push_str(message(&nodes, r_root).as_str());
    part3
}
