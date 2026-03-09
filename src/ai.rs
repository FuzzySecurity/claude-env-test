use crate::game::{Board, COLS, Cell, Player, ROWS};

const MAX_DEPTH: u32 = 8;
const WIN_SCORE: i32 = 1_000_000;
const MOVE_ORDER: [usize; COLS] = [3, 2, 4, 1, 5, 0, 6];

#[derive(Clone)]
struct SimBoard {
    grid: [[Option<bool>; COLS]; ROWS], // true = AI (Yellow), false = Human (Red)
    heights: [usize; COLS],             // number of pieces in each column
    current_is_ai: bool,
    move_count: u32,
}

impl SimBoard {
    fn from_board(board: &Board) -> Self {
        let mut grid = [[None; COLS]; ROWS];
        let mut heights = [0usize; COLS];

        for col in 0..COLS {
            for row in (0..ROWS).rev() {
                match board.cell(row, col) {
                    Cell::Occupied(Player::Yellow) => {
                        grid[row][col] = Some(true);
                        heights[col] += 1;
                    }
                    Cell::Occupied(Player::Red) => {
                        grid[row][col] = Some(false);
                        heights[col] += 1;
                    }
                    Cell::Empty => {}
                }
            }
        }

        let move_count: u32 = heights.iter().sum::<usize>() as u32;
        let current_is_ai = board.current_player() == Player::Yellow;

        Self {
            grid,
            heights,
            current_is_ai,
            move_count,
        }
    }

    fn is_valid(&self, col: usize) -> bool {
        col < COLS && self.heights[col] < ROWS
    }

    fn drop(&mut self, col: usize) {
        let row = ROWS - 1 - self.heights[col];
        self.grid[row][col] = Some(self.current_is_ai);
        self.heights[col] += 1;
        self.move_count += 1;
        self.current_is_ai = !self.current_is_ai;
    }

    fn check_win_at(&self, row: usize, col: usize, is_ai: bool) -> bool {
        let target = Some(is_ai);
        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

        for &(dr, dc) in &directions {
            let mut count = 1u32;
            for &sign in &[1isize, -1isize] {
                let mut r = row as isize + dr * sign;
                let mut c = col as isize + dc * sign;
                while r >= 0
                    && r < ROWS as isize
                    && c >= 0
                    && c < COLS as isize
                    && self.grid[r as usize][c as usize] == target
                {
                    count += 1;
                    r += dr * sign;
                    c += dc * sign;
                }
            }
            if count >= 4 {
                return true;
            }
        }
        false
    }

    fn last_drop_row(&self, col: usize) -> usize {
        ROWS - self.heights[col]
    }
}

pub fn best_move(board: &Board) -> usize {
    best_move_with_depth(board, MAX_DEPTH)
}

pub fn best_move_with_depth(board: &Board, depth: u32) -> usize {
    let sim = SimBoard::from_board(board);
    let (_, col) = minimax(&sim, depth, i32::MIN, i32::MAX, sim.current_is_ai, depth);
    col.expect("AI should always find a valid move")
}

fn minimax(
    board: &SimBoard,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
    max_depth: u32,
) -> (i32, Option<usize>) {
    if board.move_count == (ROWS * COLS) as u32 {
        return (0, None);
    }
    if depth == 0 {
        return (evaluate(board), None);
    }

    let mut best_col: Option<usize> = None;

    if maximizing {
        let mut best_score = i32::MIN;
        for &col in &MOVE_ORDER {
            if !board.is_valid(col) {
                continue;
            }
            let mut child = board.clone();
            child.drop(col);

            let row = child.last_drop_row(col);
            if child.check_win_at(row, col, true) {
                let score = WIN_SCORE - (max_depth - depth) as i32;
                return (score, Some(col));
            }

            let (score, _) = minimax(&child, depth - 1, alpha, beta, false, max_depth);
            if score > best_score {
                best_score = score;
                best_col = Some(col);
            }
            alpha = alpha.max(best_score);
            if alpha >= beta {
                break;
            }
        }
        (best_score, best_col)
    } else {
        let mut best_score = i32::MAX;
        for &col in &MOVE_ORDER {
            if !board.is_valid(col) {
                continue;
            }
            let mut child = board.clone();
            child.drop(col);

            let row = child.last_drop_row(col);
            if child.check_win_at(row, col, false) {
                let score = -WIN_SCORE + (max_depth - depth) as i32;
                return (score, Some(col));
            }

            let (score, _) = minimax(&child, depth - 1, alpha, beta, true, max_depth);
            if score < best_score {
                best_score = score;
                best_col = Some(col);
            }
            beta = beta.min(best_score);
            if alpha >= beta {
                break;
            }
        }
        (best_score, best_col)
    }
}

fn evaluate(board: &SimBoard) -> i32 {
    let mut score = 0i32;

    // Center column bonus
    for row in 0..ROWS {
        match board.grid[row][3] {
            Some(true) => score += 3,
            Some(false) => score -= 3,
            None => {}
        }
    }

    // Scan all 4-cell windows
    for row in 0..ROWS {
        for col in 0..COLS {
            // Horizontal
            if col + 3 < COLS {
                score += score_window(board, row, col, 0, 1);
            }
            // Vertical
            if row + 3 < ROWS {
                score += score_window(board, row, col, 1, 0);
            }
            // Diagonal down-right
            if row + 3 < ROWS && col + 3 < COLS {
                score += score_window(board, row, col, 1, 1);
            }
            // Diagonal down-left
            if row + 3 < ROWS && col >= 3 {
                score += score_window(board, row, col, 1, -1);
            }
        }
    }

    score
}

