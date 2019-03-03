use super::arena::ArenaIndex;
use super::ntree::NTree;
use crate::errors::*;

pub struct BuildData {
    pub tree: NTree<String>,
    stack: Vec<(usize, ArenaIndex)>,
    prefix_pattern: Option<String>,
}

impl BuildData {
    pub fn new<T>(prefix_pattern: Option<T>) -> BuildData
    where
        T: Into<String>,
    {
        BuildData {
            tree: NTree::new("".into()),
            stack: vec![],
            prefix_pattern: prefix_pattern.map(|s| s.into()),
        }
    }

    pub fn add(&mut self, s: &str) -> Result<()> {
        let (_trimmed, line_indent) = trim_with_length(s);
        let trimmed = trim_prefix(_trimmed, &self.prefix_pattern);

        // See if the indent level matches an existing level.
        let matched_level = self
            .stack
            .iter()
            .enumerate()
            .find_map(|(level, (indent, _))| {
                if line_indent == *indent {
                    Some(level)
                } else {
                    None
                }
            })
            .or_else(|| {
                if self.stack.len() > 0 {
                    let deepest_indent = self.stack[self.stack.len() - 1].0;
                    if line_indent < deepest_indent {
                        // In this case, the new indent is smaller than the current indent, but
                        // it doesn't match up with any higher level.
                        return None;
                    }
                }
                Some(self.stack.len())
            })
            .ok_or(format!(
                "Line with indent, {}, has no sibling:\n    {}",
                line_indent, s
            ))?;

        let parent_index = if matched_level == 0 {
            self.tree.root_index()
        } else {
            self.stack[matched_level - 1].1
        };

        let new_index = self.tree.add_child(parent_index, trimmed.to_string())?;

        if matched_level == self.stack.len() {
            // Add a new entry
            self.stack.push((line_indent, new_index));
        } else {
            // Update the existing entry and truncate the stack (if necessary).
            self.stack[matched_level] = (line_indent, new_index);
            self.stack.truncate(matched_level + 1);
        }

        Ok(())
    }
}

fn trim_with_length(s: &str) -> (&str, usize) {
    let trimmed = s.trim_start();
    (trimmed, s.len() - trimmed.len())
}

fn trim_prefix<'a>(s: &'a str, prefix: &Option<String>) -> &'a str {
    if let Some(p) = prefix {
        if s.starts_with(p) {
            let (_, new_str) = s.split_at(p.len());
            return new_str.trim_start();
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_with_length() {
        assert_eq!(("foo", 0), trim_with_length("foo"));
        assert_eq!(("foo", 3), trim_with_length("   foo"));
        assert_eq!(("foo  ", 3), trim_with_length("   foo  "));
        assert_eq!(("", 4), trim_with_length("   \n"));
        assert_eq!(("+ foo", 5), trim_with_length("     + foo"));
    }

    #[test]
    fn test_trim_prefix() {
        // No prefix should return the string unchanged.
        assert_eq!("quux", trim_prefix("quux", &None));
        assert_eq!("  quux", trim_prefix("  quux", &None));
        assert_eq!("- quux", trim_prefix("- quux", &None));
        assert_eq!("  - quux", trim_prefix("  - quux", &None));

        // A missing prefix should return the string unchanged.
        assert_eq!("quux", trim_prefix("quux", &Some("-".to_string())));
        assert_eq!("  quux", trim_prefix("  quux", &Some("-".to_string())));

        // Remove the prefix and any spaces which remain.
        assert_eq!("quux", trim_prefix("- quux", &Some("-".to_string())));
        assert_eq!("quux", trim_prefix("-    quux", &Some("-".to_string())));

        // The prefix must be at the very beginning of the string to have an effect.
        assert_eq!("  - quux", trim_prefix("  - quux", &Some("-".to_string())));

        // A mis-matched prefix should have no effect.
        assert_eq!("quux", trim_prefix("quux", &Some("XXX".to_string())));
        assert_eq!("  quux", trim_prefix("  quux", &Some("XXX".to_string())));
        assert_eq!("- quux", trim_prefix("- quux", &Some("XXX".to_string())));
        assert_eq!("  - quux", trim_prefix("  - quux", &Some("XXX".to_string())));
    }
}
