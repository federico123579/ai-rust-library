use crate::{State, Node};
use std::collections::{VecDeque, BinaryHeap};

pub trait Frontier: Iterator {
    type State: State;
    fn new(initial_state: Self::State) -> Self;
    fn push(&mut self, state: Node<Self::State>);
    fn pop(&mut self) -> Option<Node<Self::State>>;
}

// ================================================================================
// Implementation of the frontier
// ================================================================================
pub struct QueueFrontier<S: State> {
    queue: VecDeque<Node<S>>,
}

impl<S: State> Iterator for QueueFrontier<S> {
    type Item = Node<S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl<S: State> Frontier for QueueFrontier<S> {
    type State = S;

    fn new(initial_state: Self::State) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(Node::new(initial_state));
        Self { queue }
    }

    fn push(&mut self, state: Node<Self::State>) {
        self.queue.push_back(state);
    }

    fn pop(&mut self) -> Option<Node<Self::State>> {
        self.queue.pop_front()
    }
}

pub struct StackFrontier<S: State> {
    stack: Vec<Node<S>>,
}

impl<S: State> Iterator for StackFrontier<S> {
    type Item = Node<S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<S: State> Frontier for StackFrontier<S> {
    type State = S;

    fn new(initial_state: Self::State) -> Self {
        let mut stack = Vec::new();
        stack.push(Node::new(initial_state));
        Self { stack }
    }

    fn push(&mut self, state: Node<Self::State>) {
        self.stack.push(state);
    }

    fn pop(&mut self) -> Option<Node<Self::State>> {
        self.stack.pop()
    }
}

pub struct PriorityFrontier<S> where 
    S: State + PartialEq + Eq + PartialOrd + Ord,
{
    heap: BinaryHeap<S>,
}

impl<S: State + PartialEq + Eq + PartialOrd + Ord> Iterator for PriorityFrontier<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}