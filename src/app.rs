use std::path::PathBuf;

use chrono::{Datelike, Duration, Local, NaiveDateTime};
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{ListState, TableState};

use crate::{
    db::{load_db, parse_cli_file, resolve_data_path, save_db},
    models::{Application, ApplicationStatus, InputApplication},
};

pub enum AppMode {
    Normal,
    Editing,
    Creating,
    UpdateStatus,
    ViewingDetails,
    PickingInterviewDate,
}

pub struct App {
    items: Vec<Application>,
    selected: usize,
    table_state: TableState,
    mode: AppMode,
    input: InputApplication,
    db_file_path: PathBuf,
    application_list_state: ListState,
    selected_application_state: usize,
    highlight_symbol: String,
    interview_picker_date: NaiveDateTime,
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
            application_list_state: ListState::default(),
            selected_application_state: 0,
            highlight_symbol: "-> ".to_string(),
            interview_picker_date: Local::now().naive_local(),
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            AppMode::Normal => match key.code {
                KeyCode::Char('Q') => return true,
                KeyCode::Down => self.next(),
                KeyCode::Up => self.previous(),
                KeyCode::Char('a') => self.start_create(),
                KeyCode::Char('e') => self.start_edit(),
                KeyCode::Char('d') => self.open_details(),
                KeyCode::Char('D') => self.delete(),
                KeyCode::Char('s') => self.update_status(),
                KeyCode::Char('i') => self.open_interview_date_picker(),
                _ => {}
            },

            AppMode::Creating | AppMode::Editing => match key.code {
                KeyCode::Esc => self.cancel(),
                KeyCode::Enter => self.confirm(),
                KeyCode::Tab => self.next_input_field(),
                KeyCode::Backspace => self.backspace_input(),
                KeyCode::Char(c) => self.push_input_char(c),
                _ => {}
            },

            AppMode::UpdateStatus => match key.code {
                KeyCode::Down => self.next_status(),
                KeyCode::Up => self.previous_status(),
                KeyCode::Esc => self.cancel_status_update(),
                KeyCode::Enter => self.confirm_status_update(),
                _ => {}
            },

            AppMode::ViewingDetails => match key.code {
                KeyCode::Esc => self.close_details(),
                _ => {}
            },

