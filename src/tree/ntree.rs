use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::errors::*;

use super::arena::{Arena, ArenaIndex};

pub struct NTree<T> {
    arena: Arena<NTreeNode<T>>,
    root_index: ArenaIndex,
}

struct NTreeNode<T> {
    value: T,
    children: Vec<ArenaIndex>,
}

impl<T> NTree<T> {
    pub fn new(root_value: T) -> NTree<T> {
        let mut arena: Arena<NTreeNode<T>> = Arena::new();
        let root = NTreeNode {
            value: root_value,
            children: vec![],
        };
        let root_index = arena.alloc(root);
        NTree { arena, root_index }
    }

    pub fn root_index(&self) -> ArenaIndex {
        self.root_index
    }

    pub fn len(&self) -> usize {
        self.arena.live_count()
    }

    pub fn value(&self, index: ArenaIndex) -> Result<&T> {
        Ok(&self.arena.value(index)?.value)
    }

    pub fn add_child(&mut self, index: ArenaIndex, value: T) -> Result<ArenaIndex> {
        let new_node = NTreeNode {
            value,
            children: vec![],
        };
        let new_node_index = self.arena.alloc(new_node);
        self.arena.value_mut(index)?.children.push(new_node_index);
        Ok(new_node_index)
    }

    pub fn children(&self, index: ArenaIndex) -> Result<&Vec<ArenaIndex>> {
        Ok(&self.arena.value(index)?.children)
    }

    pub fn bf_iter(&self) -> BreadthNewIter<T> {
        self.bf_iter_from(self.root_index())
    }

    pub fn bf_iter_from(&self, idx: ArenaIndex) -> BreadthNewIter<T> {
        let mut queue = VecDeque::new();
        queue.push_back(idx);
        BreadthNewIter { tree: self, queue }
    }

    pub fn find_node<F>(&self, start_index: ArenaIndex, f: F) -> Option<ArenaIndex>
    where
        F: Fn((ArenaIndex, &T)) -> bool,
    {
        self.bf_iter_from(start_index)
            .find_map(|pair| if f(pair) { Some(pair.0) } else { None })
    }
}

pub struct BreadthNewIter<'a, T> {
    tree: &'a NTree<T>,
    queue: VecDeque<ArenaIndex>,
}

impl<'a, T> Iterator for BreadthNewIter<'a, T> {
    type Item = (ArenaIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|front| {
            self.queue.extend(self.tree.children(front).unwrap());
            (front, self.tree.value(front).unwrap())
        })
    }
}

impl<T> Display for NTree<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.format_helper(f, self.root_index(), 0);
        Ok(())
    }
}

