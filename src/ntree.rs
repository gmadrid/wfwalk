// Tree has a root node.
// For now, we will not have a parent ptr, since we may not need it.
// But each node may have N-children

struct NTree {
    val: String,
    children: Vec<NTree>,
}

impl NTree {
    fn new<T: AsRef<str>>(value: T) -> NTree {
        NTree {
            val: value.as_ref().to_owned(),
            children: vec![],
        }
    }

    fn add_child<T: AsRef<str>>(&mut self, value: T) -> usize {
        let node = NTree::new(value);
        self.children.push(node);
        self.children.len() - 1
    }

    fn child(&self, i: usize) -> &NTree {
        &self.children[i]
    }

    fn child_mut(&mut self, i: usize) -> &mut NTree {
        &mut self.children[i]
    }

    fn num_children(&self) -> usize {
        self.children.len()
    }

    fn value(&self) -> &str {
            self.val.as_str()
    }

    fn depth_iter(&self) -> DepthIter {
        DepthIter::default()
    }
}

#[derive(Default)]
struct DepthIter<'a> {
    nodes: Vec<(&'a NTree, usize)>,
}

impl<'a> Iterator for DepthIter<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some("Foobar".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = NTree::new(&"root");
        assert_eq!("root", tree.value());
        assert_eq!(0, tree.num_children());
    }

    #[test]
    fn test_one_child() {
        let mut tree = NTree::new( &"root");
        tree.add_child("child0");
        assert_eq!(1, tree.num_children());

        let child0 = tree.child(0);
        assert_eq!("child0", child0.value());
        assert_eq!(0, child0.num_children());
    }

    #[test]
    fn test_n_children() {
        let mut tree = NTree::new(&"foobar");
        tree.add_child("child0");
        tree.add_child("child1");
        tree.add_child("child2");
        tree.add_child("child3");

        assert_eq!(4, tree.num_children());
        assert_eq!("child0", tree.child(0).value());
        assert_eq!("child1", tree.child(1).value());
        assert_eq!("child2", tree.child(2).value());
        assert_eq!("child3", tree.child(3).value());
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

    fn make_a_big_tree() -> NTree {
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
    fn test_depth_first() {
        let tree = make_a_big_tree();
        let values: Vec<String> = tree.depth_iter().collect();

        assert_eq!(vec![
            "theroot",
            "child0",
            "child00",
            "child01",
            "child010",
            "child011",
            "child01",
            "child02",
            "child03",
            "child1",
            "child2",
            "child20",
            "child21",
        ], values);

    }
}
