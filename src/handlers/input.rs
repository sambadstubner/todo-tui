use crate::app::{App, AppState, TaskEditorMode, TaskEditorState};
use crate::models::Task;
use crate::utils::date_utils;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.state {
        AppState::ListOverview => handle_list_overview_input(app, key),
        AppState::TaskList => handle_task_list_input(app, key),
        AppState::MyDay => handle_my_day_input(app, key),
        AppState::TaskEditor => handle_task_editor_input(app, key),
        AppState::ListEditor => handle_list_editor_input(app, key),
        AppState::NoteEditor => handle_note_editor_input(app, key),
        AppState::MoveTask => handle_move_task_input(app, key),
    }
}

fn handle_list_overview_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Create new list
            app.input_buffer.clear();
            app.state = AppState::ListEditor;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_task_index > 0 {
                app.selected_task_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_task_index < app.lists.len().saturating_sub(1) {
                app.selected_task_index += 1;
            }
        }
        KeyCode::Char('g') => {
            // Handle 'gg' for jumping to top
            if app.input_buffer == "g" {
                app.selected_task_index = 0;
                app.input_buffer.clear();
            } else {
                app.input_buffer = "g".to_string();
            }
        }
        KeyCode::Char('G') => {
            // Jump to bottom
            app.selected_task_index = app.lists.len().saturating_sub(1);
        }
        KeyCode::Enter => {
            if let Some(list) = app.lists.get(app.selected_task_index) {
                app.set_current_list(list.id);
            }
        }
        KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.state = AppState::MyDay;
            app.selected_task_index = 0;
        }
        _ => {}
    }
    Ok(())
}

