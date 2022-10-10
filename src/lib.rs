mod algos;
mod dup_protection;
mod frontiers;
mod output;

use std::hash::Hash;

pub use algos::{BreadthFirstSearch, DepthFirstSearch};

// ================================================================================
// Traits to be implemented by the user to define the search problem
// ================================================================================
/// Base trait for action
pub trait Action: Clone {}

/// Action with a cost associated with it
pub trait CostAction: Action {
    fn cost(&self) -> usize;
}

pub trait State: Clone + Eq + Hash {
    type Action: Action;
    fn get_available_actions(&self) -> Vec<Self::Action>;
    /// this build a new state from previous
    fn apply(&self, action: &Self::Action) -> Self;
}

pub trait Space {
    type State: State;
    type Action: Action;
    fn initial_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}

#[derive(Debug, Clone)]
pub struct Node<S: State> {
    state: S,
    path: Vec<S::Action>,
}

impl<S: State> Node<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            path: Vec::new(),
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn path(&self) -> &Vec<S::Action> {
        &self.path
    }

    pub fn apply(&self, action: &S::Action) -> Self {
        let state = self.state.apply(action);
        let mut path = self.path.clone();
        path.push(action.clone());
        Self { state, path }
    }
}