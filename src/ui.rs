use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::game::{Board, COLS, Cell, GameState, Player, ROWS};

const PIECE: &str = "●";
const EMPTY: &str = "·";
const CURSOR: &str = "▼";

fn player_color(player: Player) -> Color {
    match player {
        Player::Red => Color::Red,
        Player::Yellow => Color::Yellow,
    }
}

pub fn render(frame: &mut Frame, board: &Board, cursor_col: usize) {
    let area = frame.area();

    // Calculate board dimensions: each cell is 4 chars wide ("│ ● "), plus final "│"
    let board_inner_width: u16 = (COLS as u16) * 4 + 1;
    let board_inner_height: u16 = ROWS as u16 * 2 + 1 // grid lines + cell rows
        + 2  // cursor row + column numbers
        + 2  // status + help
        + 1; // spacing

    let block_width = board_inner_width + 4; // padding
    let block_height = board_inner_height + 2; // block borders

    // Center the board
    let horizontal = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(block_width),
        Constraint::Min(0),
    ])
    .split(area);

    let vertical = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(block_height),
        Constraint::Min(0),
    ])
    .split(horizontal[1]);

    let board_area = vertical[1];

    let winning = board.winning_cells().unwrap_or_default();

    let title_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" CONNECT ", title_style),
            Span::styled(
                "4",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let inner = block.inner(board_area);
    frame.render_widget(block, board_area);

    let mut lines: Vec<Line> = Vec::new();

    // Cursor row
    let cursor_line = build_cursor_line(cursor_col, board);
    lines.push(cursor_line);

    // Column numbers
    let numbers_line = build_numbers_line(cursor_col, board);
    lines.push(numbers_line);

    // Top border of grid
    lines.push(Line::from(Span::styled(
        format!("╔{}╗", "═══╤".repeat(COLS - 1).to_string() + "═══"),
        Style::default().fg(Color::Blue),
    )));

    // Grid rows
    for row in 0..ROWS {
        let mut spans = Vec::new();
        for col in 0..COLS {
            let sep = if col == 0 { "║" } else { "│" };
            spans.push(Span::styled(
                sep.to_string(),
                Style::default().fg(Color::Blue),
            ));
            spans.push(Span::raw(" "));

            let cell = board.cell(row, col);
            let is_winning = winning.contains(&(row, col));
            let span = match cell {
                Cell::Empty => Span::styled(EMPTY, Style::default().fg(Color::DarkGray)),
                Cell::Occupied(player) => {
                    let mut style = Style::default().fg(player_color(player));
                    if is_winning {
                        style = style.add_modifier(Modifier::BOLD | Modifier::REVERSED);
                    }
                    Span::styled(PIECE, style)
                }
            };
            spans.push(span);
            spans.push(Span::raw(" "));
        }
        spans.push(Span::styled("║", Style::default().fg(Color::Blue)));
        lines.push(Line::from(spans));

        // Row separator
        if row < ROWS - 1 {
            lines.push(Line::from(Span::styled(
                format!("╟{}╢", "───┼".repeat(COLS - 1).to_string() + "───"),
                Style::default().fg(Color::Blue),
            )));
        }
    }

    // Bottom border of grid
    lines.push(Line::from(Span::styled(
        format!("╚{}╝", "═══╧".repeat(COLS - 1).to_string() + "═══"),
        Style::default().fg(Color::Blue),
    )));

    // Blank line
    lines.push(Line::from(""));

    // Status line
    lines.push(build_status_line(board));

    // Help line
    lines.push(build_help_line(board));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}

fn build_cursor_line(cursor_col: usize, board: &Board) -> Line<'static> {
    if board.state() != GameState::InProgress {
        return Line::from("");
    }
    let color = player_color(board.current_player());
    let mut parts = String::new();
    // Offset to align with cell centers: "║ ● │ ● │ ..." — each cell is 4 chars
    // First cell offset: 2 chars ("║ "), then center is at +1 = char index 2
    // Actually with center alignment, we just space it out
    for col in 0..COLS {
        if col == 0 {
            parts.push(' ');
        } else {
            parts.push_str("   ");
        }
        if col == cursor_col {
            parts.push_str(CURSOR);
        } else {
            parts.push(' ');
        }
    }
    parts.push(' ');
    Line::from(Span::styled(parts, Style::default().fg(color)))
}

fn build_numbers_line(cursor_col: usize, board: &Board) -> Line<'static> {
    let mut spans = Vec::new();
    let active_color = player_color(board.current_player());
    for col in 0..COLS {
        if col == 0 {
            spans.push(Span::raw(" "));
        } else {
            spans.push(Span::raw("   "));
        }
        let num = format!("{}", col + 1);
        if col == cursor_col && board.state() == GameState::InProgress {
            spans.push(Span::styled(
                num,
                Style::default()
                    .fg(active_color)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(num, Style::default().fg(Color::DarkGray)));
        }
    }
    spans.push(Span::raw(" "));
    Line::from(spans)
}

fn build_status_line(board: &Board) -> Line<'static> {
    match board.state() {
        GameState::InProgress => {
            let player = board.current_player();
            let color = player_color(player);
            Line::from(vec![
                Span::styled(
                    format!("{}", player),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled("'s turn", Style::default().fg(Color::White)),
            ])
        }
        GameState::Won(player) => {
            let color = player_color(player);
            Line::from(vec![
                Span::styled(
                    format!("{}", player),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " wins! ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "(r to restart, q to quit)",
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        }
        GameState::Draw => Line::from(vec![
            Span::styled(
                "Draw! ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "(r to restart, q to quit)",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    }
}

fn build_help_line(board: &Board) -> Line<'static> {
    if board.state() != GameState::InProgress {
        return Line::from("");
    }
    Line::from(Span::styled(
        "←/→ move  ·  Enter drop  ·  q quit",
        Style::default().fg(Color::DarkGray),
    ))
}
