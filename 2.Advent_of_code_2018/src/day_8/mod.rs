use std::collections::VecDeque;
use std::borrow::BorrowMut;

pub fn execute(_line: String) -> (u32, u32) {
    let mut queue = read_input(_line.lines().next().unwrap());
    let tree = read_next_node(&mut queue);
    (get_total_metadata(&tree), get_total_metadata_by_child(&tree))
}

fn read_input(_input: &str) -> VecDeque<u32> {
    let splitted = _input.split(" ").collect::<Vec<&str>>();
    splitted.iter().map(|e| e.parse().unwrap()).collect()
}

fn read_next_node(_queue: &mut VecDeque<u32>) -> TreeNode {
    let mut this_node = TreeNode::new();

    let child_number = _queue.pop_front().unwrap();
    let metadata_number = _queue.pop_front().unwrap();

    for i in 0..child_number {
        this_node.children.push(read_next_node(_queue));
    }

    for i in 0..metadata_number {
        this_node.metadata.push(_queue.pop_front().unwrap());
    }

    this_node
}

fn get_total_metadata(_tree: &TreeNode) -> u32 {
    let mut res = 0;
    for metadata in &_tree.metadata {
        res += metadata
    }

    for child in &_tree.children {
        res += get_total_metadata(&child);
    }

    res
}

fn get_total_metadata_by_child(_tree: &TreeNode) -> u32 {
    let mut res = 0;

    if _tree.children.len() > 0 {
        for metadata in &_tree.metadata {
            if metadata - 1 < (_tree.children.len() as u32) {
                res += get_total_metadata_by_child(&_tree.children.get((metadata - 1) as usize).unwrap())
            }
        }
    } else {
        for metadata in &_tree.metadata {
            res += metadata
        }
    }

    res
}

struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<u32>,
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        assert_eq!(read_input("1 2 3 4 5"), [1, 2, 3, 4, 5]);
    }
}
