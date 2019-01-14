

struct TreeReader {

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::BufRead;

    static simple_tree: &str = r#"
- Entry 1
- Entry 2
- Entry 3"#;

    static two_deep_tree: &str = r#"
- Entry 1
  - Entry 11
  - Entry 12
  - Entry 13
- Entry 2
  - Entry 21
  - Entry 22
- Entry 3
- Entry 4
  - Entry 41"#;


    #[test]
    fn test_simple() {
        let br = BufReader::new(simple_tree.as_bytes());

        for l in br.lines() {
            println!("###{}###", l.unwrap());
        }

        assert_eq!(1, 0);
    }
}