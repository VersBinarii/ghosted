use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ApplicationStatus {
    Applied,
    Rejected,
    Accepted,
    AwaitingRecruiter,
    Ghosted,
    ThinkingAboutIt,
}

impl ApplicationStatus {
    pub const ALL: [ApplicationStatus; 6] = [
        Self::Applied,
        Self::Rejected,
        Self::Accepted,
        Self::AwaitingRecruiter,
        Self::Ghosted,
        Self::ThinkingAboutIt,
    ];
}

impl Display for ApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Applied => write!(f, "Applied"),
            Self::Rejected => write!(f, "Rejected"),
            Self::AwaitingRecruiter => write!(f, "Awaiting recruiter"),
            Self::Accepted => write!(f, "Accepted"),
            Self::Ghosted => write!(f, "Ghosted"),
            Self::ThinkingAboutIt => write!(f, "Thinking about it"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub company_name: String,
    pub description: String,
    pub url: String,
    pub application_status: ApplicationStatus,
    pub origin: String,
    pub application_date: DateTime<Utc>,
}

impl Application {
    pub fn update(&mut self, v: &InputApplication) {
        self.company_name = v.company_name.clone();
        self.description = v.description.clone();
        self.origin = v.origin.clone();
        self.url = v.url.clone();
    }
}

impl From<InputApplication> for Application {
    fn from(v: InputApplication) -> Self {
        Self {
            company_name: v.company_name,
            description: v.description,
            origin: v.origin,
            url: v.url,
            application_status: ApplicationStatus::Applied,
            application_date: Utc::now(),
        }
    }
}

#[derive(Clone, Default)]
pub struct InputApplication {
    pub company_name: String,
    pub description: String,
    pub url: String,
    pub origin: String,
    pub input_field: usize,
}

impl InputApplication {
    pub fn clear(&mut self) {
        self.company_name.clear();
        self.description.clear();
        self.url.clear();
        self.origin.clear();
        self.input_field = 0;
    }
}

impl From<&Application> for InputApplication {
    fn from(value: &Application) -> Self {
        Self {
            company_name: value.company_name.clone(),
            description: value.description.clone(),
            url: value.url.clone(),
            origin: value.origin.clone(),
            input_field: 0,
        }
    }
}
