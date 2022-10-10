//! Search algorithms:
//! - DFS
//! - BFS

use crate::{
    dup_protection::StateCacheSet,
    frontiers::{Frontier, QueueFrontier, StackFrontier},
    output::SearchResult,
    Action, Space, State,
};
use std::collections::HashSet;

pub trait DepthFirstSearch<S: Space> {
    fn dfs_search(&self) -> Option<SearchResult<S::State>>;
}

impl<S> DepthFirstSearch<S> for S
where
    S: Space,
    S::Action: Action,
    S::State: State,
{
    fn dfs_search(&self) -> Option<SearchResult<S::State>> {
        let mut generated: usize = 0;
        let mut frontier = StackFrontier::new(self.initial_state());
        let mut visited = StateCacheSet::new();
        while let Some(node) = frontier.pop() {
            let state = node.state();
            if self.is_goal(&state) {
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

pub trait BreadthFirstSearch<S: Space> {
    fn bfs_search(&self) -> Option<SearchResult<S::State>>;
}

impl<S> BreadthFirstSearch<S> for S
where
    S: Space,
    S::Action: Action,
    S::State: State,
{
    fn bfs_search(&self) -> Option<SearchResult<S::State>> {
        let mut queue = QueueFrontier::new(self.initial_state());
        let mut visited = HashSet::new();
        let mut generated: usize = 0;
        while let Some(node) = queue.pop() {
            let state = node.state();
            if self.is_goal(&state) {
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

// pub trait UniformCostSearch {
//     fn uniform_search<P>(space: P) -> Option<SearchResult<P::State>> where
//         P: Space,
//         P::Action: CostAction,
//         P::State: State;
// }

// impl<S> UniformCostSearch for S where
//         S: Space,
//         S::Action: CostAction,
//         S::State: State,
//     {
//     fn uniform_search<P: Space>(space: P) -> Option<SearchResult<P::State>> {
//         let mut frontier = frontiers::QueueFrontier::new(space.initial_state());
//         let mut visited = HashSet::new();
//         let mut generated: usize = 0;
//         while let Some(node) = frontier.pop() {
//             let state = node.state();
//             if space.is_goal(&state) {
//                 return Some(SearchResult::new(node, generated, visited.len()));
//             }
//             if visited.contains(state) {
//                 continue;
//             }
//             visited.insert(state.clone());
//             for action in state.get_available_actions() {
//                 frontier.push(node.apply(&action));
//                 generated += 1;
//             }
//         }
//         None
//     }
// }