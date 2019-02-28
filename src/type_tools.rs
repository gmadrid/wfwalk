pub trait OptionTools {
    /// Returns None if self has Some value; otherwise returns Some(f()).
    fn not<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce() -> U;
}

impl<T> OptionTools for Option<T> {
    fn not<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce() -> U,
    {
        match self {
            Some(_) => None,
            None => Some(f()),
        }
    }
}

pub trait BoolTools {
    /// If the value is true, returns Some(f()). Otherwise, returns None.
    fn then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T;
}

impl BoolTools for bool {
    fn then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

pub trait VecTools {
    fn to_strings(&self) -> Vec<String>;
}

impl<T> VecTools for Vec<T>
where
    T: ToString,
{
    fn to_strings(&self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_none() {
        assert_eq!(Some("False"), None::<Option<&str>>.not(|| "False"));
        assert_eq!(None, Some("True").not(|| "Not used"));
    }

    #[test]
    fn test_if_then() {
        assert_eq!(Some("True"), true.then(|| "True"));
        assert_eq!(None, false.then(|| "Not used"));
    }

    #[test]
    fn test_to_strings() {
        assert_eq!(
            vec!["one".to_string(), "two".to_string()],
            vec!["one", "two"].to_strings()
        );
        assert_eq!(
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec![1, 2, 3].to_strings()
        );
    }
}
