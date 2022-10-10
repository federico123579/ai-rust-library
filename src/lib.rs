mod algos;
mod dup_protection;
mod frontiers;

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

pub trait Space {
    type State: State;
    type Action: Action;
    fn initial_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}

#[derive(Debug)]
pub struct SearchResult<S>
where
    S: State,
{
    pub end_state: S,
    pub path: Vec<S::Action>,
    pub expanded: usize,
    pub generated: usize,
}

impl<S: State> SearchResult<S> {
    fn new(node: Node<S>, generated: usize, expanded: usize) -> Self {
        let path = node.path().to_owned();
        Self {
            end_state: node.state().to_owned(),
            path,
            expanded,
            generated,
        }
    }
}

impl<S: State> From<Node<S>> for SearchResult<S> {
    fn from(node: Node<S>) -> Self {
        SearchResult::new(node, 0, 0)
    }
}