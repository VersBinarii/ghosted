use chrono::{Datelike, Duration, NaiveDate};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap},
};

use crate::{
    app::{App, AppMode},
    models::ApplicationStatus,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(14)])
        .split(frame.area());

    draw_list(frame, app, layout[0]);
    draw_status_update(frame, app, layout[0]);
    draw_editor(frame, app, layout[1]);
    draw_details_popup(frame, app, frame.area());
    draw_interview_date_picker(frame, app, frame.area());
}

fn draw_status_update(frame: &mut Frame, app: &mut App, area: Rect) {
    if matches!(app.mode(), AppMode::UpdateStatus) {
        let popup = centered_rect(20, 40, area);
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
                Cell::from(
                    app.interview_date
                        .map(|date| date.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
                Cell::from(app.application_date.format("%Y-%m-%d %H:%M").to_string()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(28),
        Constraint::Length(28),
        Constraint::Length(22),
        Constraint::Length(18),
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
                Cell::from("Interview"),
                Cell::from("Application date"),
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
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(6),
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

    let input = app.input();

    let company_style = active_field_style(is_editing, input.input_field, 0);
    let description_style = active_field_style(is_editing, input.input_field, 1);
    let origin_style = active_field_style(is_editing, input.input_field, 2);
    let url_style = active_field_style(is_editing, input.input_field, 3);
    let comments_style = active_field_style(is_editing, input.input_field, 4);

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

    let comments = Paragraph::new(input.comments.as_str())
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("Comments")
                .borders(Borders::ALL)
                .border_style(comments_style)
                .title_style(comments_style),
        );

    frame.render_widget(company, row0[0]);
    frame.render_widget(description, row0[1]);
    frame.render_widget(origin, row1[0]);
    frame.render_widget(url, row1[1]);
    frame.render_widget(comments, rows[2]);
}

fn draw_details_popup(frame: &mut Frame, app: &mut App, area: Rect) {
    if !matches!(app.mode(), AppMode::ViewingDetails) {
        return;
    }

    let Some(selected) = app.selected_item() else {
        return;
    };

    let popup = centered_rect(70, 70, area);
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(13), Constraint::Min(5)])
        .margin(1)
        .split(popup);

    let detail_rows = vec![
        Row::new(vec![
            Cell::from("Company"),
            Cell::from(selected.company_name.clone()),
        ]),
        Row::new(vec![
            Cell::from("Origin"),
            Cell::from(selected.origin.clone()),
        ]),
        Row::new(vec![
            Cell::from("Description"),
            Cell::from(selected.description.clone()),
        ]),
        Row::new(vec![Cell::from("URL"), Cell::from(selected.url.clone())]),
        Row::new(vec![
            Cell::from("Status"),
            Cell::from(selected.application_status.to_string()),
        ]),
        Row::new(vec![
            Cell::from("Interview"),
            Cell::from(
                selected
                    .interview_date
                    .map(|date| date.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ),
        ]),
        Row::new(vec![
            Cell::from("Date"),
            Cell::from(selected.application_date.to_rfc3339()),
        ]),
    ];

    let details_table = Table::new(detail_rows, [Constraint::Length(14), Constraint::Min(10)])
        .block(
            Block::default()
                .title("Application Details")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .column_spacing(1);

    let comments = Paragraph::new(selected.comments.as_str())
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("Comments")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title_style(Style::default().fg(Color::Cyan)),
        );

    frame.render_widget(Clear, popup);
    frame.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL),
        popup,
    );
    frame.render_widget(details_table, sections[0]);
    frame.render_widget(comments, sections[1]);
}