fn score_window(board: &SimBoard, row: usize, col: usize, dr: usize, dc: isize) -> i32 {
    let mut ai_count = 0u32;
    let mut human_count = 0u32;

    for i in 0..4 {
        let r = row + dr * i;
        let c = (col as isize + dc * i as isize) as usize;
        match board.grid[r][c] {
            Some(true) => ai_count += 1,
            Some(false) => human_count += 1,
            None => {}
        }
    }

    if human_count == 0 {
        match ai_count {
            3 => 50,
            2 => 5,
            _ => 0,
        }
    } else if ai_count == 0 {
        match human_count {
            3 => -50,
            2 => -5,
            _ => 0,
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    fn play_moves(board: &mut Board, cols: &[usize]) {
        for &col in cols {
            board.drop_piece(col).unwrap();
        }
    }

    #[test]
    fn test_sim_board_from_empty() {
        let board = Board::new();
        let sim = SimBoard::from_board(&board);
        assert_eq!(sim.move_count, 0);
        assert!(sim.heights.iter().all(|&h| h == 0));
        for row in 0..ROWS {
            for col in 0..COLS {
                assert_eq!(sim.grid[row][col], None);
            }
        }
    }

    #[test]
    fn test_sim_board_from_midgame() {
        let mut board = Board::new();
        play_moves(&mut board, &[3, 3, 4, 2]);
        let sim = SimBoard::from_board(&board);
        assert_eq!(sim.move_count, 4);
        assert_eq!(sim.grid[5][3], Some(false)); // Red at bottom of col 3
        assert_eq!(sim.grid[4][3], Some(true)); // Yellow above
        assert_eq!(sim.grid[5][4], Some(false)); // Red at bottom of col 4
        assert_eq!(sim.grid[5][2], Some(true)); // Yellow at bottom of col 2
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut board = Board::new();
        // Set up: Yellow has 3 in a row at bottom, needs col 3 to win
        // Red: 0,1,2  Yellow: 0,1,2 (stacked on Red)
        // Then Red plays col 6, Yellow should win at col 3
        // Actually: let's make Yellow have 3 at row 4, cols 0,1,2
        // R drops 0, Y drops 0, R drops 1, Y drops 1, R drops 2, Y drops 2 => stacked pairs
        // R drops 6, Y should play 3 to get 4 at row 4 (cols 0-3)
        // Wait, Yellow is at row 4 cols 0,1,2. Let me trace:
        // Move 1: R->col0 (row5), Move 2: Y->col0 (row4)
        // Move 3: R->col1 (row5), Move 4: Y->col1 (row4)
        // Move 5: R->col2 (row5), Move 6: Y->col2 (row4)
        // Move 7: R->col6 (row5), now it's Yellow's turn with 3-in-a-row at row4
        play_moves(&mut board, &[0, 0, 1, 1, 2, 2, 6]);
        assert_eq!(board.current_player(), Player::Yellow);
        let col = best_move_with_depth(&board, 4);
        assert_eq!(col, 3); // Win by completing row 4
    }

    #[test]
    fn test_ai_blocks_opponent_win() {
        let mut board = Board::new();
        // Red has 3 in bottom row (cols 0,1,2), Yellow pieces are scattered (no threat).
        // R->0, Y->6, R->1, Y->4, R->2, Y->6, R->5
        // Red: (5,0),(5,1),(5,2),(5,5)  Yellow: (5,6),(5,4),(4,6)
        // Yellow's turn, must block col 3 to prevent Red's horizontal win.
        play_moves(&mut board, &[0, 6, 1, 4, 2, 6, 5]);
        assert_eq!(board.current_player(), Player::Yellow);
        let col = best_move_with_depth(&board, 4);
        assert_eq!(col, 3);
    }

    #[test]
    fn test_ai_center_preference_opening() {
        let mut board = Board::new();
        // Red plays first (col 0), then it's AI's turn
        board.drop_piece(0).unwrap();
        let col = best_move_with_depth(&board, 6);
        assert_eq!(col, 3); // Center is optimal
    }

    #[test]
    fn test_ai_returns_valid_column() {
        let mut board = Board::new();
        play_moves(&mut board, &[0, 1, 2, 3]);
        let col = best_move(&board);
        assert!(col < COLS);
    }

    #[test]
    fn test_ai_handles_nearly_full_board() {
        let mut board = Board::new();
        // Fill columns 0-5 completely (36 pieces), leave col 6 open
        for _ in 0..3 {
            board.drop_piece(0).unwrap();
            board.drop_piece(1).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(1).unwrap();
            board.drop_piece(0).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(2).unwrap();
            board.drop_piece(3).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(3).unwrap();
            board.drop_piece(2).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(4).unwrap();
            board.drop_piece(5).unwrap();
        }
        for _ in 0..3 {
            board.drop_piece(5).unwrap();
            board.drop_piece(4).unwrap();
        }
        // Cols 0-5 full, col 6 empty, 36 moves played
        // It's Red's turn, play Red in col 6
        board.drop_piece(6).unwrap();
        // Now Yellow's turn, only col 6 is valid
        let col = best_move(&board);
        assert_eq!(col, 6);
    }

    #[test]
    fn test_ai_prefers_faster_win() {
        let mut board = Board::new();
        // Set up a position where AI can win immediately in one specific column
        // R->0, Y->3, R->0, Y->3, R->0, Y->3, R->6
        // Yellow has 3 stacked in col 3 (rows 5,4,3), can win by playing col 3 again
        play_moves(&mut board, &[0, 3, 0, 3, 0, 3, 6]);
        assert_eq!(board.current_player(), Player::Yellow);
        let col = best_move_with_depth(&board, 4);
        assert_eq!(col, 3); // Immediate vertical win
    }
}
