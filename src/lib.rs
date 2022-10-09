use std::{collections::HashSet, hash::Hash};

pub trait StateAction {}

pub trait SearchState: Clone + Eq + Hash {
    type Action: StateAction;
    fn actions(&self) -> Vec<Self::Action>;
    fn apply(&self, action: &Self::Action) -> Self;
}

pub trait SearchSpace {
    type State: SearchState;
    type Action: StateAction;
    fn initial_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}

pub trait SearchAlgorithm {
    // fn search(&self, space: impl SearchSpace) -> Option<dyn SearchState<Action=dyn StateAction>>;
    fn search<S,A,P>(space: P) -> Option<S> where
        S: SearchState<Action=A> + std::fmt::Display,
        A: StateAction,
        P: SearchSpace<State=S, Action=A>;
}

// struct Node<S: SearchState> {
//     state: S,
//     parent: Option<Box<Node<S>>>,
//     action: Option<S::Action>,
//     cost: usize,
// }

pub struct DepthFirstSearch {}

impl SearchAlgorithm for DepthFirstSearch {
    fn search<S,A,P>(space: P) -> Option<S> where
        S: SearchState<Action=A> + std::fmt::Display,
        A: StateAction,
        P: SearchSpace<State=S, Action=A>,
    {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(space.initial_state());
        while let Some(state) = stack.pop() {
            println!("state:\n{}", state);
            if space.is_goal(&state) {
                return Some(state);
            }
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());
            for action in state.actions() {
                stack.push(state.apply(&action));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
