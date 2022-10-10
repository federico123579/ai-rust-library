mod frontiers;
mod dup_protection;

use std::{collections::{HashSet, VecDeque}, hash::Hash};
use frontiers::{Frontier};

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

#[derive(Clone)]
pub struct StateWrap<S: SearchState> {
    state: S,
    actions: Vec<S::Action>,
}

impl<S: SearchState> StateWrap<S> {
    fn new(state: S) -> StateWrap<S> {
        StateWrap {
            state,
            actions: Vec::new(),
        }
    }

    fn unwrap(self) -> (S, Vec<S::Action>) {
        (self.state, self.actions.into())
    }
}

trait WrappedState: SearchState {
    fn wrap_actions(self, actions: Vec<Self::Action>) -> StateWrap<Self>;
}

impl<S: SearchState> WrappedState for S {
    fn wrap_actions(self, actions: Vec<Self::Action>) -> StateWrap<Self> {
        StateWrap {
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
            let (state, actions) = wrap.unwrap();
            // println!("state:\n{}", &state);
            if space.is_goal(&state) {
                return Some(SearchResult {
                    end_state: state,
                    path: actions.into(),
                    expanded: visited.len(),
                    generated: generated as usize,
                });
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
            let (state, actions) = wrap.unwrap();
            // println!("state:\n{}", state);
            if space.is_goal(&state) {
                return Some(SearchResult {
                    end_state: state,
                    path: actions.into(),
                    expanded: visited.len(),
                    generated });
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
