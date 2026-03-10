use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
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
    if let AppMode::UpdateStatus = app.mode {
        let popup = centered_rect(20, 30, area);
        let items: Vec<ListItem> = ApplicationStatus::ALL
            .iter()
            .map(|a| ListItem::new(a.to_string()))
            .collect();
        let list = List::new(items)
            .block(
                Block::default()
                    .title("Application State")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol(app.highlight_symbol.as_str());

        frame.render_stateful_widget(list, popup, &mut app.application_list_state);
    }
}

fn draw_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let rows: Vec<Row> = app
        .items
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
        Constraint::Length(20), // Company
        Constraint::Length(20), // Origin
        Constraint::Length(45), // Description
        Constraint::Length(45), // URL
        Constraint::Length(16), // Status
        Constraint::Length(25), // Date
    ];

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
        .highlight_symbol(app.highlight_symbol.as_str());

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

fn draw_editor(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default().title(app.usage()).borders(Borders::ALL);

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