fn handle_task_list_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.state = AppState::ListOverview;
            app.current_list_id = None;
            app.selected_task_index = 0;
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Create new task
            if let Some(list_id) = app.current_list_id {
                let new_task = Task::new(0, "".to_string(), list_id);
                app.editing_task = Some(new_task);
                app.input_buffer.clear();
                app.state = AppState::TaskEditor;
                app.task_editor_mode = TaskEditorMode::Create;
                app.task_editor_state = TaskEditorState::Title;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_task_index > 0 {
                app.selected_task_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let tasks = app.get_current_list_tasks();
            let displayable_count = app.get_displayable_task_count(&tasks);
            if app.selected_task_index < displayable_count.saturating_sub(1) {
                app.selected_task_index += 1;
            }
        }
        KeyCode::Char('g') => {
            // Handle 'gg' for jumping to top
            if app.input_buffer == "g" {
                app.selected_task_index = 0;
                app.input_buffer.clear();
            } else {
                app.input_buffer = "g".to_string();
            }
        }
        KeyCode::Char('G') => {
            // Jump to bottom
            let tasks = app.get_current_list_tasks();
            let displayable_count = app.get_displayable_task_count(&tasks);
            app.selected_task_index = displayable_count.saturating_sub(1);
        }
        KeyCode::Enter => {
            let tasks = app.get_current_list_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_clone = task.clone();
                let title = task.title.clone();
                app.editing_task = Some(task_clone);
                app.input_buffer = title;
                app.state = AppState::TaskEditor;
                app.task_editor_mode = TaskEditorMode::Edit;
                app.task_editor_state = TaskEditorState::Title;
            }
        }
        KeyCode::Char(' ') => {
            let tasks = app.get_current_list_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_id = task.id;
                app.toggle_task_completion(task_id)?;
                
                // For single task lists, keep selection at 0
                let new_tasks = app.get_current_list_tasks();
                let displayable_count = app.get_displayable_task_count(&new_tasks);
                
                if displayable_count == 1 {
                    app.selected_task_index = 0;
                } else if displayable_count > 1 {
                    // Ensure selection stays within bounds for multi-task lists
                    if app.selected_task_index >= displayable_count {
                        app.selected_task_index = displayable_count.saturating_sub(1);
                    }
                } else {
                    // No tasks left
                    app.selected_task_index = 0;
                }
            }
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let tasks = app.get_current_list_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                if task.is_in_my_day {
                    app.remove_task_from_my_day(task.id)?;
                } else {
                    app.add_task_to_my_day(task.id)?;
                }
            }
        }
        KeyCode::Delete | KeyCode::Backspace => {
            let tasks = app.get_current_list_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_id = task.id;
                let displayable_count = app.get_displayable_task_count(&tasks);
                app.delete_task(task_id)?;
                if app.selected_task_index >= displayable_count.saturating_sub(1) {
                    app.selected_task_index = displayable_count.saturating_sub(2);
                }
            }
        }
        KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.state = AppState::MyDay;
            app.selected_task_index = 0;
        }
        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let tasks = app.get_current_list_tasks();
            if let Some(task) = tasks.get(app.selected_task_index) {
                let task_clone = (*task).clone();
                let title = task.title.clone();
                app.editing_task = Some(task_clone);
                app.input_buffer = title;
                app.state = AppState::TaskEditor;
                app.task_editor_mode = TaskEditorMode::Edit;
                app.task_editor_state = TaskEditorState::Title;
            }
        }
        KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // View notes
            let tasks = app.get_current_list_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_clone = task.clone();
                let notes = task.notes.clone().unwrap_or_default();
                app.editing_task = Some(task_clone);
                app.input_buffer = notes;
                app.state = AppState::NoteEditor;
            }
        }
        KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Move task to another list
            let tasks = app.get_current_list_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                app.moving_task = Some(task.clone());
                app.state = AppState::MoveTask;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_my_day_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.state = AppState::ListOverview;
            app.selected_task_index = 0;
        }
        KeyCode::Up if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.selected_task_index > 0 {
                app.selected_task_index -= 1;
            }
        }
        KeyCode::Char('k') => {
            if app.selected_task_index > 0 {
                app.selected_task_index -= 1;
            }
        }
        KeyCode::Down if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            let tasks = app.get_my_day_tasks();
            let displayable_count = app.get_displayable_task_count(&tasks);
            // Ensure selection doesn't go out of bounds
            if displayable_count > 0 && app.selected_task_index < displayable_count.saturating_sub(1) {
                app.selected_task_index += 1;
            }
        }
        KeyCode::Char('j') => {
            let tasks = app.get_my_day_tasks();
            let displayable_count = app.get_displayable_task_count(&tasks);
            // Ensure selection doesn't go out of bounds
            if displayable_count > 0 && app.selected_task_index < displayable_count.saturating_sub(1) {
                app.selected_task_index += 1;
            }
        }
        KeyCode::Char('g') => {
            // Handle 'gg' for jumping to top
            if app.input_buffer == "g" {
                app.selected_task_index = 0;
                app.input_buffer.clear();
            } else {
                app.input_buffer = "g".to_string();
            }
        }
        KeyCode::Char('G') => {
            // Jump to bottom
            let tasks = app.get_my_day_tasks();
            let displayable_count = app.get_displayable_task_count(&tasks);
            app.selected_task_index = displayable_count.saturating_sub(1);
        }
        KeyCode::Char(' ') => {
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_id = task.id;
                app.toggle_task_completion(task_id)?;
                
                // For single task lists, keep selection at 0
                let new_tasks = app.get_my_day_tasks();
                let displayable_count = app.get_displayable_task_count(&new_tasks);
                
                if displayable_count == 1 {
                    app.selected_task_index = 0;
                } else if displayable_count > 1 {
                    // Ensure selection stays within bounds for multi-task lists
                    if app.selected_task_index >= displayable_count {
                        app.selected_task_index = displayable_count.saturating_sub(1);
                    }
                } else {
                    // No tasks left
                    app.selected_task_index = 0;
                }
            }
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                app.remove_task_from_my_day(task.id)?;
            }
        }
        KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // View notes
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_clone = task.clone();
                let notes = task.notes.clone().unwrap_or_default();
                app.editing_task = Some(task_clone);
                app.input_buffer = notes;
                app.state = AppState::NoteEditor;
            }
        }
        KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Move task to another list
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                app.moving_task = Some(task.clone());
                app.state = AppState::MoveTask;
            }
        }
        KeyCode::Delete | KeyCode::Backspace => {
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_id = task.id;
                let displayable_count = app.get_displayable_task_count(&tasks);
                app.delete_task(task_id)?;
                if app.selected_task_index >= displayable_count.saturating_sub(1) {
                    app.selected_task_index = displayable_count.saturating_sub(2);
                }
            }
        }
        KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Move selected task up in My Day order
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_id = task.id;
                app.move_task_up_in_my_day(task_id)?;
                
                // Update selection to follow the moved task
                let new_tasks = app.get_my_day_tasks();
                // Find the display index of the moved task
                for (display_index, _) in (0..app.get_displayable_task_count(&new_tasks)).enumerate() {
                    if let Some(task_at_index) = app.get_task_at_display_index(&new_tasks, display_index) {
                        if task_at_index.id == task_id {
                            app.selected_task_index = display_index;
                            break;
                        }
                    }
                }
            }
        }
        KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Move selected task down in My Day order
            let tasks = app.get_my_day_tasks();
            if let Some(task) = app.get_task_at_display_index(&tasks, app.selected_task_index) {
                let task_id = task.id;
                app.move_task_down_in_my_day(task_id)?;
                
                // Update selection to follow the moved task
                let new_tasks = app.get_my_day_tasks();
                // Find the display index of the moved task
                for (display_index, _) in (0..app.get_displayable_task_count(&new_tasks)).enumerate() {
                    if let Some(task_at_index) = app.get_task_at_display_index(&new_tasks, display_index) {
                        if task_at_index.id == task_id {
                            app.selected_task_index = display_index;
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_task_editor_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.state = if app.current_list_id.is_some() {
                AppState::TaskList
            } else {
                AppState::ListOverview
            };
            app.editing_task = None;
            app.input_buffer.clear();
            app.task_editor_state = TaskEditorState::Title;
        }
        KeyCode::Enter => {
            if let Some(task) = app.editing_task.as_mut() {
                match app.task_editor_state {
                    TaskEditorState::Title => {
                        if !app.input_buffer.trim().is_empty() {
                            task.title = app.input_buffer.trim().to_string();
                            app.input_buffer.clear();
                            app.task_editor_state = TaskEditorState::DueDate;
                        } else {
                            // For new tasks, we need a title, so don't proceed
                            // For editing existing tasks, we can proceed with empty title
                            if app.task_editor_mode == TaskEditorMode::Edit {
                                app.input_buffer.clear();
                                app.task_editor_state = TaskEditorState::DueDate;
                            }
                        }
                    }
                    TaskEditorState::DueDate => {
                        if !app.input_buffer.trim().is_empty() {
                            if let Some(due_date) = date_utils::parse_date_input(&app.input_buffer) {
                                task.set_due_date(Some(due_date));
                            }
                        }
                        app.input_buffer.clear();
                        app.task_editor_state = TaskEditorState::Frequency;
                    }
                    TaskEditorState::Frequency => {
                        if !app.input_buffer.trim().is_empty() {
                            let freq_input = app.input_buffer.trim().to_lowercase();
                            let frequency = match freq_input.as_str() {
                                "daily" => Some(crate::models::RecurringFrequency::Daily),
                                "weekdays" => Some(crate::models::RecurringFrequency::Weekdays),
                                "weekly" => Some(crate::models::RecurringFrequency::Weekly),
                                "monthly" => Some(crate::models::RecurringFrequency::Monthly),
                                "yearly" => Some(crate::models::RecurringFrequency::Yearly),
                                "none" | "" => None,
                                _ => None,
                            };
                            task.set_recurring_frequency(frequency);
                        }
                        app.input_buffer.clear();
                        app.task_editor_state = TaskEditorState::Notes;
                    }
                    TaskEditorState::Notes => {
                        if !app.input_buffer.trim().is_empty() {
                            task.set_notes(Some(app.input_buffer.trim().to_string()));
                        }
                        
                        // Extract the task and save it
                        let task_to_save = task.clone();
                        match app.task_editor_mode {
                            TaskEditorMode::Create => {
                                app.add_task(task_to_save)?;
                            }
                            TaskEditorMode::Edit => {
                                app.update_task(task_to_save)?;
                            }
                        }
                        
                        // Return to task list
                        app.state = if app.current_list_id.is_some() {
                            AppState::TaskList
                        } else {
                            AppState::ListOverview
                        };
                        app.editing_task = None;
                        app.input_buffer.clear();
                        app.task_editor_state = TaskEditorState::Title;
                    }
                }
            }
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }
        _ => {}
    }
    Ok(())
}

fn handle_note_editor_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.state = if app.current_list_id.is_some() {
                AppState::TaskList
            } else {
                AppState::ListOverview
            };
            app.input_buffer.clear();
        }
        KeyCode::Enter => {
            if let Some(mut task) = app.editing_task.take() {
                task.set_notes(Some(app.input_buffer.clone()));
                app.update_task(task)?;
            }
            
            app.state = if app.current_list_id.is_some() {
                AppState::TaskList
            } else {
                AppState::ListOverview
            };
            app.input_buffer.clear();
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }
        _ => {}
    }
    Ok(())
}