            AppMode::PickingInterviewDate => match key.code {
                KeyCode::Esc => self.close_interview_date_picker(),
                KeyCode::Left => self.shift_interview_picker_days(-1),
                KeyCode::Right => self.shift_interview_picker_days(1),
                KeyCode::Up => self.shift_interview_picker_days(-7),
                KeyCode::Down => self.shift_interview_picker_days(7),
                KeyCode::PageUp => self.shift_interview_picker_months(-1),
                KeyCode::PageDown => self.shift_interview_picker_months(1),
                KeyCode::Char('+') => self.shift_interview_picker_minutes(60),
                KeyCode::Char('-') => self.shift_interview_picker_minutes(-60),
                KeyCode::Char(']') => self.shift_interview_picker_minutes(15),
                KeyCode::Char('[') => self.shift_interview_picker_minutes(-15),
                KeyCode::Enter => self.confirm_interview_date(),
                _ => {}
            },
        }

        false
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn items(&self) -> &[Application] {
        &self.items
    }

    pub fn input(&self) -> &InputApplication {
        &self.input
    }

    pub fn selected_item(&self) -> Option<&Application> {
        self.items.get(self.selected)
    }

    pub fn table_state_mut(&mut self) -> &mut TableState {
        &mut self.table_state
    }

    pub fn application_list_state_mut(&mut self) -> &mut ListState {
        &mut self.application_list_state
    }

    pub fn highlight_symbol(&self) -> &str {
        &self.highlight_symbol
    }

    pub fn interview_picker_date(&self) -> NaiveDateTime {
        self.interview_picker_date
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

    pub fn open_details(&mut self) {
        if !self.items.is_empty() {
            self.mode = AppMode::ViewingDetails;
        }
    }

    pub fn close_details(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn open_interview_date_picker(&mut self) {
        if let Some(item) = self.items.get(self.selected) {
            self.interview_picker_date = item
                .interview_date
                .unwrap_or_else(|| Local::now().naive_local());
            self.mode = AppMode::PickingInterviewDate;
        }
    }

    pub fn close_interview_date_picker(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn confirm_interview_date(&mut self) {
        if let Some(item) = self.items.get_mut(self.selected) {
            item.interview_date = Some(self.interview_picker_date);
            item.application_status = ApplicationStatus::InterviewScheduled;
            self.save_db();
        }

        self.mode = AppMode::Normal;
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
        self.input.clear();
        self.mode = AppMode::Normal;
    }

    pub fn usage(&self) -> &'static str {
        match self.mode {
            AppMode::Normal => {
                "Main: ↑/↓ move | a add | e edit | d details | i interview | s status | D delete | Q quit"
            }
            AppMode::Creating => {
                "Create: Tab next field | type to edit | Backspace delete | Enter save | Esc cancel"
            }
            AppMode::Editing => {
                "Edit: Tab next field | type to edit | Backspace delete | Enter save | Esc cancel"
            }
            AppMode::UpdateStatus => "Status: ↑/↓ choose status | Enter save | Esc cancel",
            AppMode::ViewingDetails => "Details: Esc close",
            AppMode::PickingInterviewDate => {
                "Interview: ←/→ day | ↑/↓ week | PgUp/PgDn month | +/- hour | [/ ] minute | Enter save | Esc cancel"
            }
        }
    }

    pub fn update_status(&mut self) {
        self.mode = AppMode::UpdateStatus;
        self.selected_application_state = 0;
        self.application_list_state.select(Some(0));
    }

    pub fn cancel_status_update(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn next_status(&mut self) {
        self.selected_application_state =
            (self.selected_application_state + 1) % ApplicationStatus::ALL.len();
        self.application_list_state
            .select(Some(self.selected_application_state));
    }

    pub fn previous_status(&mut self) {
        if self.selected_application_state == 0 {
            self.selected_application_state = ApplicationStatus::ALL.len() - 1;
        } else {
            self.selected_application_state -= 1;
        }
        self.application_list_state
            .select(Some(self.selected_application_state));
    }

    pub fn confirm_status_update(&mut self) {
        if let AppMode::UpdateStatus = self.mode
            && let Some(new_status) = ApplicationStatus::ALL.get(self.selected_application_state)
            && let Some(item) = self.items.get_mut(self.selected)
        {
            item.application_status = *new_status;
        }

        self.mode = AppMode::Normal;
        self.save_db();
    }

    fn shift_interview_picker_days(&mut self, days: i64) {
        self.interview_picker_date = self
            .interview_picker_date
            .checked_add_signed(Duration::days(days))
            .unwrap_or(self.interview_picker_date);
    }

    fn shift_interview_picker_minutes(&mut self, minutes: i64) {
        self.interview_picker_date = self
            .interview_picker_date
            .checked_add_signed(Duration::minutes(minutes))
            .unwrap_or(self.interview_picker_date);
    }

    fn shift_interview_picker_months(&mut self, months: i32) {
        let date = self.interview_picker_date.date();
        let time = self.interview_picker_date.time();

        let mut year = date.year();
        let mut month = date.month() as i32 + months;

        while month < 1 {
            year -= 1;
            month += 12;
        }

        while month > 12 {
            year += 1;
            month -= 12;
        }

        let month_u32 = month as u32;
        let mut day = date.day();

        while chrono::NaiveDate::from_ymd_opt(year, month_u32, day).is_none() && day > 1 {
            day -= 1;
        }

        if let Some(new_date) = chrono::NaiveDate::from_ymd_opt(year, month_u32, day) {
            self.interview_picker_date = NaiveDateTime::new(new_date, time);
        }
    }

    fn next_input_field(&mut self) {
        self.input.input_field = (self.input.input_field + 1) % 5;
    }

    fn backspace_input(&mut self) {
        match self.input.input_field {
            0 => {
                self.input.company_name.pop();
            }
            1 => {
                self.input.description.pop();
            }
            2 => {
                self.input.origin.pop();
            }
            3 => {
                self.input.url.pop();
            }
            4 => {
                self.input.comments.pop();
            }
            _ => {}
        }
    }

    fn push_input_char(&mut self, c: char) {
        match self.input.input_field {
            0 => self.input.company_name.push(c),
            1 => self.input.description.push(c),
            2 => self.input.origin.push(c),
            3 => self.input.url.push(c),
            4 => self.input.comments.push(c),
            _ => {}
        }
    }

    fn save_db(&self) {
        if let Err(e) = save_db::<Application>(&self.db_file_path, &self.items) {
            eprintln!("Save failed: {e}");
        }
    }
}
