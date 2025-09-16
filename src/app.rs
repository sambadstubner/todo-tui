use crate::models::{List, Storage, Task};
use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    ListOverview,
    TaskList,
    MyDay,
    TaskEditor,
    ListEditor,
    NoteEditor,
    MoveTask,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskEditorMode {
    Create,
    Edit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskEditorState {
    Title,
    DueDate,
    Frequency,
    Notes,
}

pub struct App {
    pub state: AppState,
    pub tasks: Vec<Task>,
    pub lists: Vec<List>,
    pub storage: Storage,
    pub current_list_id: Option<u32>,
    pub selected_task_index: usize,
    pub task_editor_mode: TaskEditorMode,
    pub task_editor_state: TaskEditorState,
    pub editing_task: Option<Task>,
    pub input_buffer: String,
    pub should_quit: bool,
    pub moving_task: Option<Task>,
    pub my_day_task_order: Vec<u32>, // Task IDs in display order for My Day
}

impl App {
    pub fn new() -> Result<Self> {
        // Get the home directory and create todo-data folder
        let home_dir = dirs::home_dir()
            .context("Could not find home directory")?;
        let data_dir = home_dir.join("todo-data");
        
        // Create the data directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)
            .context("Failed to create todo-data directory")?;
        
        let data_dir_str = data_dir.to_string_lossy().to_string();
        let mut storage = Storage::new(&data_dir_str);
        let (tasks, lists) = storage.load_all()?;

        // Ensure we have at least one default list
        let mut lists = lists;
        if lists.is_empty() {
            let default_list = List::new(storage.get_next_list_id(), "My Tasks".to_string());
            lists.push(default_list.clone());
            storage.save_lists(&lists)?;
        }

        // Initialize My Day task order
        let my_day_task_order: Vec<u32> = tasks
            .iter()
            .filter(|task| task.is_in_my_day || task.is_due_today())
            .map(|task| task.id)
            .collect();

        Ok(Self {
            state: AppState::ListOverview,
            tasks,
            lists,
            storage,
            current_list_id: None,
            selected_task_index: 0,
            task_editor_mode: TaskEditorMode::Create,
            task_editor_state: TaskEditorState::Title,
            editing_task: None,
            input_buffer: String::new(),
            should_quit: false,
            moving_task: None,
            my_day_task_order,
        })
    }


    pub fn get_current_list_tasks(&self) -> Vec<&Task> {
        if let Some(list_id) = self.current_list_id {
            self.tasks
                .iter()
                .filter(|task| task.list_id == list_id)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_my_day_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks
            .iter()
            .filter(|task| task.is_in_my_day || task.is_due_today())
            .collect();
        
        // Sort by the order in my_day_task_order
        tasks.sort_by_key(|task| {
            self.my_day_task_order.iter().position(|&id| id == task.id)
                .unwrap_or(usize::MAX)
        });
        
        tasks
    }

    /// Get the displayable task count that matches the UI structure
    /// This includes active tasks + separator (if both active and completed exist) + completed tasks
    pub fn get_displayable_task_count(&self, tasks: &[&Task]) -> usize {
        let active_tasks: Vec<_> = tasks.iter().filter(|t| !t.is_completed).collect();
        let completed_tasks: Vec<_> = tasks.iter().filter(|t| t.is_completed).collect();
        
        let mut count = active_tasks.len();
        if !active_tasks.is_empty() && !completed_tasks.is_empty() {
            count += 1; // separator
        }
        count += completed_tasks.len();
        count
    }

    /// Get the task at a specific display index (accounting for UI structure)
    pub fn get_task_at_display_index<'a>(&self, tasks: &[&'a Task], display_index: usize) -> Option<&'a Task> {
        let active_tasks: Vec<_> = tasks.iter().filter(|t| !t.is_completed).collect();
        let completed_tasks: Vec<_> = tasks.iter().filter(|t| t.is_completed).collect();
        
        if display_index < active_tasks.len() {
            // Index is in active tasks
            active_tasks.get(display_index).map(|t| **t)
        } else if !active_tasks.is_empty() && !completed_tasks.is_empty() && display_index == active_tasks.len() {
            // Index is the separator - return None
            None
        } else {
            // Index is in completed tasks
            let completed_index = if !active_tasks.is_empty() && !completed_tasks.is_empty() {
                display_index - active_tasks.len() - 1
            } else {
                display_index - active_tasks.len()
            };
            completed_tasks.get(completed_index).map(|t| **t)
        }
    }

    pub fn get_current_list(&self) -> Option<&List> {
        if let Some(list_id) = self.current_list_id {
            self.lists.iter().find(|list| list.id == list_id)
        } else {
            None
        }
    }

    pub fn add_task(&mut self, mut task: Task) -> Result<()> {
        task.id = self.storage.get_next_task_id();
        self.tasks.push(task);
        self.save_tasks()?;
        Ok(())
    }

    pub fn update_task(&mut self, updated_task: Task) -> Result<()> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == updated_task.id) {
            *task = updated_task;
            self.save_tasks()?;
        }
        Ok(())
    }

    pub fn delete_task(&mut self, task_id: u32) -> Result<()> {
        self.tasks.retain(|task| task.id != task_id);
        // Remove from My Day order if present
        self.my_day_task_order.retain(|&id| id != task_id);
        self.save_tasks()?;
        Ok(())
    }

    pub fn toggle_task_completion(&mut self, task_id: u32) -> Result<()> {
        // First, find the task and collect the data we need
        let (was_completed, has_recurring, task_data) = if let Some(task) = self.tasks.iter().find(|t| t.id == task_id) {
            let was_completed = task.is_completed;
            let has_recurring = task.recurring_frequency.is_some();
            let task_data = if has_recurring {
                Some((task.title.clone(), task.list_id, task.due_date, task.recurring_frequency.clone(), task.notes.clone()))
            } else {
                None
            };
            (was_completed, has_recurring, task_data)
        } else {
            return Ok(());
        };
        
        // Now toggle the task completion
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.toggle_completion();
        }
        
        // If task was just completed and has recurring frequency, create a new instance
        if !was_completed && has_recurring && task_data.is_some() {
            let (title, list_id, due_date, frequency, notes) = task_data.unwrap();
            self.create_recurring_task_instance(title, list_id, due_date, frequency, notes)?;
        }
        
        self.save_tasks()?;
        Ok(())
    }

    fn create_recurring_task_instance(
        &mut self, 
        title: String, 
        list_id: u32, 
        current_due_date: Option<chrono::DateTime<chrono::Local>>, 
        frequency: Option<crate::models::RecurringFrequency>, 
        notes: Option<String>
    ) -> Result<()> {
        use chrono::{Datelike, Duration, Local, Weekday};
        
        let frequency = frequency.unwrap();
        let current_due_date = current_due_date.unwrap_or_else(|| Local::now());
        let mut next_due_date = current_due_date;
        
        // Calculate the next due date based on frequency
        match frequency {
            crate::models::RecurringFrequency::Daily => {
                next_due_date = current_due_date + Duration::days(1);
            }
            crate::models::RecurringFrequency::Weekdays => {
                // Find the next weekday (Monday-Friday)
                loop {
                    next_due_date = next_due_date + Duration::days(1);
                    let weekday = next_due_date.weekday();
                    if weekday != Weekday::Sat && weekday != Weekday::Sun {
                        break;
                    }
                }
            }
            crate::models::RecurringFrequency::Weekly => {
                next_due_date = current_due_date + Duration::weeks(1);
            }
            crate::models::RecurringFrequency::Monthly => {
                // Add one month (approximate)
                next_due_date = current_due_date + Duration::days(30);
            }
            crate::models::RecurringFrequency::Yearly => {
                // Add one year (approximate)
                next_due_date = current_due_date + Duration::days(365);
            }
        }
        
        // Create new task instance
        let mut new_task = crate::models::Task::new(
            self.get_next_task_id(),
            title,
            list_id,
        );
        
        new_task.set_due_date(Some(next_due_date));
        new_task.set_recurring_frequency(Some(frequency));
        new_task.set_notes(notes);
        
        // Add the new task
        let new_task_id = new_task.id;
        self.tasks.push(new_task);
        
        // Add to My Day order if the new task is due today
        if next_due_date.date_naive() == chrono::Local::now().date_naive() {
            if !self.my_day_task_order.contains(&new_task_id) {
                self.my_day_task_order.push(new_task_id);
            }
        }
        
        Ok(())
    }

    fn get_next_task_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    /// Remove duplicate task IDs from my_day_task_order, keeping only the first occurrence
    fn deduplicate_my_day_order(&mut self) {
        let mut seen = std::collections::HashSet::new();
        self.my_day_task_order.retain(|&id| seen.insert(id));
    }

    pub fn move_task_up_in_my_day(&mut self, task_id: u32) -> Result<()> {
        // Clean up any duplicate IDs first
        self.deduplicate_my_day_order();
        
        if let Some(pos) = self.my_day_task_order.iter().position(|&id| id == task_id) {
            if pos > 0 {
                self.my_day_task_order.swap(pos, pos - 1);
            }
        }
        Ok(())
    }

    pub fn move_task_down_in_my_day(&mut self, task_id: u32) -> Result<()> {
        // Clean up any duplicate IDs first
        self.deduplicate_my_day_order();
        
        if let Some(pos) = self.my_day_task_order.iter().position(|&id| id == task_id) {
            if pos < self.my_day_task_order.len() - 1 {
                self.my_day_task_order.swap(pos, pos + 1);
            }
        }
        Ok(())
    }

    pub fn add_task_to_my_day(&mut self, task_id: u32) -> Result<()> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.add_to_my_day();
            // Add to My Day order if not already present
            if !self.my_day_task_order.contains(&task_id) {
                self.my_day_task_order.push(task_id);
            }
            self.save_tasks()?;
        }
        Ok(())
    }

    pub fn remove_task_from_my_day(&mut self, task_id: u32) -> Result<()> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.remove_from_my_day();
            // Remove from My Day order
            self.my_day_task_order.retain(|&id| id != task_id);
            self.save_tasks()?;
        }
        Ok(())
    }

    pub fn move_task_to_list(&mut self, task_id: u32, new_list_id: u32) -> Result<()> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.list_id = new_list_id;
            task.updated_at = chrono::Local::now();
            self.save_tasks()?;
        }
        Ok(())
    }

    pub fn add_list(&mut self, mut list: List) -> Result<()> {
        list.id = self.storage.get_next_list_id();
        self.lists.push(list);
        self.save_lists()?;
        Ok(())
    }


    pub fn set_current_list(&mut self, list_id: u32) {
        self.current_list_id = Some(list_id);
        self.selected_task_index = 0;
        self.state = AppState::TaskList;
    }


    fn save_tasks(&self) -> Result<()> {
        self.storage.save_tasks(&self.tasks)
    }

    fn save_lists(&self) -> Result<()> {
        self.storage.save_lists(&self.lists)
    }
}
