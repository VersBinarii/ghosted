use std::path::PathBuf;

use color_eyre::eyre;
use ratatui::widgets::TableState;

use crate::{
    db::{load_db, parse_cli_file, resolve_data_path, save_db},
    models::{Application, InputApplication},
};

pub enum AppMode {
    Normal,
    Editing,
    Creating,
}

pub struct App {
    pub items: Vec<Application>,
    selected: usize,
    pub table_state: TableState,
    pub mode: AppMode,
    pub input: InputApplication,
    db_file_path: PathBuf,
}

impl App {
    pub fn new() -> eyre::Result<Self> {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let cli_file_param = parse_cli_file()?;
        let db_file_path = resolve_data_path(cli_file_param)?;
        let items = load_db::<Application>(&db_file_path)?;

        Ok(Self {
            items,
            selected: 0,
            table_state,
            mode: AppMode::Normal,
            input: InputApplication::default(),
            db_file_path,
        })
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.items.len();
        self.table_state.select(Some(self.selected));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }

        self.table_state.select(Some(self.selected));
    }

    pub fn delete(&mut self) {
        if self.items.is_empty() {
            return;
        }

        self.items.remove(self.selected);

        if self.selected >= self.items.len() && !self.items.is_empty() {
            self.selected = self.items.len() - 1;
        }

        self.table_state.select(Some(self.selected));
        self.save_db();
    }

    pub fn start_create(&mut self) {
        self.mode = AppMode::Creating;
        self.input.clear();
    }

    pub fn start_edit(&mut self) {
        if let Some(item) = self.items.get(self.selected) {
            self.mode = AppMode::Editing;
            self.input = InputApplication::from(item);
        }
    }

    pub fn confirm(&mut self) {
        match self.mode {
            AppMode::Creating => {
                self.items.push(Application::from(self.input.clone()));

                self.selected = self.items.len() - 1;
            }

            AppMode::Editing => {
                if let Some(item) = self.items.get_mut(self.selected) {
                    item.update(&self.input);
                }
            }

            _ => {}
        }

        self.mode = AppMode::Normal;
        self.table_state.select(Some(self.selected));
        self.save_db();
    }

    pub fn cancel(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn usage(&self) -> &'static str {
        "(a - add | e - edit | D - delete | Tab - switch | Enter - save | Esc - cancel | Q - quit)"
    }

    fn save_db(&self) {
        if let Err(e) = save_db::<Application>(&self.db_file_path, &self.items) {
            eprintln!("Save failed: {e}");
        } else {
            println!("Database saved");
        }
    }
}
