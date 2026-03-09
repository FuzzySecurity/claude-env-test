use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub enum Action {
    MoveLeft,
    MoveRight,
    Drop,
    Quit,
    Restart,
    None,
}

pub fn read_action() -> std::io::Result<Action> {
    if event::poll(Duration::from_millis(100))?
        && let Event::Key(key) = event::read()?
    {
        if key.kind != KeyEventKind::Press {
            return Ok(Action::None);
        }
        return Ok(match key.code {
            KeyCode::Left | KeyCode::Char('h') => Action::MoveLeft,
            KeyCode::Right | KeyCode::Char('l') => Action::MoveRight,
            KeyCode::Enter | KeyCode::Char(' ') => Action::Drop,
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
            KeyCode::Char('r') => Action::Restart,
            _ => Action::None,
        });
    }
    Ok(Action::None)
}
