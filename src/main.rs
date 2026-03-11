use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use ghosted::{app::App, ui};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let mut app = App::new()?;

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(key) = event::read()?
            && app.handle_key(key)
        {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
