use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, AppMode};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(11)])
        .split(frame.area());

    draw_list(frame, app, layout[0]);
    draw_editor(frame, app, layout[1]);
}

fn draw_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|i| ListItem::new(i.to_string()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Job applications")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol("-> ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_editor(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title("(a - add | e - edit | D - delete | Tab - switch | Enter - save | Esc - cancel | Q - quit)")
        .borders(Borders::ALL);

    let inner = block.inner(area);

    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .split(inner);
    let row0 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);

    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    let row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[2]);

    let company_name_style = if app.input.input_field == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let description_style = if app.input.input_field == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let origin_style = if app.input.input_field == 2 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let url_style = if app.input.input_field == 3 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let company = Paragraph::new(app.input.company_name.as_str())
        .style(company_name_style)
        .block(Block::default().title("Company").borders(Borders::ALL));

    let description = Paragraph::new(app.input.description.as_str())
        .style(description_style)
        .block(Block::default().title("Description").borders(Borders::ALL));

    let origin = Paragraph::new(app.input.origin.as_str())
        .style(origin_style)
        .block(Block::default().title("Origin").borders(Borders::ALL));

    let url = Paragraph::new(app.input.url.as_str())
        .style(url_style)
        .block(Block::default().title("URL").borders(Borders::ALL));

    frame.render_widget(company, row0[0]);
    frame.render_widget(description, row0[1]);
    frame.render_widget(origin, row1[0]);
    frame.render_widget(url, row1[1]);

    if matches!(app.mode, AppMode::Creating | AppMode::Editing) {
        let (x, y) = match app.input.input_field {
            0 => (
                row0[0].x + app.input.company_name.len() as u16 + 1,
                row0[0].y + 1,
            ),
            1 => (
                row0[1].x + app.input.description.len() as u16 + 1,
                row0[1].y + 1,
            ),
            _ => (0, 0),
        };

        frame.set_cursor_position(Position::new(x, y));
    }
}