impl<T> NTree<T>
where
    T: Display,
{
    fn format_helper(&self, f: &mut Formatter, index: ArenaIndex, indent: usize) {
        write!(f, "{2:1$}{0}\n", self.value(index).unwrap(), indent, "").unwrap();
        for child_index in self.children(index).unwrap() {
            self.format_helper(f, *child_index, indent + 3);
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Write;

    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = NTree::new("root");
        assert_eq!("root", *tree.value(tree.root_index()).unwrap());
        assert_eq!(1, tree.len());
    }

    #[test]
    fn test_one_child() {
        let mut tree = NTree::new("root");
        let child1 = tree.add_child(tree.root_index(), "child").unwrap();

        assert_eq!("child", *tree.value(child1).unwrap());
        assert_eq!(2, tree.len());
    }

    #[test]
    fn test_many_children() {
        let mut tree = NTree::new("root");
        let child1 = tree.add_child(tree.root_index(), "child1").unwrap();
        let child2 = tree.add_child(tree.root_index(), "child2").unwrap();
        let child3 = tree.add_child(tree.root_index(), "child3").unwrap();

        assert_eq!("child1", *tree.value(child1).unwrap());
        assert_eq!("child2", *tree.value(child2).unwrap());
        assert_eq!("child3", *tree.value(child3).unwrap());
        assert_eq!(4, tree.len());

        let children = tree.children(tree.root_index()).unwrap();
        assert_eq!(vec![child1, child2, child3], *children);
    }

    #[test]
    fn test_deep_children() {
        let mut tree = NTree::new("root");
        let child0 = tree.add_child(tree.root_index(), "child0").unwrap();
        let child1 = tree.add_child(tree.root_index(), "child1").unwrap();
        let child2 = tree.add_child(tree.root_index(), "child2").unwrap();

        let child00 = tree.add_child(child0, "child00").unwrap();
        let child01 = tree.add_child(child0, "child01").unwrap();
        let child02 = tree.add_child(child0, "child02").unwrap();
        let child03 = tree.add_child(child0, "child03").unwrap();

        let child20 = tree.add_child(child2, "child20").unwrap();
        let child21 = tree.add_child(child2, "child21").unwrap();

        assert_eq!(
            vec![child0, child1, child2],
            *tree.children(tree.root_index()).unwrap()
        );
        assert_eq!(
            vec![child00, child01, child02, child03],
            *tree.children(child0).unwrap()
        );
        assert_eq!(Vec::<ArenaIndex>::new(), *tree.children(child1).unwrap());
        assert_eq!(vec![child20, child21], *tree.children(child2).unwrap());
    }

    fn make_a_big_tree() -> NTree<&'static str> {
        let mut tree = NTree::new("root");
        let child0 = tree.add_child(tree.root_index(), "child0").unwrap();
        let child1 = tree.add_child(tree.root_index(), "child1").unwrap();
        let child2 = tree.add_child(tree.root_index(), "child2").unwrap();

        let _child00 = tree.add_child(child0, "child00").unwrap();
        let child01 = tree.add_child(child0, "child01").unwrap();
        let _child02 = tree.add_child(child0, "child02").unwrap();
        let _child03 = tree.add_child(child0, "child03").unwrap();

        tree.add_child(child01, "child010").unwrap();
        tree.add_child(child01, "child011").unwrap();

        let _child10 = tree.add_child(child1, "child10").unwrap();
        let _child11 = tree.add_child(child1, "child11").unwrap();

        let _child20 = tree.add_child(child2, "child20").unwrap();
        let _child21 = tree.add_child(child2, "child21").unwrap();

        tree
    }

    #[test]
    fn test_iter_from() {
        let tree = make_a_big_tree();
        let (child10, _) = tree.bf_iter().find(|(_, val)| **val == "child01").unwrap();

        let values: Vec<&str> = tree.bf_iter_from(child10).map(|(_, v)| *v).collect();

        assert_eq!(vec!["child01", "child010", "child011"], values);
    }

    #[test]
    fn test_simple_bf() {
        let mut tree = NTree::new("root");
        let child0 = tree.add_child(tree.root_index(), "child0").unwrap();
        let child1 = tree.add_child(tree.root_index(), "child1").unwrap();
        let child2 = tree.add_child(tree.root_index(), "child2").unwrap();

        let child3 = tree.add_child(child0, "child3").unwrap();

        let indices: Vec<ArenaIndex> = tree.bf_iter().map(|(i, _)| i).collect();

        assert_eq!(
            vec![tree.root_index(), child0, child1, child2, child3],
            indices
        );
    }

    #[test]
    fn test_simple_bf_with_indices_and_values() {
        let mut tree = NTree::new("root");
        let child0 = tree.add_child(tree.root_index(), "child0").unwrap();
        let child1 = tree.add_child(tree.root_index(), "child1").unwrap();
        let child2 = tree.add_child(tree.root_index(), "child2").unwrap();

        let child10 = tree.add_child(child1, "child10").unwrap();
        let child11 = tree.add_child(child1, "child11").unwrap();
        let child110 = tree.add_child(child11, "child110").unwrap();
        let child20 = tree.add_child(child2, "child20").unwrap();
        let child200 = tree.add_child(child20, "child200").unwrap();
        let child201 = tree.add_child(child20, "child201").unwrap();
        let child21 = tree.add_child(child2, "child21").unwrap();
        let child210 = tree.add_child(child21, "child210").unwrap();

        let values: Vec<(ArenaIndex, &str)> = tree.bf_iter().map(|(i, v)| (i, *v)).collect();
        assert_eq!(
            vec![
                (tree.root_index(), "root"),
                (child0, "child0"),
                (child1, "child1"),
                (child2, "child2"),
                (child10, "child10"),
                (child11, "child11"),
                (child20, "child20"),
                (child21, "child21"),
                (child110, "child110"),
                (child200, "child200"),
                (child201, "child201"),
                (child210, "child210"),
            ],
            values
        );
    }

    #[test]
    fn test_deep_iter_with_values() {
        let tree = make_a_big_tree();

        let values: Vec<&str> = tree.bf_iter().map(|(_, s)| *s).collect();
        assert_eq!(
            vec![
                "root", "child0", "child1", "child2", "child00", "child01", "child02", "child03",
                "child10", "child11", "child20", "child21", "child010", "child011",
            ],
            values
        );
    }

    static DISPLAY_TEST_OUTPUT: &str = r#"root
   child0
   child1
      child10
      child11
         child110
   child2
      child20
         child200
         child201
      child21
         child210
"#;

    #[test]
    fn test_display() {
        let mut tree = NTree::new("root");
        let _child0 = tree.add_child(tree.root_index(), "child0").unwrap();
        let child1 = tree.add_child(tree.root_index(), "child1").unwrap();
        let child2 = tree.add_child(tree.root_index(), "child2").unwrap();

        let _child10 = tree.add_child(child1, "child10").unwrap();
        let child11 = tree.add_child(child1, "child11").unwrap();
        let _child110 = tree.add_child(child11, "child110").unwrap();
        let child20 = tree.add_child(child2, "child20").unwrap();
        let _child200 = tree.add_child(child20, "child200").unwrap();
        let _child201 = tree.add_child(child20, "child201").unwrap();
        let child21 = tree.add_child(child2, "child21").unwrap();
        let _child210 = tree.add_child(child21, "child210").unwrap();

        println!("{}", tree);

        let mut str = String::new();
        write!(str, "{}", tree).unwrap();

        assert_eq!(DISPLAY_TEST_OUTPUT, str);
    }
}
