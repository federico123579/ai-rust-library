use search::{State, Action, Space};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SudokuCell {
    value: Option<u8>,
}

impl SudokuCell {
    fn full(value: u8) -> Self {
        Self { value: Some(value) }
    }

    fn empty() -> Self {
        Self { value: None }
    }

    fn is_full(&self) -> bool {
        self.value.is_some()
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    fn value(&self) -> Option<u8> {
        self.value
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct SudokuBoard {
    board: [[SudokuCell; 9]; 9],
}

impl SudokuBoard {
    fn new() -> Self {
        Self {
            board: [[SudokuCell::empty(); 9]; 9],
        }
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.board[y][x] = SudokuCell::full(value);
    }

    fn clear(&mut self, x: usize, y: usize) {
        self.board[y][x] = SudokuCell::empty();
    }

    fn get(&self, x: usize, y: usize) -> SudokuCell {
        self.board[y][x]
    }

    fn get_value(&self, x: usize, y: usize) -> Option<u8> {
        self.board[y][x].value()
    }

    fn is_valid(&self) -> bool {
        for i in 0..9 {
            let mut row = [false; 9];
            let mut col = [false; 9];
            let mut square = [false; 9];
            for j in 0..9 {
                if let Some(value) = self.get_value(i, j) {
                    if row[value as usize - 1] {
                        return false;
                    }
                    row[value as usize - 1] = true;
                }
                if let Some(value) = self.get_value(j, i) {
                    if col[value as usize - 1] {
                        return false;
                    }
                    col[value as usize - 1] = true;
                }
                let x = (i % 3) * 3 + j % 3;
                let y = (i / 3) * 3 + j / 3;
                if let Some(value) = self.get_value(x, y) {
                    if square[value as usize - 1] {
                        return false;
                    }
                    square[value as usize - 1] = true;
                }
            }
        }
        true
    }
}

impl std::fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                if let Some(value) = self.get_value(i, j) {
                    write!(f, "{}", value)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Actions {
    Set(usize, usize, u8),
}

impl Actions {
    fn set(x: usize, y: usize, value: u8) -> Self {
        Self::Set(x, y, value)
    }
}

impl Action for Actions {}

impl State for SudokuBoard {
    type Action = Actions;

    fn get_available_actions(&self) -> Vec<Self::Action> {
        let mut actions = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if self.get(i, j).is_empty() {
                    for value in 1..10 {
                        actions.push(Actions::set(i, j, value));
                    }
                }
            }
        }

        // prune invalid actions
        actions.retain(|action| {
            let board = self.clone();
            board.apply(action);
            board.is_valid()
        });

        actions
    }

    fn apply(&self, action: &Self::Action) -> Self {
        let mut board = self.clone();
        match action {
            Actions::Set(x, y, value) => board.set(*x, *y, *value),
        }
        board
    }
}

struct SudokuSolver {
    board_to_solve: SudokuBoard,
}

impl Space for SudokuSolver {
    type Action = Actions;
    type State = SudokuBoard;

    fn initial_state(&self) -> Self::State {
        self.board_to_solve.clone()
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        state.get_available_actions().is_empty() & state.is_valid()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use search::DepthFirstSearch;

//     fn medium_sudoku() -> SudokuBoard {
//         let mut board = SudokuBoard::new();
//         board.set(0, 0, 5);
//         board.set(0, 1, 3);
//         board.set(0, 4, 7);
//         board.set(1, 0, 6);
//         board.set(1, 3, 1);
//         board.set(1, 4, 9);
//         board.set(1, 5, 5);
//         board.set(2, 1, 9);
//         board.set(2, 2, 8);
//         board.set(2, 7, 6);
//         board.set(3, 0, 8);
//         board.set(3, 4, 6);
//         board.set(3, 8, 3);
//         board.set(4, 0, 4);
//         board.set(4, 3, 8);
//         board.set(4, 5, 3);
//         board.set(4, 8, 1);
//         board.set(5, 0, 7);
//         board.set(5, 4, 2);
//         board.set(5, 8, 6);
//         board.set(6, 1, 6);
//         board.set(6, 6, 2);
//         board.set(6, 7, 8);
//         board.set(7, 3, 4);
//         board.set(7, 4, 1);
//         board.set(7, 5, 9);
//         board.set(7, 8, 5);
//         board.set(8, 4, 8);
//         board.set(8, 7, 7);
//         board.set(8, 8, 9);
//         board
//     }

//     #[test]
//     fn valid_medium_sudoku() {
//         let board = medium_sudoku();
//         println!("{}", board);
//         assert!(board.is_valid());
//     }

//     #[test]
//     fn solve_medium_sudoku() {
//         let board = medium_sudoku();
//         let solver = SudokuSolver {
//             board_to_solve: board,
//         };
//         let solution = solver.dfs_search();
//         assert!(solution.is_some());
//         let solution = solution.unwrap();
//         let state = solution.end_state;
//         println!("Breadth first search results:");
//         println!("  Generated: {}", solution.generated);
//         println!("  Expanded: {}", solution.expanded);
//         println!("  Path length: {}", solution.path.len());
//         println!("  Path: {:?}", solution.path);
//         println!("{}", &state);
//         assert!(state.is_valid());
//     }
// }

fn main() {
    use search::BreadthFirstSearch;
    fn medium_sudoku() -> SudokuBoard {
        let mut board = SudokuBoard::new();
        board.set(0, 0, 5);
        board.set(0, 1, 3);
        board.set(0, 4, 7);
        board.set(1, 0, 6);
        board.set(1, 3, 1);
        board.set(1, 4, 9);
        board.set(1, 5, 5);
        board.set(2, 1, 9);
        board.set(2, 2, 8);
        board.set(2, 7, 6);
        board.set(3, 0, 8);
        board.set(3, 4, 6);
        board.set(3, 8, 3);
        board.set(4, 0, 4);
        board.set(4, 3, 8);
        board.set(4, 5, 3);
        board.set(4, 8, 1);
        board.set(5, 0, 7);
        board.set(5, 4, 2);
        board.set(5, 8, 6);
        board.set(6, 1, 6);
        board.set(6, 6, 2);
        board.set(6, 7, 8);
        board.set(7, 3, 4);
        board.set(7, 4, 1);
        board.set(7, 5, 9);
        board.set(7, 8, 5);
        board.set(8, 4, 8);
        board.set(8, 7, 7);
        board.set(8, 8, 9);
        board
    }

    let board = medium_sudoku();
    let solver = SudokuSolver {
        board_to_solve: board,
    };
    let solution = solver.bfs_search();
    assert!(solution.is_some());
    let solution = solution.unwrap();
    let state = solution.end_state;
    println!("Breadth first search results:");
    println!("  Generated: {}", solution.generated);
    println!("  Expanded: {}", solution.expanded);
    println!("  Path length: {}", solution.path.len());
    println!("  Path: {:?}", solution.path);
    println!("{}", &state);
    assert!(state.is_valid());
}