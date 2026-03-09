mod ai;
mod game;
mod input;
mod ui;

use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{DefaultTerminal, Terminal, prelude::CrosstermBackend};

use game::{Board, COLS, GameState, Player};
use input::Action;

fn main() -> io::Result<()> {
    // Set up panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        original_hook(info);
    }));

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    restore_terminal()?;
    result
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut board = Board::new();
    let mut cursor_col: usize = 3;
    let mut ai_thinking = false;

    loop {
        terminal.draw(|frame| ui::render(frame, &board, cursor_col, ai_thinking))?;

        // AI's turn
        if board.state() == GameState::InProgress && board.current_player() == Player::Yellow {
            if !ai_thinking {
                ai_thinking = true;
                continue; // Re-render with "AI is thinking..." before computing
            }
            let col = ai::best_move(&board);
            let _ = board.drop_piece(col);
            ai_thinking = false;
            continue;
        }

        ai_thinking = false;

        match input::read_action()? {
            Action::MoveLeft => cursor_col = cursor_col.saturating_sub(1),
            Action::MoveRight => {
                if cursor_col < COLS - 1 {
                    cursor_col += 1;
                }
            }
            Action::Drop => {
                if board.state() == GameState::InProgress {
                    let _ = board.drop_piece(cursor_col);
                }
            }
            Action::Restart => {
                if board.state() != GameState::InProgress {
                    board.reset();
                    cursor_col = 3;
                }
            }
            Action::Quit => break,
            Action::None => {}
        }
    }

    Ok(())
}
