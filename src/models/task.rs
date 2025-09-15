use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub list_id: u32,
    pub due_date: Option<DateTime<Local>>,
    pub reminder_date: Option<DateTime<Local>>,
    pub recurring_frequency: Option<RecurringFrequency>,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Local>>,
    pub is_in_my_day: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurringFrequency {
    Daily,
    Weekdays,
    Weekly,
    Monthly,
    Yearly,
}

impl Task {
    pub fn new(id: u32, title: String, list_id: u32) -> Self {
        let now = Local::now();
        Self {
            id,
            title,
            description: None,
            list_id,
            due_date: None,
            reminder_date: None,
            recurring_frequency: None,
            is_completed: false,
            completed_at: None,
            is_in_my_day: false,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_due_today(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let today = Local::now().date_naive();
            let due_date_local = due_date.with_timezone(&Local).date_naive();
            due_date_local == today
        } else {
            false
        }
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let now = Local::now();
            due_date < now && !self.is_completed
        } else {
            false
        }
    }

    pub fn toggle_completion(&mut self) {
        self.is_completed = !self.is_completed;
        if self.is_completed {
            self.completed_at = Some(Local::now());
        } else {
            self.completed_at = None;
        }
        self.updated_at = Local::now();
    }

    pub fn add_to_my_day(&mut self) {
        self.is_in_my_day = true;
        self.updated_at = Local::now();
    }

    pub fn remove_from_my_day(&mut self) {
        self.is_in_my_day = false;
        self.updated_at = Local::now();
    }

    pub fn set_due_date(&mut self, due_date: Option<DateTime<Local>>) {
        self.due_date = due_date;
        self.updated_at = Local::now();
    }


    pub fn set_recurring_frequency(&mut self, frequency: Option<RecurringFrequency>) {
        self.recurring_frequency = frequency;
        self.updated_at = Local::now();
    }

    pub fn set_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
        self.updated_at = Local::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFilter {
    pub list_id: Option<u32>,
    pub show_completed: bool,
    pub show_my_day_only: bool,
    pub due_today_only: bool,
    pub overdue_only: bool,
}

impl Default for TaskFilter {
    fn default() -> Self {
        Self {
            list_id: None,
            show_completed: true,
            show_my_day_only: false,
            due_today_only: false,
            overdue_only: false,
        }
    }
}

