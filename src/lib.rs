mod frontiers;
mod dup_protection;

use std::{collections::{HashSet, VecDeque}, hash::Hash};
use frontiers::Frontier;

// ================================================================================
// Traits to be implemented by the user to define the search problem
// ================================================================================
pub trait StateAction: Clone {}

pub trait SearchState: Clone + Eq + Hash {
    type Action: StateAction;
    fn actions(&self) -> Vec<Self::Action>;
    fn apply(&self, action: &Self::Action) -> Self;
}

pub trait SearchSpace {
    type State: SearchState;
    fn initial_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}

// ================================================================================
// State wrapper
// ================================================================================
#[derive(Clone)]
pub struct StateActionWrap<S: SearchState> {
    state: S,
    actions: Vec<S::Action>,
}

impl<S: SearchState> StateActionWrap<S> {
    fn new(state: S) -> StateActionWrap<S> {
        StateActionWrap {
            state,
            actions: Vec::new(),
        }
    }

    fn unwrap(self) -> (S, Vec<S::Action>) {
        (self.state, self.actions.into())
    }
}

trait ActionWrappedState: SearchState {
    fn wrap_actions(self, actions: Vec<Self::Action>) -> StateActionWrap<Self>;
}

impl<S: SearchState> ActionWrappedState for S {
    fn wrap_actions(self, actions: Vec<Self::Action>) -> StateActionWrap<Self> {
        StateActionWrap {
            state: self,
            actions: actions.into(),
        }
    }
} 

#[derive(Debug)]
pub struct SearchResult<S> where
    S: SearchState,
{
    pub end_state: S,
    pub path: VecDeque<S::Action>,
    pub expanded: usize,
    pub generated: usize,
}

impl <S: SearchState> SearchResult<S> {
    fn new(end_state: S, path: VecDeque<S::Action>, expanded: usize, generated: usize) -> Self {
        Self {
            end_state,
            path,
            expanded,
            generated,
        }
    }

    fn from_wrap(wrap: StateActionWrap<S>, generated: usize, expanded: usize) -> Self {
        let (state, actions) = wrap.unwrap();
        Self {
            end_state: state,
            path: actions.into(),
            expanded,
            generated,
        }
    }
}

impl<S: SearchState> From<StateActionWrap<S>> for SearchResult<S> {
    fn from(wrap: StateActionWrap<S>) -> Self {
        SearchResult::new(wrap.state, wrap.actions.into(), 0, 0)
    }
}


// ================================================================================
// Search algorithms:
// - DFS
// - BFS
// ================================================================================
pub trait SearchAlgorithm {
    // fn search(&self, space: impl SearchSpace) -> Option<dyn SearchState<Action=dyn StateAction>>;
    fn search<S,P>(space: P) -> Option<SearchResult<S>> where
        S: SearchState + std::fmt::Display,
        P: SearchSpace<State=S>;
}

pub struct DepthFirstSearch {}

impl SearchAlgorithm for DepthFirstSearch {
    fn search<S,P>(space: P) -> Option<SearchResult<S>> where
        S: SearchState + std::fmt::Display,
        P: SearchSpace<State=S>,
    {
        let mut generated: usize = 0;
        let mut frontier = frontiers::StackFrontier::new(space.initial_state());
        let mut visited = dup_protection::StateCacheSet::new();
        while let Some(wrap) = frontier.pop() {
            let (state, actions) = wrap.clone().unwrap();
            // println!("state:\n{}", &state);
            if space.is_goal(&state) {
                return Some(SearchResult::from_wrap(wrap, generated, visited.len()));
            }
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());
            for action in state.actions() {
                let mut past_actions = actions.clone();
                past_actions.push(action.clone());
                frontier.push(state.apply(&action).wrap_actions(past_actions));
                generated += 1;
            }
        }
        None
    }
}

pub struct BreadthFirstSearch {}

impl SearchAlgorithm for BreadthFirstSearch {
    fn search<S,P>(space: P) -> Option<SearchResult<S>> where
        S: SearchState + std::fmt::Display,
        P: SearchSpace<State=S>,
    {
        let mut queue = frontiers::QueueFrontier::new(space.initial_state());
        let mut visited = HashSet::new();
        let mut generated: usize = 0;
        while let Some(wrap) = queue.pop() {
            let (state, actions) = wrap.clone().unwrap();
            // println!("state:\n{}", state);
            if space.is_goal(&state) {
                return Some(SearchResult::from_wrap(wrap, generated, visited.len()));
            }
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());
            for action in state.actions() {
                let mut past_actions = actions.clone();
                past_actions.push(action.clone());
                queue.push(state.apply(&action).wrap_actions(past_actions));
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
