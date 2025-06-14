use std::{cmp::Ordering, fmt::Display, mem::swap, num::NonZeroUsize};

use scan_fmt::scan_fmt;
use slab::Slab;

const MAX_NODES: usize = 200; // Maximum number of nodes in the tree

struct Node {
    rank: i64,
    symbol: char,
    left: Option<NonZeroUsize>,
    right: Option<NonZeroUsize>,
}

enum Instruction {
    Add {
        id: usize,
        left_rank: i64,
        left_symbol: char,
        right_rank: i64,
        right_symbol: char,
    },
    Swap {
        id: usize,
    },
}

fn do_insert(slab: &mut Slab<Node>, root: usize, rank: i64, symbol: char) -> NonZeroUsize {
    match rank.cmp(&slab[root].rank) {
        Ordering::Equal => {
            unreachable!();
        }
        Ordering::Less => {
            // Insert into the left subtree.
            if let Some(left) = slab[root].left {
                do_insert(slab, left.get(), rank, symbol)
            } else {
                let idx = NonZeroUsize::new(slab.insert(Node {
                    rank,
                    symbol,
                    left: None,
                    right: None,
                }))
                .unwrap();
                slab[root].left = Some(idx);
                idx
            }
        }
        Ordering::Greater => {
            // Insert into the right subtree.
            if let Some(right) = slab[root].right {
                do_insert(slab, right.get(), rank, symbol)
            } else {
                let idx = NonZeroUsize::new(slab.insert(Node {
                    rank,
                    symbol,
                    left: None,
                    right: None,
                }))
                .unwrap();
                slab[root].right = Some(idx);
                idx
            }
        }
    }
}

fn height(slab: &Slab<Node>, root: usize) -> usize {
    let left_height = slab[root].left.map_or(0, |l| height(slab, l.get()));
    let right_height = slab[root].right.map_or(0, |r| height(slab, r.get()));
    1 + left_height.max(right_height)
}

fn message(slab: &Slab<Node>, root: usize) -> String {
    let mut levels = vec![String::new(); height(slab, root)];
    let mut queue = vec![(root, 0)]; // (node index, level)
    while let Some((node, level)) = queue.pop() {
        if let Some(node_ref) = slab.get(node) {
            levels[level].push(node_ref.symbol);
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
            usize,
            i64,
            char,
            i64,
            char
        )
        .unwrap()
    });

    let (_, l_root_rank, l_root_sym, r_root_rank, r_root_sym) = lines.next().unwrap();
    let l_root = left_tree_nodes.insert(Node {
        rank: l_root_rank,
        symbol: l_root_sym,
        left: None,
        right: None,
    });
    let r_root = right_tree_nodes.insert(Node {
        rank: r_root_rank,
        symbol: r_root_sym,
        left: None,
        right: None,
    });

    for (_id, l_rank, l_sym, r_rank, r_sym) in lines {
        do_insert(&mut left_tree_nodes, l_root, l_rank, l_sym);
        do_insert(&mut right_tree_nodes, r_root, r_rank, r_sym);
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
            usize,
            i64,
            char,
            i64,
            char
        )
        .map(|(id, l_rank, l_sym, r_rank, r_sym)| Instruction::Add {
            id,
            left_rank: l_rank,
            left_symbol: l_sym,
            right_rank: r_rank,
            right_symbol: r_sym,
        })
        .or_else(|_| scan_fmt!(line, "SWAP {}", usize).map(|id| Instruction::Swap { id }))
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
    });
    let r_root = right_tree_nodes.insert(Node {
        rank: r_root_rank,
        symbol: r_root_sym,
        left: None,
        right: None,
    });

    let mut nodes_by_id = [(usize::MAX, usize::MAX); MAX_NODES + 1];

    for instruction in lines {
        match instruction {
            Instruction::Add {
                id,
                left_rank: l_rank,
                left_symbol: l_sym,
                right_rank: r_rank,
                right_symbol: r_sym,
            } => {
                let (l_node, r_node) = 
                    (
                        do_insert(&mut left_tree_nodes, l_root, l_rank, l_sym),
                        do_insert(&mut right_tree_nodes, r_root, r_rank, r_sym),
                    )
                ;
                nodes_by_id[id] = (l_node.get(), r_node.get());
            }
            Instruction::Swap { id } => {
                let (l_node_idx, r_node_idx) = nodes_by_id[id];
                if l_node_idx == usize::MAX || r_node_idx == usize::MAX {
                    continue; // Skip if either node doesn't exist
                }
                let l_node = left_tree_nodes.get_mut(l_node_idx).unwrap();
                let r_node = right_tree_nodes.get_mut(r_node_idx).unwrap();
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
            usize,
            i64,
            char,
            i64,
            char
        )
        .map(|(id, l_rank, l_sym, r_rank, r_sym)| Instruction::Add {
            id,
            left_rank: l_rank,
            left_symbol: l_sym,
            right_rank: r_rank,
            right_symbol: r_sym,
        })
        .or_else(|_| scan_fmt!(line, "SWAP {}", usize).map(|id| Instruction::Swap { id }))
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
    });
    let r_root = nodes.insert(Node {
        rank: r_root_rank,
        symbol: r_root_sym,
        left: None,
        right: None,
    });

    let mut nodes_by_id = [(usize::MAX, usize::MAX); MAX_NODES + 1];
    nodes_by_id[root_id] = (l_root, r_root);

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
                nodes_by_id[id] = (l_node.get(), r_node.get());
            }
            Instruction::Swap { id } => {
                let (l_node_idx, r_node_idx) = nodes_by_id[id];
                let (l_node, r_node) = nodes
                    .get2_mut(l_node_idx, r_node_idx)
                    .expect("Both nodes should exist");
                swap(l_node, r_node);
            }
        }
    }

    let mut part3 = message(&nodes, l_root);
    part3.push_str(message(&nodes, r_root).as_str());
    part3
}
