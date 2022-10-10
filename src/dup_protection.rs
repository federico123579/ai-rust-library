use std::collections::HashSet;
use crate::{SearchState};

pub struct StateCacheSet<S: SearchState> {
    seen: HashSet<S>,
}

impl <S: SearchState> StateCacheSet<S> {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    pub fn contains(&self, state: &S) -> bool {
        self.seen.contains(state)
    }

    pub fn insert(&mut self, state: S) {
        self.seen.insert(state);
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }
}