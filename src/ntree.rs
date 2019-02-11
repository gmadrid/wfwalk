// Tree has a root node.
// For now, we will not have a parent ptr, since we may not need it.
// But each node may have N-children

use std::collections::VecDeque;

struct NTree<T> {
    val: T,
    children: Vec<NTree<T>>,
}

impl<T> NTree<T> {
    fn new(value: T) -> NTree<T> {
        NTree {
            val: value,
            children: vec![],
        }
    }

    fn add_child(&mut self, value: T) -> usize {
        let node = NTree::new(value);
        self.children.push(node);
        self.children.len() - 1
    }

    fn child(&self, i: usize) -> &NTree<T> {
        &self.children[i]
    }

    fn child_mut(&mut self, i: usize) -> &mut NTree<T> {
        &mut self.children[i]
    }

    fn num_children(&self) -> usize {
        self.children.len()
    }

    fn value(&self) -> &T {
        &self.val
    }

    fn bf_iter(&self) -> BreadthIter<T> {
        BreadthIter::new_at(self)
    }
}

#[derive(Default)]
struct BreadthIter<'a, T> {
    queue: VecDeque<&'a NTree<T>>
}

impl<'a, T> BreadthIter<'a, T> {
    fn new_at(node: &'a NTree<T>) -> BreadthIter<'a, T> {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        BreadthIter {
            queue
        }
    }
}

impl<'a, T> Iterator for BreadthIter<'a, T> {
    type Item = &'a NTree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(front) = self.queue.pop_front() {
            self.queue.extend(front.children.iter().by_ref());
            return Some(front)
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = NTree::new("root");
        assert_eq!("root", *tree.value());
        assert_eq!(0, tree.num_children());
    }

    #[test]
    fn test_one_child() {
        let mut tree = NTree::new("root");
        tree.add_child("child0");
        assert_eq!(1, tree.num_children());

        let child0 = tree.child(0); 
       assert_eq!("child0", *child0.value());
        assert_eq!(0, child0.num_children());
    }

    #[test]
    fn test_n_children() {
        let mut tree = NTree::new("foobar");
        tree.add_child("child0");
        tree.add_child("child1");
        tree.add_child("child2");
        tree.add_child("child3");

        assert_eq!(4, tree.num_children());
        assert_eq!("child0", *tree.child(0).value());
        assert_eq!("child1", *tree.child(1).value());
        assert_eq!("child2", *tree.child(2).value());
        assert_eq!("child3", *tree.child(3).value());
    }

    #[test]
    fn test_deep_children() {
        let mut tree = NTree::new("foobar");
        tree.add_child("child0");
        tree.add_child("child1");
        tree.add_child("child2");

        let child = tree.child_mut(0);
        child.add_child("child00");
        child.add_child("child01");
        child.add_child("child02");
        child.add_child("child03");

        let child = tree.child_mut(2);
        child.add_child("child20");
        child.add_child("child21");

        assert_eq!(3, tree.num_children());
        assert_eq!(4, tree.child(0).num_children());
        assert_eq!(0, tree.child(1).num_children());
        assert_eq!(2, tree.child(2).num_children());
    }

    fn make_a_big_tree() -> NTree<&'static str> {
        let mut tree = NTree::new("theroot");
        tree.add_child("child0");
        tree.add_child("child1");
        tree.add_child("child2");

        let child0 = tree.child_mut(0);
        child0.add_child("child00");
        child0.add_child("child01");
        child0.add_child("child02");
        child0.add_child("child03");

        let child01 = child0.child_mut(1);
        child01.add_child("child010");
        child01.add_child("child011");

        let child2 = tree.child_mut(2);
        child2.add_child("child20");
        child2.add_child("child21");

        tree
    }

    #[test]
    fn test_simple_bf() {
        let mut tree = NTree::new("root");
        tree.add_child("child1");
        tree.add_child("child2");
        tree.add_child("child3");

        let child1 = tree.child_mut(0);
        child1.add_child("child4");

        let values:  Vec<&str> = tree.bf_iter().map(|nt| nt.val).collect();

        assert_eq!(vec!["root", "child1", "child2", "child3", "child4"], values);
    }

    #[test]
    fn test_breadth_first() {
        let tree = make_a_big_tree();
        let values: Vec<&str> = tree.bf_iter().map(|nt| nt.val).collect();

        assert_eq!(vec![
            "theroot",
            "child0",
            "child1",
            "child2",
            "child00",
            "child01",
            "child02",
            "child03",
            "child20",
            "child21",
            "child010",
            "child011",
        ], values);
    }
}
