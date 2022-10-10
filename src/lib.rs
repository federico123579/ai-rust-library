mod frontiers;
mod dup_protection;

use std::{collections::HashSet, hash::Hash};
use frontiers::Frontier;

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
        Self {
            state,
            path,
        }
    }
}

pub trait Space {
    type State: State;
    type Action: Action;
    fn initial_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}


#[derive(Debug)]
pub struct SearchResult<S> where
    S: State,
{
    pub end_state: S,
    pub path: Vec<S::Action>,
    pub expanded: usize,
    pub generated: usize,
}

impl <S: State> SearchResult<S> {
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


// ================================================================================
// Search algorithms:
// - DFS
// - BFS
// ================================================================================
pub trait SearchAlgorithm {
    // fn search(&self, space: impl SearchSpace) -> Option<dyn SearchState<Action=dyn StateAction>>;
    fn search<P: Space>(space: P) -> Option<SearchResult<P::State>>;
        // S: State + std::fmt::Display,
        // P: Space<State=S>;
}

pub struct DepthFirstSearch {}

impl SearchAlgorithm for DepthFirstSearch {
    fn search<P: Space>(space: P) -> Option<SearchResult<P::State>>
    {
        let mut generated: usize = 0;
        let mut frontier = frontiers::StackFrontier::new(space.initial_state());
        let mut visited = dup_protection::StateCacheSet::new();
        while let Some(node) = frontier.pop() {
            let state = node.state();
            if space.is_goal(&state) {
                return Some(SearchResult::new(node, generated, visited.len()));
            }
            if visited.contains(state) {
                continue;
            }
            visited.insert(state.clone());
            for action in state.get_available_actions() {
                frontier.push(node.apply(&action));
                generated += 1;
            }
        }
        None
    }
}

pub struct BreadthFirstSearch {}

impl SearchAlgorithm for BreadthFirstSearch {
    fn search<P: Space>(space: P) -> Option<SearchResult<P::State>> {
        let mut queue = frontiers::QueueFrontier::new(space.initial_state());
        let mut visited = HashSet::new();
        let mut generated: usize = 0;
        while let Some(node) = queue.pop() {
            let state = node.state();
            if space.is_goal(&state) {
                return Some(SearchResult::new(node, generated, visited.len()));
            }
            if visited.contains(state) {
                continue;
            }
            visited.insert(state.clone());
            for action in state.get_available_actions() {
                queue.push(node.apply(&action));
                generated += 1;
            }
        }
        None
    }
}

pub trait UniformCostSearch {
    fn uniform_search<P>(space: P) -> Option<SearchResult<P::State>> where
        P: Space,    
        P::Action: CostAction,
        P::State: State;
}

impl<S> UniformCostSearch for S where
        S: Space,
        S::Action: CostAction,
        S::State: State,
    {
    fn uniform_search<P: Space>(space: P) -> Option<SearchResult<P::State>> {
        let mut frontier = frontiers::QueueFrontier::new(space.initial_state());
        let mut visited = HashSet::new();
        let mut generated: usize = 0;
        while let Some(node) = frontier.pop() {
            let state = node.state();
            if space.is_goal(&state) {
                return Some(SearchResult::new(node, generated, visited.len()));
            }
            if visited.contains(state) {
                continue;
            }
            visited.insert(state.clone());
            for action in state.get_available_actions() {
                frontier.push(node.apply(&action));
                generated += 1;
            }
        }
        None
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
