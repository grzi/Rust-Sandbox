use std::collections::VecDeque;

pub fn execute(_line: String) -> (usize, usize) {
    let tree = TreeNode::read_next_node(&mut read_input(_line.lines().next().unwrap()));
    (get_total_metadata(&tree), get_total_metadata_by_child(&tree))
}

fn read_input(_input: &str) -> VecDeque<usize> {
    _input.split(" ").collect::<Vec<&str>>().iter().map(|e| e.parse().unwrap()).collect()
}

fn get_total_metadata(_tree: &TreeNode) -> usize {
    _tree.metadata.iter().sum::<usize>()
        + _tree.children.iter().map(|e| get_total_metadata(&e)).sum::<usize>()
}

fn get_total_metadata_by_child(_tree: &TreeNode) -> usize {
    if _tree.children.len() == 0 {
        return _tree.metadata.iter().sum::<usize>();
    }

    _tree.metadata.iter().filter(|e| (**e - 1 ) < _tree.children.len())
        .map(|e| get_total_metadata_by_child(_tree.children.get(*e - 1).unwrap())).sum()
}

struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<usize>,
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }

    fn read_next_node(_queue: &mut VecDeque<usize>) -> TreeNode {
        let mut this_node = TreeNode::new();

        let child_number = _queue.pop_front().unwrap();
        let metadata_number = _queue.pop_front().unwrap();

        (0..child_number).for_each(|_| this_node.children.push(TreeNode::read_next_node(_queue)));
        (0..metadata_number).for_each(|_| this_node.metadata.push(_queue.pop_front().unwrap()));
        this_node
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
