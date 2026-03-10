use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ghosted::{
    app::{App, AppMode},
    ui,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let mut app = App::new()?;

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(key) = event::read()?
            && handle_input(&mut app, key)
        {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn handle_input(app: &mut App, key: KeyEvent) -> bool {
    match app.mode {
        AppMode::Normal => match key.code {
            KeyCode::Char('Q') => return true,
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Char('a') => app.start_create(),
            KeyCode::Char('e') => app.start_edit(),
            KeyCode::Char('D') => app.delete(),
            KeyCode::Char('s') => app.update_status(),
            _ => {}
        },

        AppMode::Creating | AppMode::Editing => match key.code {
            KeyCode::Esc => app.cancel(),
            KeyCode::Enter => app.confirm(),

            KeyCode::Tab => {
                app.input.input_field = (app.input.input_field + 1) % 4;
            }

            KeyCode::Backspace => match app.input.input_field {
                0 => {
                    app.input.company_name.pop();
                }
                1 => {
                    app.input.description.pop();
                }
                2 => {
                    app.input.origin.pop();
                }
                3 => {
                    app.input.url.pop();
                }
                _ => {}
            },

            KeyCode::Char(c) => match app.input.input_field {
                0 => app.input.company_name.push(c),
                1 => app.input.description.push(c),
                2 => app.input.origin.push(c),
                3 => app.input.url.push(c),
                _ => {}
            },

            _ => {}
        },
        AppMode::UpdateStatus => match key.code {
            KeyCode::Down => app.next_status(),
            KeyCode::Up => app.previous_status(),
            KeyCode::Esc => app.cancel_status_update(),
            KeyCode::Enter => app.confirm_status_update(),
            _ => {}
        },
    }

    false
}