fn handle_list_editor_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.state = AppState::ListOverview;
            app.input_buffer.clear();
        }
        KeyCode::Enter => {
            if !app.input_buffer.trim().is_empty() {
                let new_list = crate::models::List::new(0, app.input_buffer.trim().to_string());
                app.add_list(new_list)?;
                app.state = AppState::ListOverview;
                app.input_buffer.clear();
            }
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        }
        _ => {}
    }
    Ok(())
}

fn handle_move_task_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.moving_task = None;
            app.state = AppState::TaskList;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_task_index > 0 {
                app.selected_task_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_task_index < app.lists.len().saturating_sub(1) {
                app.selected_task_index += 1;
            }
        }
        KeyCode::Char('g') => {
            // Handle 'gg' for jumping to top
            if app.input_buffer == "g" {
                app.selected_task_index = 0;
                app.input_buffer.clear();
            } else {
                app.input_buffer = "g".to_string();
            }
        }
        KeyCode::Char('G') => {
            // Jump to bottom
            app.selected_task_index = app.lists.len().saturating_sub(1);
        }
        KeyCode::Enter => {
            if let Some(list) = app.lists.get(app.selected_task_index) {
                if let Some(task) = app.moving_task.take() {
                    app.move_task_to_list(task.id, list.id)?;
                    app.state = AppState::TaskList;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

