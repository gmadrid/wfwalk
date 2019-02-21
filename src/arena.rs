use std::ops::Index;

use crate::errors::*;

enum ArenaCell<T> {
    Live(T),
}

pub struct Arena<T> {
    cells: Vec<ArenaCell<T>>,
    live_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArenaIndex(usize);

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            cells: vec![],
            live_count: 0,
        }
    }

    pub fn live_count(&self) -> usize {
        self.live_count
    }

    pub fn alloc(&mut self, value: T) -> ArenaIndex {
        let result = self.cells.len();
        self.cells.push(ArenaCell::Live(value));
        self.live_count += 1;
        ArenaIndex(result)
    }

    pub fn value(&self, index: ArenaIndex) -> Result<&T> {
        match self.cells.get(index.0) {
            Some(ArenaCell::Live(value)) => Ok(value),
            _ => bail!("Index out of range: {}", index.0),
        }
    }

    pub fn value_mut(&mut self, index: ArenaIndex) -> Result<&mut T> {
        match self.cells.get_mut(index.0) {
            Some(ArenaCell::Live(value)) => Ok(value),
            _ => bail!("Index out of range: {}", index.0),
        }
    }
}

impl<T> Index<ArenaIndex> for Arena<T> {
    type Output = T;

    fn index(&self, index: ArenaIndex) -> &T {
        match &self.cells[index.0] {
            ArenaCell::Live(value) => &value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let arena: Arena<u8> = Arena::new();
        assert_eq!(0, arena.live_count());
    }

    #[test]
    fn test_add_some() {
        let mut arena: Arena<u8> = Arena::new();
        let idx0 = arena.alloc(0);
        let idx1 = arena.alloc(1);
        let idx2 = arena.alloc(22);

        assert_ne!(idx0, idx1);
        assert_ne!(idx0, idx2);
        assert_ne!(idx1, idx2);

        assert_eq!(3, arena.live_count())
    }

    #[test]
    fn test_values() {
        let mut arena: Arena<u8> = Arena::new();
        let idx0 = arena.alloc(0);
        let idx1 = arena.alloc(11);
        let idx2 = arena.alloc(222);

        assert_eq!(0, *arena.value(idx0).unwrap());
        assert_eq!(11, *arena.value(idx1).unwrap());
        assert_eq!(222, *arena.value(idx2).unwrap());
    }

    #[test]
    fn test_mutate() {
        let mut arena: Arena<u8> = Arena::new();
        let _idx0 = arena.alloc(0);
        let idx1 = arena.alloc(11);
        let _idx2 = arena.alloc(222);

        assert_eq!(11, *arena.value(idx1).unwrap());

        *arena.value_mut(idx1).unwrap() = 77;

        assert_eq!(77, *arena.value(idx1).unwrap());
    }

    #[test]
    fn test_out_of_range() {
        // The only way to get an out-of-range index is to use an index with the wrong Arena.
        let mut arena: Arena<u8> = Arena::new();
        let _idx0 = arena.alloc(0);
        let idx1 = arena.alloc(11);
        let idx2 = arena.alloc(222);

        let mut arena2: Arena<u8> = Arena::new();

        assert!(arena2.value(idx2).is_err());
        assert!(arena2.value_mut(idx1).is_err());
    }
}
