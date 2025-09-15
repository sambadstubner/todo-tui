use crate::models::{List, Task};
use anyhow::Result;
use chrono::{DateTime, Local};
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskRecord {
    id: u32,
    title: String,
    description: Option<String>,
    list_id: u32,
    due_date: Option<String>,
    reminder_date: Option<String>,
    recurring_frequency: Option<String>,
    is_completed: bool,
    completed_at: Option<String>,
    is_in_my_day: bool,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ListRecord {
    id: u32,
    name: String,
    color: Option<String>,
    created_at: String,
    updated_at: String,
}

pub struct Storage {
    tasks_file: String,
    lists_file: String,
    next_task_id: u32,
    next_list_id: u32,
}

impl Storage {
    pub fn new(data_dir: &str) -> Self {
        Self {
            tasks_file: format!("{}/tasks.csv", data_dir),
            lists_file: format!("{}/lists.csv", data_dir),
            next_task_id: 1,
            next_list_id: 1,
        }
    }

    pub fn load_all(&mut self) -> Result<(Vec<Task>, Vec<List>)> {
        // Ensure data directory exists
        if let Some(parent) = Path::new(&self.tasks_file).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let tasks = self.load_tasks()?;
        let lists = self.load_lists()?;

        // Update next IDs
        self.next_task_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.next_list_id = lists.iter().map(|l| l.id).max().unwrap_or(0) + 1;

        Ok((tasks, lists))
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        if !Path::new(&self.tasks_file).exists() {
            return Ok(vec![]);
        }

        let file = File::open(&self.tasks_file)?;
        let mut reader = Reader::from_reader(file);
        let mut tasks = Vec::new();

        for result in reader.deserialize() {
            let record: TaskRecord = result?;
            let task = self.task_from_record(record)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub fn load_lists(&self) -> Result<Vec<List>> {
        if !Path::new(&self.lists_file).exists() {
            return Ok(vec![]);
        }

        let file = File::open(&self.lists_file)?;
        let mut reader = Reader::from_reader(file);
        let mut lists = Vec::new();

        for result in reader.deserialize() {
            let record: ListRecord = result?;
            let list = self.list_from_record(record)?;
            lists.push(list);
        }

        Ok(lists)
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.tasks_file)?;

        let mut writer = Writer::from_writer(file);

        for task in tasks {
            let record = self.task_to_record(task);
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn save_lists(&self, lists: &[List]) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.lists_file)?;

        let mut writer = Writer::from_writer(file);

        for list in lists {
            let record = self.list_to_record(list);
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn get_next_task_id(&mut self) -> u32 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        id
    }

    pub fn get_next_list_id(&mut self) -> u32 {
        let id = self.next_list_id;
        self.next_list_id += 1;
        id
    }

    fn task_from_record(&self, record: TaskRecord) -> Result<Task> {
        let due_date = if let Some(date_str) = record.due_date {
            Some(DateTime::parse_from_rfc3339(&date_str)?.with_timezone(&Local))
        } else {
            None
        };

        let reminder_date = if let Some(date_str) = record.reminder_date {
            Some(DateTime::parse_from_rfc3339(&date_str)?.with_timezone(&Local))
        } else {
            None
        };

        let recurring_frequency = if let Some(freq_str) = record.recurring_frequency {
            Some(match freq_str.as_str() {
                "Daily" => crate::models::RecurringFrequency::Daily,
                "Weekdays" => crate::models::RecurringFrequency::Weekdays,
                "Weekly" => crate::models::RecurringFrequency::Weekly,
                "Monthly" => crate::models::RecurringFrequency::Monthly,
                "Yearly" => crate::models::RecurringFrequency::Yearly,
                _ => return Err(anyhow::anyhow!("Invalid recurring frequency: {}", freq_str)),
            })
        } else {
            None
        };

        let created_at = DateTime::parse_from_rfc3339(&record.created_at)?.with_timezone(&Local);
        let updated_at = DateTime::parse_from_rfc3339(&record.updated_at)?.with_timezone(&Local);
        let completed_at = if let Some(completed_at_str) = &record.completed_at {
            Some(DateTime::parse_from_rfc3339(completed_at_str)?.with_timezone(&Local))
        } else {
            None
        };

        Ok(Task {
            id: record.id,
            title: record.title,
            description: record.description,
            list_id: record.list_id,
            due_date,
            reminder_date,
            recurring_frequency,
            is_completed: record.is_completed,
            completed_at,
            is_in_my_day: record.is_in_my_day,
            notes: record.notes,
            created_at,
            updated_at,
        })
    }

    fn list_from_record(&self, record: ListRecord) -> Result<List> {
        let created_at = DateTime::parse_from_rfc3339(&record.created_at)?.with_timezone(&Local);
        let updated_at = DateTime::parse_from_rfc3339(&record.updated_at)?.with_timezone(&Local);

        Ok(List {
            id: record.id,
            name: record.name,
            color: record.color,
            created_at,
            updated_at,
        })
    }

    fn task_to_record(&self, task: &Task) -> TaskRecord {
        TaskRecord {
            id: task.id,
            title: task.title.clone(),
            description: task.description.clone(),
            list_id: task.list_id,
            due_date: task.due_date.map(|d| d.to_rfc3339()),
            reminder_date: task.reminder_date.map(|d| d.to_rfc3339()),
            recurring_frequency: task.recurring_frequency.as_ref().map(|f| match f {
                crate::models::RecurringFrequency::Daily => "Daily".to_string(),
                crate::models::RecurringFrequency::Weekdays => "Weekdays".to_string(),
                crate::models::RecurringFrequency::Weekly => "Weekly".to_string(),
                crate::models::RecurringFrequency::Monthly => "Monthly".to_string(),
                crate::models::RecurringFrequency::Yearly => "Yearly".to_string(),
            }),
            is_completed: task.is_completed,
            completed_at: task.completed_at.map(|d| d.to_rfc3339()),
            is_in_my_day: task.is_in_my_day,
            notes: task.notes.clone(),
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        }
    }

    fn list_to_record(&self, list: &List) -> ListRecord {
        ListRecord {
            id: list.id,
            name: list.name.clone(),
            color: list.color.clone(),
            created_at: list.created_at.to_rfc3339(),
            updated_at: list.updated_at.to_rfc3339(),
        }
    }
}
