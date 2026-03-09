use std::ops::Not;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Red,
    Yellow,
}

impl Not for Player {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Red => write!(f, "Red"),
            Player::Yellow => write!(f, "Yellow"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    ColumnFull,
    ColumnOutOfBounds,
    GameOver,
}

pub struct Board {
    grid: [[Cell; COLS]; ROWS],
    current_player: Player,
    state: GameState,
    move_count: u32,
    last_move: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; COLS]; ROWS],
            current_player: Player::Red,
            state: GameState::InProgress,
            move_count: 0,
            last_move: None,
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn cell(&self, row: usize, col: usize) -> Cell {
        self.grid[row][col]
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<(usize, usize), MoveError> {
        if col >= COLS {
            return Err(MoveError::ColumnOutOfBounds);
        }
        if self.state != GameState::InProgress {
            return Err(MoveError::GameOver);
        }
        if self.grid[0][col] != Cell::Empty {
            return Err(MoveError::ColumnFull);
        }

        // Find lowest empty row
        let row = (0..ROWS)
            .rev()
            .find(|&r| self.grid[r][col] == Cell::Empty)
            .unwrap();

        self.grid[row][col] = Cell::Occupied(self.current_player);
        self.move_count += 1;
        self.last_move = Some((row, col));

        if self.check_win(row, col, self.current_player) {
            self.state = GameState::Won(self.current_player);
        } else if self.move_count == (ROWS * COLS) as u32 {
            self.state = GameState::Draw;
        } else {
            self.current_player = !self.current_player;
        }

        Ok((row, col))
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn winning_cells(&self) -> Option<Vec<(usize, usize)>> {
        let player = match self.state {
            GameState::Won(p) => p,
            _ => return None,
        };
        let (row, col) = self.last_move?;
        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

        for &(dr, dc) in &directions {
            let mut cells = vec![(row, col)];
            // Check positive direction
            Self::collect_direction(&self.grid, row, col, dr, dc, player, &mut cells);
            // Check negative direction
            Self::collect_direction(&self.grid, row, col, -dr, -dc, player, &mut cells);

            if cells.len() >= 4 {
                return Some(cells);
            }
        }
        None
    }

    fn check_win(&self, row: usize, col: usize, player: Player) -> bool {
        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
        for &(dr, dc) in &directions {
            let count = 1
                + self.count_direction(row, col, dr, dc, player)
                + self.count_direction(row, col, -dr, -dc, player);
            if count >= 4 {
                return true;
            }
        }
        false
    }

    fn count_direction(&self, row: usize, col: usize, dr: isize, dc: isize, player: Player) -> u32 {
        let mut count = 0;
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        while r >= 0
            && r < ROWS as isize
            && c >= 0
            && c < COLS as isize
            && self.grid[r as usize][c as usize] == Cell::Occupied(player)
        {
            count += 1;
            r += dr;
            c += dc;
        }
        count
    }

    fn collect_direction(
        grid: &[[Cell; COLS]; ROWS],
        row: usize,
        col: usize,
        dr: isize,
        dc: isize,
        player: Player,
        cells: &mut Vec<(usize, usize)>,
    ) {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        while r >= 0
            && r < ROWS as isize
            && c >= 0
            && c < COLS as isize
            && grid[r as usize][c as usize] == Cell::Occupied(player)
        {
            cells.push((r as usize, c as usize));
            r += dr;
            c += dc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play_moves(board: &mut Board, cols: &[usize]) {
        for &col in cols {
            board.drop_piece(col).unwrap();
        }
    }

    #[test]
    fn new_board() {
        let board = Board::new();
        assert_eq!(board.current_player(), Player::Red);
        assert_eq!(board.state(), GameState::InProgress);
        for r in 0..ROWS {
            for c in 0..COLS {
                assert_eq!(board.cell(r, c), Cell::Empty);
            }
        }
    }

    #[test]
    fn drop_piece_lands_at_bottom() {
        let mut board = Board::new();
        let (row, col) = board.drop_piece(3).unwrap();
        assert_eq!((row, col), (5, 3));
        assert_eq!(board.cell(5, 3), Cell::Occupied(Player::Red));
    }

    #[test]
    fn drop_piece_stacks() {
        let mut board = Board::new();
        board.drop_piece(3).unwrap(); // Red at row 5
        board.drop_piece(3).unwrap(); // Yellow at row 4
        assert_eq!(board.cell(5, 3), Cell::Occupied(Player::Red));
        assert_eq!(board.cell(4, 3), Cell::Occupied(Player::Yellow));
    }

    #[test]
    fn column_full() {
        let mut board = Board::new();
        for _ in 0..ROWS {
            board.drop_piece(0).unwrap();
        }
        assert_eq!(board.drop_piece(0), Err(MoveError::ColumnFull));
    }

    #[test]
    fn column_out_of_bounds() {
        let mut board = Board::new();
        assert_eq!(board.drop_piece(7), Err(MoveError::ColumnOutOfBounds));
        assert_eq!(board.drop_piece(100), Err(MoveError::ColumnOutOfBounds));
    }

    #[test]
    fn game_over_error() {
        let mut board = Board::new();
        // Red wins horizontally on bottom row
        play_moves(&mut board, &[0, 0, 1, 1, 2, 2, 3]);
        assert_eq!(board.state(), GameState::Won(Player::Red));
        assert_eq!(board.drop_piece(4), Err(MoveError::GameOver));
    }

    #[test]
    fn horizontal_win() {
        let mut board = Board::new();
        // Red: 0,1,2,3  Yellow: 0,1,2
        play_moves(&mut board, &[0, 0, 1, 1, 2, 2, 3]);
        assert_eq!(board.state(), GameState::Won(Player::Red));
    }

    #[test]
    fn vertical_win() {
        let mut board = Board::new();
        // Red plays col 0, Yellow plays col 1, alternating
        play_moves(&mut board, &[0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(board.state(), GameState::Won(Player::Red));
    }

    #[test]
    fn diagonal_ascending_win() {
        let mut board = Board::new();
        // Build a / diagonal for Red
        // Col layout (bottom to top):
        // Col 0: R
        // Col 1: Y R
        // Col 2: R Y R
        // Col 3: Y R Y R
        play_moves(&mut board, &[0, 1, 1, 2, 3, 2, 2, 3, 3, 6, 3]);
        assert_eq!(board.state(), GameState::Won(Player::Red));
    }

    #[test]
    fn diagonal_descending_win() {
        let mut board = Board::new();
        // Build a \ diagonal for Red
        // Col 3: R
        // Col 2: Y R
        // Col 1: R Y R
        // Col 0: Y R Y R
        play_moves(&mut board, &[3, 2, 2, 1, 0, 1, 1, 0, 0, 6, 0]);
        assert_eq!(board.state(), GameState::Won(Player::Red));
    }

    #[test]
    fn no_false_win_at_three() {
        let mut board = Board::new();
        play_moves(&mut board, &[0, 0, 1, 1, 2, 2]);
        assert_eq!(board.state(), GameState::InProgress);
    }

    #[test]
    fn draw_detection() {
        let mut board = Board::new();
        // Fill the board without anyone winning
        // Pattern: fill columns with alternating groups to avoid 4-in-a-row
        // Col: 0  1  2  3  4  5  6
        // R5:  R  Y  R  Y  R  Y  R
        // R4:  R  Y  R  Y  R  Y  R
        // R3:  Y  R  Y  R  Y  R  Y
        // R2:  Y  R  Y  R  Y  R  Y
        // R1:  R  Y  R  Y  R  Y  R
        // R0:  R  Y  R  Y  R  Y  R
        // We need to carefully construct moves that fill the board with no winner.
        // Use a known draw sequence:
        for _ in 0..3 {
            for col in 0..COLS {
                let _ = board.drop_piece(col);
            }
        }
        // This alternates R/Y across columns, filling 3 rows per pass (6 rows total = 42 moves)
        // But we need to verify no win occurs. Let's use a different approach.
        board.reset();

        // Fill column by column in a pattern that avoids 4-in-a-row
        // Each column gets: [R,R,Y,Y,R,R] or [Y,Y,R,R,Y,Y] alternating
        // But we need to alternate turns, so let's use two columns at a time
        // Actually, let's just construct it directly:
        // Fill paired columns in blocks of 3 to avoid 4-in-a-row
        for _ in 0..3 {
            board.drop_piece(0).unwrap();
            board.drop_piece(1).unwrap();
        }
        // Col 0 bottom-up: R,R,R and col 1: Y,Y,Y — that's 3 in a row max
        // Now swap for remaining 3 rows
        for _ in 0..3 {
            board.drop_piece(1).unwrap();
            board.drop_piece(0).unwrap();
        }
        // Col 0: R,R,R,Y,Y,Y  Col 1: Y,Y,Y,R,R,R — no 4-in-a-row

        // Repeat for cols 2,3
        for _ in 0..3 {
            board.drop_piece(2).unwrap();
            board.drop_piece(3).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(3).unwrap();
            board.drop_piece(2).unwrap();
        }

        // Repeat for cols 4,5
        for _ in 0..3 {
            board.drop_piece(4).unwrap();
            board.drop_piece(5).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(5).unwrap();
            board.drop_piece(4).unwrap();
        }

        // Col 6: 6 pieces alternating
        for _ in 0..6 {
            board.drop_piece(6).unwrap();
        }

        assert_eq!(board.state(), GameState::Draw);
    }

    #[test]
    fn player_alternation() {
        let mut board = Board::new();
        assert_eq!(board.current_player(), Player::Red);
        board.drop_piece(0).unwrap();
        assert_eq!(board.current_player(), Player::Yellow);
        board.drop_piece(1).unwrap();
        assert_eq!(board.current_player(), Player::Red);
    }

    #[test]
    fn reset_board() {
        let mut board = Board::new();
        play_moves(&mut board, &[0, 0, 1, 1, 2, 2, 3]);
        assert_eq!(board.state(), GameState::Won(Player::Red));
        board.reset();
        assert_eq!(board.state(), GameState::InProgress);
        assert_eq!(board.current_player(), Player::Red);
        assert_eq!(board.cell(5, 0), Cell::Empty);
    }

    #[test]
    fn winning_cells_horizontal() {
        let mut board = Board::new();
        play_moves(&mut board, &[0, 0, 1, 1, 2, 2, 3]);
        let cells = board.winning_cells().unwrap();
        assert!(cells.len() >= 4);
        // Should contain the bottom row, cols 0-3
        for col in 0..4 {
            assert!(cells.contains(&(5, col)));
        }
    }

    #[test]
    fn winning_cells_none_when_in_progress() {
        let board = Board::new();
        assert!(board.winning_cells().is_none());
    }
}
