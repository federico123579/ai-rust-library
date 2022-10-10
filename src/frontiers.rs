use crate::{SearchState, StateActionWrap};
use std::collections::VecDeque;

pub trait Frontier: Iterator {
    type State: SearchState;
    fn new(initial_state: Self::State) -> Self;
    fn push(&mut self, state: StateActionWrap<Self::State>);
    fn pop(&mut self) -> Option<StateActionWrap<Self::State>>;
}

pub struct QueueFrontier<S: SearchState> {
    queue: VecDeque<StateActionWrap<S>>,
}

impl<S: SearchState> Iterator for QueueFrontier<S> {
    type Item = StateActionWrap<S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl<S: SearchState> Frontier for QueueFrontier<S> {
    type State = S;

    fn new(initial_state: Self::State) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(StateActionWrap::new(initial_state));
        Self { queue }
    }

    fn push(&mut self, state: StateActionWrap<Self::State>) {
        self.queue.push_back(state);
    }

    fn pop(&mut self) -> Option<StateActionWrap<Self::State>> {
        self.queue.pop_front()
    }
}

pub struct StackFrontier<S: SearchState> {
    stack: Vec<StateActionWrap<S>>,
}

impl<S: SearchState> Iterator for StackFrontier<S> {
    type Item = StateActionWrap<S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<S: SearchState> Frontier for StackFrontier<S> {
    type State = S;

    fn new(initial_state: Self::State) -> Self {
        let mut stack = Vec::new();
        stack.push(StateActionWrap::new(initial_state));
        Self { stack }
    }

    fn push(&mut self, state: StateActionWrap<Self::State>) {
        self.stack.push(state);
    }

    fn pop(&mut self) -> Option<StateActionWrap<Self::State>> {
        self.stack.pop()
    }
}