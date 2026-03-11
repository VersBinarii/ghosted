use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
};

use crate::{
    app::{App, AppMode},
    models::ApplicationStatus,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(11)])
        .split(frame.area());

    draw_list(frame, app, layout[0]);
    draw_status_update(frame, app, layout[0]);
    draw_editor(frame, app, layout[1]);
}

fn draw_status_update(frame: &mut Frame, app: &mut App, area: Rect) {
    if matches!(app.mode(), AppMode::UpdateStatus) {
        let popup = centered_rect(20, 30, area);
        let items: Vec<ListItem> = ApplicationStatus::ALL
            .iter()
            .map(|a| ListItem::new(a.to_string()))
            .collect();
        let highlight_symbol = app.highlight_symbol().to_string();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Application State")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol(highlight_symbol.as_str());

        frame.render_stateful_widget(list, popup, app.application_list_state_mut());
    }
}

fn draw_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let rows: Vec<Row> = app
        .items()
        .iter()
        .map(|app| {
            Row::new(vec![
                Cell::from(app.company_name.clone()),
                Cell::from(app.origin.clone()),
                Cell::from(app.description.clone()),
                Cell::from(app.url.clone()),
                Cell::from(app.application_status.to_string()),
                Cell::from(app.application_date.to_rfc3339()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(45),
        Constraint::Length(45),
        Constraint::Length(16),
        Constraint::Length(25),
    ];

    let highlight_symbol = app.highlight_symbol().to_string();

    let table = Table::new(rows, widths)
        .header(
            Row::new(vec![
                Cell::from("Company"),
                Cell::from("Origin"),
                Cell::from("Description"),
                Cell::from("URL"),
                Cell::from("Status"),
                Cell::from("Date"),
            ])
            .style(Style::default().fg(Color::Yellow)),
        )
        .block(
            Block::default()
                .title("Job Applications")
                .borders(Borders::ALL),
        )
        .row_highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(highlight_symbol.as_str());

    frame.render_stateful_widget(table, area, app.table_state_mut());
}

fn draw_editor(frame: &mut Frame, app: &mut App, area: Rect) {
    let is_editing = matches!(app.mode(), AppMode::Creating | AppMode::Editing);

    let section_style = if is_editing {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let block = Block::default()
        .title(app.usage())
        .borders(Borders::ALL)
        .border_style(section_style);

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

    let _row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[2]);

    let input = app.input();

    let company_style = active_field_style(is_editing, input.input_field, 0);
    let description_style = active_field_style(is_editing, input.input_field, 1);
    let origin_style = active_field_style(is_editing, input.input_field, 2);
    let url_style = active_field_style(is_editing, input.input_field, 3);

    let company = Paragraph::new(input.company_name.as_str()).block(
        Block::default()
            .title("Company")
            .borders(Borders::ALL)
            .border_style(company_style)
            .title_style(company_style),
    );

    let description = Paragraph::new(input.description.as_str()).block(
        Block::default()
            .title("Description")
            .borders(Borders::ALL)
            .border_style(description_style)
            .title_style(description_style),
    );

    let origin = Paragraph::new(input.origin.as_str()).block(
        Block::default()
            .title("Origin")
            .borders(Borders::ALL)
            .border_style(origin_style)
            .title_style(origin_style),
    );

    let url = Paragraph::new(input.url.as_str()).block(
        Block::default()
            .title("URL")
            .borders(Borders::ALL)
            .border_style(url_style)
            .title_style(url_style),
    );

    frame.render_widget(company, row0[0]);
    frame.render_widget(description, row0[1]);
    frame.render_widget(origin, row1[0]);
    frame.render_widget(url, row1[1]);
}

fn active_field_style(is_editing: bool, selected_field: usize, field_index: usize) -> Style {
    if is_editing && selected_field == field_index {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    vertical[1]
}
