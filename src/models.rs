use std::fmt::Display;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ApplicationStatus {
    Applied,
    Rejected,
    Accepted,
    AwaitingRecruiter,
    Ghosted,
    ThinkingAboutIt,
    InterviewScheduled,
}

impl ApplicationStatus {
    pub const ALL: [ApplicationStatus; 7] = [
        Self::Applied,
        Self::Rejected,
        Self::Accepted,
        Self::AwaitingRecruiter,
        Self::Ghosted,
        Self::ThinkingAboutIt,
        Self::InterviewScheduled,
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
            Self::InterviewScheduled => write!(f, "Interview scheduled"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub company_name: String,
    pub description: String,
    pub url: String,
    pub comments: String,
    pub application_status: ApplicationStatus,
    pub origin: String,
    pub application_date: DateTime<Utc>,
    pub interview_date: Option<NaiveDateTime>,
}

impl Application {
    pub fn update(&mut self, v: &InputApplication) {
        self.company_name = v.company_name.clone();
        self.description = v.description.clone();
        self.origin = v.origin.clone();
        self.url = v.url.clone();
        self.comments = v.comments.clone();
        self.interview_date = v.interview_date;
    }
}

impl From<InputApplication> for Application {
    fn from(v: InputApplication) -> Self {
        Self {
            company_name: v.company_name,
            description: v.description,
            origin: v.origin,
            url: v.url,
            comments: v.comments,
            application_status: ApplicationStatus::Applied,
            application_date: Utc::now(),
            interview_date: v.interview_date,
        }
    }
}

#[derive(Clone, Default)]
pub struct InputApplication {
    pub company_name: String,
    pub description: String,
    pub url: String,
    pub origin: String,
    pub comments: String,
    pub interview_date: Option<NaiveDateTime>,
    pub input_field: usize,
}

impl InputApplication {
    pub fn clear(&mut self) {
        self.company_name.clear();
        self.description.clear();
        self.url.clear();
        self.origin.clear();
        self.comments.clear();
        self.interview_date = None;
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
            comments: value.comments.clone(),
            interview_date: value.interview_date,
            input_field: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    fn sample_input() -> InputApplication {
        InputApplication {
            company_name: "Acme".to_string(),
            description: "Rust role".to_string(),
            url: "https://example.com".to_string(),
            origin: "Referral".to_string(),
            comments: "Interesting team".to_string(),
            interview_date: Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 4, 10).expect("valid date"),
                NaiveTime::from_hms_opt(11, 0, 0).expect("valid time"),
            )),
            input_field: 3,
        }
    }

    #[test]
    fn application_update_replaces_editable_fields() {
        let input = sample_input();
        let mut app = Application {
            company_name: "Old".to_string(),
            description: "Old".to_string(),
            url: "old".to_string(),
            comments: "Old".to_string(),
            application_status: ApplicationStatus::Ghosted,
            origin: "Old".to_string(),
            application_date: Utc::now(),
            interview_date: None,
        };

        app.update(&input);

        assert_eq!(app.company_name, input.company_name);
        assert_eq!(app.description, input.description);
        assert_eq!(app.url, input.url);
        assert_eq!(app.comments, input.comments);
        assert_eq!(app.origin, input.origin);
        assert_eq!(app.interview_date, input.interview_date);
        assert!(matches!(app.application_status, ApplicationStatus::Ghosted));
    }

    #[test]
    fn input_clear_resets_all_fields() {
        let mut input = sample_input();

        input.clear();

        assert!(input.company_name.is_empty());
        assert!(input.description.is_empty());
        assert!(input.url.is_empty());
        assert!(input.origin.is_empty());
        assert!(input.comments.is_empty());
        assert_eq!(input.interview_date, None);
        assert_eq!(input.input_field, 0);
    }

    #[test]
    fn input_from_application_copies_values_and_resets_selected_field() {
        let app = Application {
            company_name: "Acme".to_string(),
            description: "Rust role".to_string(),
            url: "https://example.com".to_string(),
            comments: "Interesting team".to_string(),
            application_status: ApplicationStatus::Applied,
            origin: "Referral".to_string(),
            application_date: Utc::now(),
            interview_date: Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 4, 10).expect("valid date"),
                NaiveTime::from_hms_opt(11, 0, 0).expect("valid time"),
            )),
        };

        let input = InputApplication::from(&app);

        assert_eq!(input.company_name, app.company_name);
        assert_eq!(input.description, app.description);
        assert_eq!(input.url, app.url);
        assert_eq!(input.comments, app.comments);
        assert_eq!(input.origin, app.origin);
        assert_eq!(input.interview_date, app.interview_date);
        assert_eq!(input.input_field, 0);
    }
}
