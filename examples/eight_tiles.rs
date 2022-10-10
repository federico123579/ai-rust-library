use search::{self, State, Space, Action};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EightTilesAction {
    Up,
    Down,
    Left,
    Right,
}

impl Action for EightTilesAction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileType {
    Empty,
    Number(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EightTiles {
    tiles: [[TileType; 3]; 3],
}

impl EightTiles {
    pub fn new(tiles: [[u8; 3]; 3]) -> EightTiles {
        let mut new_tiles = [[TileType::Empty; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_tiles[i][j] = match tiles[i][j] {
                    0 => TileType::Empty,
                    n => TileType::Number(n),
                };
            }
        }
        let obj = EightTiles { tiles: new_tiles };
        obj.check_validity();
        obj
    }

    fn check_validity(&self) {
        let mut empty_count = 0;
        let mut numbers = [false; 9];
        for i in 0..3 {
            for j in 0..3 {
                match self.tiles[i][j] {
                    TileType::Empty => {
                        assert_eq!(empty_count, 0);
                        empty_count += 1;
                    }
                    TileType::Number(n) => {
                        assert!(n > 0 && n < 9);
                        assert!(!numbers[n as usize]);
                        numbers[n as usize] = true;
                    }
                }
            }
        }
    }

    fn find_empty(&self) -> (usize, usize) {
        for i in 0..3 {
            for j in 0..3 {
                if let TileType::Empty = self.tiles[i][j] {
                    return (j, i);
                }
            }
        }
        panic!("No empty tile found");
    }

    fn solved() -> Self {
        EightTiles::new([[1, 2, 3], [4, 5, 6], [7, 8, 0]])
    }
}

impl State for EightTiles {
    type Action = EightTilesAction;

    fn get_available_actions(&self) -> Vec<Self::Action> {
        let mut actions = Vec::new();
        let (x, y) = self.find_empty();
        if x > 0 {
            actions.push(EightTilesAction::Left);
        }
        if x < 2 {
            actions.push(EightTilesAction::Right);
        }
        if y > 0 {
            actions.push(EightTilesAction::Up);
        }
        if y < 2 {
            actions.push(EightTilesAction::Down);
        }
        actions
    }

    fn apply(&self, action: &Self::Action) -> Self {
        let mut tiles = self.tiles;
        let (x, y) = self.find_empty();
        let (dx, dy): (isize, isize) = match action {
            EightTilesAction::Left => (-1, 0),
            EightTilesAction::Right => (1, 0),
            EightTilesAction::Up => (0, -1),
            EightTilesAction::Down => (0, 1),
        };
        let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        tiles[y][x] = tiles[ny][nx];
        tiles[ny][nx] = TileType::Empty;
        EightTiles { tiles }
    }
}

impl std::fmt::Display for EightTiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                match self.tiles[i][j] {
                    TileType::Empty => write!(f, "  ")?,
                    TileType::Number(n) => write!(f, "{:2}", n)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct EightTilesSpace {
    initial_state: EightTiles,
}

impl EightTilesSpace {
    pub fn new(initial_state: EightTiles) -> EightTilesSpace {
        EightTilesSpace { initial_state }
    }
}

impl search::Space for EightTilesSpace {
    type State = EightTiles;
    type Action = EightTilesAction;

    fn initial_state(&self) -> Self::State {
        self.initial_state.clone()
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        *state == EightTiles::solved()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use search::{DepthFirstSearch, BreadthFirstSearch};

    mod test_utils {
        use super::*;

        use crate::EightTilesSpace;

        pub fn get_state_space(tiles: [[u8; 3]; 3]) -> EightTilesSpace {
            EightTilesSpace::new(EightTiles::new(tiles))
        }

        pub fn get_easy_problem_space() -> EightTilesSpace {
            get_state_space([[1, 2, 3], [4, 5, 6], [7, 0, 8]])
        }

        // pub fn get_hard_problem_space() -> EightTilesSpace {
        //     get_state_space([[2, 7, 3], [1, 6, 4], [8, 0, 5]])
        // }
    }

    #[test]
    fn search_with_dfs() {
        let space = test_utils::get_easy_problem_space();
        let result = space.dfs_search();
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(space.is_goal(&result.end_state));
        assert!(result.path.len() > 1);
        assert!(result.path.contains(&EightTilesAction::Right));
        assert!(result.generated > result.expanded);
        println!("Depth first search results:");
        println!("  Generated: {}", result.generated);
        println!("  Expanded: {}", result.expanded);
        println!("  Path length: {}", result.path.len());
        println!("  Path: {:?}", result.path);
    }

    #[test]
    fn search_with_bfs() {
        let space = test_utils::get_easy_problem_space();
        let result = space.bfs_search();
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(space.is_goal(&result.end_state));
        assert_eq!(result.path.len(), 1);
        assert_eq!(result.path[0], EightTilesAction::Right);
        assert!(result.generated > result.expanded);
        println!("Breadth first search results:");
        println!("  Generated: {}", result.generated);
        println!("  Expanded: {}", result.expanded);
        println!("  Path length: {}", result.path.len());
        println!("  Path: {:?}", result.path);
    }
}