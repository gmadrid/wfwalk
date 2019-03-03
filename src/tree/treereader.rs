use std::io::BufRead;

use super::arena::ArenaIndex;
use super::ntree::NTree;
use crate::errors::*;
//
pub fn read_tree<R: BufRead>(reader: R, prefix_pattern: Option<&str>) -> Result<NTree<String>> {
//    let mut tree = NTree::new("ROOT".to_string());
//
//    let mut data = BuildData {
//        tree: &mut tree,
//        stack: Vec::default(),
//        prefix_pattern,
//    };
//
//    for line in reader.lines() {
//        match line {
//            Ok(txt) => data.add(txt.as_str())?,
//            Err(e) => return Err(Error::with_chain(e, "Failed while reading input.")),
//        };
//    }
//
//    Ok(tree)
    Err("foobar".into())
}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::io::BufReader;
//
//    fn bf_values_from_string(s: &'static str) -> Vec<String> {
//        let tree = read_tree(BufReader::new(s.as_bytes()), None).unwrap();
//        tree.bf_iter().map(|(_, s)| s.clone()).collect()
//    }
//
//    #[test]
//    fn test_trim_with_length() {
//        assert_eq!(("foo", 0), trim_with_length("foo"));
//        assert_eq!(("foo", 3), trim_with_length("   foo"));
//        assert_eq!(("foo  ", 3), trim_with_length("   foo  "));
//        assert_eq!(("", 4), trim_with_length("   \n"));
//        assert_eq!(("+ foo", 5), trim_with_length("     + foo"));
//    }
//
//    static TOP_LEVEL: &str = r#"ONE
//TWO
//THREE"#;
//
//    #[test]
//    fn test_top_level() {
//        assert_eq!(
//            vec!["ROOT", "ONE", "TWO", "THREE"],
//            bf_values_from_string(TOP_LEVEL)
//        );
//    }
//
//    static TWO_LEVELS: &str = r#"ONE
//    ONE-ONE
//    ONE-TWO
//TWO
//THREE"#;
//
//    #[test]
//    fn test_two_levels() {
//        assert_eq!(
//            vec!["ROOT", "ONE", "TWO", "THREE", "ONE-ONE", "ONE-TWO"],
//            bf_values_from_string(TWO_LEVELS)
//        );
//    }
//
//    static COMPLEX: &str = r#"ONE
//    ONE-ONE
//    ONE-TWO
//TWO
//  TWO-ONE
//  TWO-TWO
//  TWO-THREE
//THREE
//  THREE-ONE
//    THREE-ONE-ONE
//    THREE-ONE-TWO
//    THREE-ONE-THREE
//  THREE-TWO
//       THREE-TWO-ONE
//  THREE-THREE
//FOUR
//    FOUR-ONE
//    FOUR-TWO"#;
//
//    #[test]
//    fn test_complex() {
//        assert_eq!(
//            vec![
//                "ROOT",
//                "ONE",
//                "TWO",
//                "THREE",
//                "FOUR",
//                "ONE-ONE",
//                "ONE-TWO",
//                "TWO-ONE",
//                "TWO-TWO",
//                "TWO-THREE",
//                "THREE-ONE",
//                "THREE-TWO",
//                "THREE-THREE",
//                "FOUR-ONE",
//                "FOUR-TWO",
//                "THREE-ONE-ONE",
//                "THREE-ONE-TWO",
//                "THREE-ONE-THREE",
//                "THREE-TWO-ONE"
//            ],
//            bf_values_from_string(COMPLEX)
//        );
//    }
//
//    static SKIP_LEVELS: &str = r#"ONE
//TWO
//  TWO-ONE
//  TWO-TWO
//    TWO-TWO-ONE
//      TWO-TWO-ONE-ONE
//  TWO-THREE
//    TWO-THREE-ONE
//      TWO-THREE-ONE-ONE
//      TWO-THREE-ONE-TWO
//THREE
//  THREE-ONE"#;
//
//    #[test]
//    fn test_skip_levels() {
//        assert_eq!(
//            vec![
//                "ROOT",
//                "ONE",
//                "TWO",
//                "THREE",
//                "TWO-ONE",
//                "TWO-TWO",
//                "TWO-THREE",
//                "THREE-ONE",
//                "TWO-TWO-ONE",
//                "TWO-THREE-ONE",
//                "TWO-TWO-ONE-ONE",
//                "TWO-THREE-ONE-ONE",
//                "TWO-THREE-ONE-TWO"
//            ],
//            bf_values_from_string(SKIP_LEVELS)
//        );
//    }
//
//    static BAD_INDENT: &str = r#"ONE
//TWO
//   TWO-ONE
//  BAD-INDENT"#;
//
//    #[test]
//    fn test_bad_indent() {
//        let bad = read_tree(BufReader::new(BAD_INDENT.as_bytes()), None);
//        assert!(bad.is_err());
//    }
//
//    #[test]
//    fn test_trim_prefix() {
//        // No prefix should return the string unchanged.
//        assert_eq!("quux", trim_prefix("quux", None));
//        assert_eq!("  quux", trim_prefix("  quux", None));
//        assert_eq!("- quux", trim_prefix("- quux", None));
//        assert_eq!("  - quux", trim_prefix("  - quux", None));
//
//        // A missing prefix should return the string unchanged.
//        assert_eq!("quux", trim_prefix("quux", Some("-")));
//        assert_eq!("  quux", trim_prefix("  quux", Some("-")));
//
//        // Remove the prefix and any spaces which remain.
//        assert_eq!("quux", trim_prefix("- quux", Some("-")));
//        assert_eq!("quux", trim_prefix("-    quux", Some("-")));
//
//        // The prefix must be at the very beginning of the string to have an effect.
//        assert_eq!("  - quux", trim_prefix("  - quux", Some("-")));
//
//        // A mis-matched prefix should have no effect.
//        assert_eq!("quux", trim_prefix("quux", Some("XXX")));
//        assert_eq!("  quux", trim_prefix("  quux", Some("XXX")));
//        assert_eq!("- quux", trim_prefix("- quux", Some("XXX")));
//        assert_eq!("  - quux", trim_prefix("  - quux", Some("XXX")));
//    }
//}
