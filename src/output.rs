use crate::{State, Node};

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
    pub fn new(node: Node<S>, generated: usize, expanded: usize) -> Self {
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