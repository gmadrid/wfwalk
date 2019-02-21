//use std::io::BufRead;
//
//trait TreeNode {
//}
//
//fn trim_with_length(s: &str) -> (&str, usize) {
//    let trimmed = s.trim_start();
//    (trimmed, s.len() - trimmed.len())
//}
//
//struct TreeReader<N: TreeNode> {
//    root: N,
//}
//
//impl<N: TreeNode> TreeReader<N> {
//    fn read<R:BufRead>(reader: R) -> N {
//        for line in reader.lines() {
//        }
//
//        unimplemented!()
//    }
//
//
//}
//
//
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::io::BufReader;
//    use std::io::BufRead;
//
//    static SIMPLE_TREE: &str = r#"
//- Entry 1
//- Entry 2
//- Entry 3"#;
//
//    static TWO_DEEP_TREE: &str = r#"
//- Entry 1
//  - Entry 11
//  - Entry 12
//  - Entry 13
//- Entry 2
//  - Entry 21
//  - Entry 22
//- Entry 3
//- Entry 4
//  - Entry 41"#;
//
//    #[test]
//    fn test_trim_with_length() {
//        assert_eq!(("George", 0), trim_with_length("George"));
//        assert_eq!(("George", 2), trim_with_length("  George"));
//        assert_eq!(("George", 3), trim_with_length("   George"));
//        assert_eq!(("George  ", 3), trim_with_length("   George  "));
//    }
//
//
//    // #[test]
//    // fn test_simple() {
//    //     let br = BufReader::new(simple_tree.as_bytes());
//
//    //     for l in br.lines() {
//    //         println!("###{}###", l.unwrap());
//    //     }
//
//    //     assert_eq!(1, 0);
//    // }
//}