fn draw_interview_date_picker(frame: &mut Frame, app: &mut App, area: Rect) {
    if !matches!(app.mode(), AppMode::PickingInterviewDate) {
        return;
    }

    let selected = app.interview_picker_date();
    let selected_date = selected.date();
    let popup = centered_rect(40, 48, area);
    let month_start = NaiveDate::from_ymd_opt(selected_date.year(), selected_date.month(), 1)
        .expect("valid first day of month");

    let weekday_offset = month_start.weekday().num_days_from_monday() as i64;
    let grid_start = month_start - Duration::days(weekday_offset);

    let mut lines = vec![
        Line::from(Span::styled(
            format!("{} {}", selected_date.format("%B"), selected_date.year()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("Mo Tu We Th Fr Sa Su"),
    ];

    for week in 0..6 {
        let mut spans = Vec::new();

        for day in 0..7 {
            let date = grid_start + Duration::days((week * 7 + day) as i64);
            let mut style = if date.month() == selected_date.month() {
                Style::default()
            } else {
                Style::default().fg(Color::DarkGray)
            };

            if date == selected_date {
                style = style.bg(Color::Blue).fg(Color::White);
            }

            spans.push(Span::styled(format!("{:>2}", date.day()), style));

            if day < 6 {
                spans.push(Span::raw(" "));
            }
        }

        lines.push(Line::from(spans));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "Selected: {}",
        selected.format("%Y-%m-%d %H:%M")
    )));
    lines.push(Line::from("+/- hour  [/ ] 15 min"));
    lines.push(Line::from("←/→ day  ↑/↓ week  PgUp/PgDn month"));
    lines.push(Line::from("c clear  Enter save  Esc cancel"));

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .title("Interview Date & Time")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(Clear, popup);
    frame.render_widget(paragraph, popup);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::{App, AppMode},
        models::{Application, ApplicationStatus, InputApplication},
    };
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
    use ratatui::{Terminal, backend::TestBackend};

    fn sample_application() -> Application {
        Application {
            company_name: "Acme".to_string(),
            description: "Rust Developer".to_string(),
            url: "https://example.com/job".to_string(),
            comments: "Strong fit".to_string(),
            application_status: ApplicationStatus::Applied,
            origin: "LinkedIn".to_string(),
            application_date: Utc::now(),
            interview_date: Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 3, 20).expect("valid date"),
                NaiveTime::from_hms_opt(14, 30, 0).expect("valid time"),
            )),
        }
    }

    fn render(app: &mut App, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("terminal should be created");
        terminal
            .draw(|frame| ui(frame, app))
            .expect("ui should render");

        let backend = terminal.backend();
        let buffer = backend.buffer().clone();

        let mut text = String::new();

        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                text.push_str(buffer[(x, y)].symbol());
            }
            text.push('\n');
        }

        text
    }

    #[test]
    fn main_table_renders_headers_and_row_values() {
        let mut app = App::test_app();
        app.test_set_items(vec![sample_application()]);

        let output = render(&mut app, 140, 40);

        assert!(output.contains("Job Applications"));
        assert!(output.contains("Company"));
        assert!(output.contains("Origin"));
        assert!(output.contains("Description"));
        assert!(output.contains("URL"));
        assert!(output.contains("Status"));
        assert!(output.contains("Interview"));
        assert!(output.contains("Application date"));
        assert!(output.contains("Acme"));
        assert!(output.contains("LinkedIn"));
        assert!(output.contains("Rust Developer"));
        assert!(output.contains("Applied"));
    }

    #[test]
    fn editor_renders_input_fields_and_values_in_create_mode() {
        let mut app = App::test_app();
        app.test_set_mode(AppMode::Creating);
        app.test_set_input(InputApplication {
            company_name: "Acme".to_string(),
            description: "Backend".to_string(),
            url: "https://example.com".to_string(),
            origin: "Referral".to_string(),
            comments: "Need to follow up".to_string(),
            interview_date: None,
            input_field: 0,
        });

        let output = render(&mut app, 140, 40);

        assert!(output.contains(
            "Create: Tab next field | type to edit | Backspace delete | Enter save | Esc cancel"
        ));
        assert!(output.contains("Company"));
        assert!(output.contains("Description"));
        assert!(output.contains("Origin"));
        assert!(output.contains("URL"));
        assert!(output.contains("Comments"));
        assert!(output.contains("Acme"));
        assert!(output.contains("Backend"));
        assert!(output.contains("Referral"));
        assert!(output.contains("https://example.com"));
        assert!(output.contains("Need to follow up"));
    }

    #[test]
    fn status_popup_renders_when_updating_status() {
        let mut app = App::test_app();
        app.test_set_mode(AppMode::UpdateStatus);

        let output = render(&mut app, 140, 40);

        assert!(output.contains("Application State"));
        assert!(output.contains("Applied"));
        assert!(output.contains("Rejected"));
        assert!(output.contains("Accepted"));
        assert!(output.contains("Awaiting recruiter"));
        assert!(output.contains("Ghosted"));
        assert!(output.contains("Thinking about it"));
        assert!(output.contains("Interview scheduled"));
    }

    #[test]
    fn details_popup_renders_selected_application_details() {
        let mut app = App::test_app();
        app.test_set_items(vec![sample_application()]);
        app.test_set_mode(AppMode::ViewingDetails);

        let output = render(&mut app, 140, 40);

        assert!(output.contains("Application Details"));
        assert!(output.contains("Company"));
        assert!(output.contains("Origin"));
        assert!(output.contains("Description"));
        assert!(output.contains("URL"));
        assert!(output.contains("Status"));
        assert!(output.contains("Interview"));
        assert!(output.contains("Date"));
        assert!(output.contains("Acme"));
        assert!(output.contains("LinkedIn"));
        assert!(output.contains("Rust Developer"));
        assert!(output.contains("https://example.com/job"));
        assert!(output.contains("Strong fit"));
    }

    #[test]
    fn interview_picker_renders_calendar_and_help_text() {
        let mut app = App::test_app();
        app.test_set_mode(AppMode::PickingInterviewDate);
        app.test_set_picker_date(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 3, 20).expect("valid date"),
            NaiveTime::from_hms_opt(14, 30, 0).expect("valid time"),
        ));

        let output = render(&mut app, 140, 40);

        assert!(output.contains("Interview Date & Time"));
        assert!(output.contains("March 2025"));
        assert!(output.contains("Mo Tu We Th Fr Sa Su"));
        assert!(output.contains("Selected: 2025-03-20 14:30"));
        assert!(output.contains("+/- hour  [/ ] 15 min"));
        assert!(output.contains("PgUp/PgDn month"));
        assert!(output.contains("c clear  Enter save  Esc cancel"));
    }

    #[test]
    fn details_popup_is_not_rendered_in_normal_mode() {
        let mut app = App::test_app();
        app.test_set_items(vec![sample_application()]);
        app.test_set_mode(AppMode::Normal);

        let output = render(&mut app, 140, 40);

        assert!(!output.contains("Application Details"));
    }

    #[test]
    fn centered_rect_returns_middle_area() {
        let area = Rect::new(0, 0, 100, 50);

        let rect = centered_rect(40, 48, area);

        assert_eq!(rect.width, 40);
        assert_eq!(rect.height, 24);
        assert_eq!(rect.x, 30);
        assert_eq!(rect.y, 13);
    }

    #[test]
    fn active_field_style_is_cyan_only_for_selected_field_while_editing() {
        let selected = active_field_style(true, 2, 2);
        let not_selected = active_field_style(true, 2, 1);
        let not_editing = active_field_style(false, 2, 2);

        assert_eq!(selected.fg, Some(Color::Cyan));
        assert_eq!(not_selected.fg, None);
        assert_eq!(not_editing.fg, None);
    }
}
