use super::build_data::BuildData;
use super::ntree::NTree;
use crate::errors::*;
use std::io::BufRead;

pub fn read_tree<R: BufRead>(reader: R, prefix_pattern: Option<&str>) -> Result<NTree<String>> {
    let mut build_data = BuildData::new(prefix_pattern);

    for line in reader.lines() {
        match line {
            Ok(txt) => build_data.add(txt.as_str())?,
            Err(e) => return Err(Error::with_chain(e, "Failed while reading input.")),
        };
    }

    Ok(build_data.tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn bf_values_from_string(s: &'static str) -> Vec<String> {
        let tree = read_tree(BufReader::new(s.as_bytes()), None).unwrap();
        tree.bf_iter().map(|(_, s)| s.clone()).collect()
    }

    const TOP_LEVEL: &str = r#"ONE
TWO
THREE"#;

    #[test]
    fn test_top_level() {
        assert_eq!(
            vec!["", "ONE", "TWO", "THREE"],
            bf_values_from_string(TOP_LEVEL)
        );
    }

    static TWO_LEVELS: &str = r#"ONE
    ONE-ONE
    ONE-TWO
TWO
THREE"#;

    #[test]
    fn test_two_levels() {
        assert_eq!(
            vec!["", "ONE", "TWO", "THREE", "ONE-ONE", "ONE-TWO"],
            bf_values_from_string(TWO_LEVELS)
        );
    }

    static COMPLEX: &str = r#"ONE
    ONE-ONE
    ONE-TWO
TWO
  TWO-ONE
  TWO-TWO
  TWO-THREE
THREE
  THREE-ONE
    THREE-ONE-ONE
    THREE-ONE-TWO
    THREE-ONE-THREE
  THREE-TWO
       THREE-TWO-ONE
  THREE-THREE
FOUR
    FOUR-ONE
    FOUR-TWO"#;

    #[test]
    fn test_complex() {
        assert_eq!(
            vec![
                "",
                "ONE",
                "TWO",
                "THREE",
                "FOUR",
                "ONE-ONE",
                "ONE-TWO",
                "TWO-ONE",
                "TWO-TWO",
                "TWO-THREE",
                "THREE-ONE",
                "THREE-TWO",
                "THREE-THREE",
                "FOUR-ONE",
                "FOUR-TWO",
                "THREE-ONE-ONE",
                "THREE-ONE-TWO",
                "THREE-ONE-THREE",
                "THREE-TWO-ONE"
            ],
            bf_values_from_string(COMPLEX)
        );
    }

    static SKIP_LEVELS: &str = r#"ONE
TWO
  TWO-ONE
  TWO-TWO
    TWO-TWO-ONE
      TWO-TWO-ONE-ONE
  TWO-THREE
    TWO-THREE-ONE
      TWO-THREE-ONE-ONE
      TWO-THREE-ONE-TWO
THREE
  THREE-ONE"#;

    #[test]
    fn test_skip_levels() {
        assert_eq!(
            vec![
                "",
                "ONE",
                "TWO",
                "THREE",
                "TWO-ONE",
                "TWO-TWO",
                "TWO-THREE",
                "THREE-ONE",
                "TWO-TWO-ONE",
                "TWO-THREE-ONE",
                "TWO-TWO-ONE-ONE",
                "TWO-THREE-ONE-ONE",
                "TWO-THREE-ONE-TWO"
            ],
            bf_values_from_string(SKIP_LEVELS)
        );
    }

    static BAD_INDENT: &str = r#"ONE
TWO
   TWO-ONE
  BAD-INDENT"#;

    #[test]
    fn test_bad_indent() {
        let bad = read_tree(BufReader::new(BAD_INDENT.as_bytes()), None);
        assert!(bad.is_err());
    }
}
