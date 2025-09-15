use crate::app::App;
use crate::theme::BlulocoTheme;
use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_lists(f, app, chunks[0]);
    draw_my_day_tasks(f, app, chunks[1]);
}

fn draw_lists(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .lists
        .iter()
        .map(|list| {
            let style = if Some(list.id) == app.current_list_id {
                Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(BlulocoTheme::TEXT_PRIMARY)
            };

            let task_count = app.tasks.iter().filter(|t| t.list_id == list.id).count();
            let completed_count = app.tasks.iter().filter(|t| t.list_id == list.id && t.is_completed).count();
            
            let content = if task_count > 0 {
                format!("{} ({}/{})", list.name, completed_count, task_count)
            } else {
                list.name.clone()
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Lists"));

    f.render_widget(list, area);
}

fn draw_my_day_tasks(f: &mut Frame, app: &App, area: Rect) {
    let tasks = app.get_my_day_tasks();
    
    // Separate active and completed tasks
    let active_tasks: Vec<_> = tasks.iter().filter(|t| !t.is_completed).collect();
    let completed_tasks: Vec<_> = tasks.iter().filter(|t| t.is_completed).collect();
    
    let mut items: Vec<ListItem> = Vec::new();
    let mut current_index = 0;
    
    // Add active tasks
    for (_i, task) in active_tasks.iter().enumerate() {
        let style = if task.is_overdue() {
            if current_index == app.selected_task_index {
                Style::default().fg(BlulocoTheme::ERROR).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(BlulocoTheme::ERROR)
            }
        } else if task.is_due_today() {
            if current_index == app.selected_task_index {
                Style::default().fg(BlulocoTheme::WARNING).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(BlulocoTheme::WARNING)
            }
        } else if current_index == app.selected_task_index {
            Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(BlulocoTheme::TEXT_PRIMARY)
        };

        let checkbox = "☐";
        
        let due_info = if let Some(due_date) = task.due_date {
            let now = Local::now();
            let days_until = (due_date.date_naive() - now.date_naive()).num_days();
            
            if days_until < 0 {
                format!(" [Overdue {} days]", -days_until)
            } else if days_until == 0 {
                " [Today]".to_string()
            } else if days_until == 1 {
                " [Tomorrow]".to_string()
            } else {
                format!(" [{} days]", days_until)
            }
        } else {
            String::new()
        };

        let frequency_info = if let Some(freq) = &task.recurring_frequency {
            match freq {
                crate::models::RecurringFrequency::Daily => " [Daily]".to_string(),
                crate::models::RecurringFrequency::Weekdays => " [Weekdays]".to_string(),
                crate::models::RecurringFrequency::Weekly => " [Weekly]".to_string(),
                crate::models::RecurringFrequency::Monthly => " [Monthly]".to_string(),
                crate::models::RecurringFrequency::Yearly => " [Yearly]".to_string(),
            }
        } else {
            String::new()
        };

        // Show which list the task belongs to
        let list_name = app.lists
            .iter()
            .find(|l| l.id == task.list_id)
            .map(|l| l.name.as_str())
            .unwrap_or("Unknown");

        let content = format!("{} {} ({}){}{}", checkbox, task.title, list_name, due_info, frequency_info);
        items.push(ListItem::new(Line::from(Span::styled(content, style))));
        current_index += 1;
    }
    
    // Add separator if there are both active and completed tasks
    if !active_tasks.is_empty() && !completed_tasks.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled("─────────────── Completed ───────────────", Style::default().fg(BlulocoTheme::TEXT_MUTED)))));
        current_index += 1; // Increment index for the separator
    }
    
    // Add completed tasks
    for (_i, task) in completed_tasks.iter().enumerate() {
        let style = if current_index == app.selected_task_index {
            Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(BlulocoTheme::TEXT_MUTED)
        };

        let checkbox = "☑";
        
        let due_info = if let Some(due_date) = task.due_date {
            let now = Local::now();
            let days_until = (due_date.date_naive() - now.date_naive()).num_days();
            
            if days_until < 0 {
                format!(" [Overdue {} days]", -days_until)
            } else if days_until == 0 {
                " [Today]".to_string()
            } else if days_until == 1 {
                " [Tomorrow]".to_string()
            } else {
                format!(" [{} days]", days_until)
            }
        } else {
            String::new()
        };

        let frequency_info = if let Some(freq) = &task.recurring_frequency {
            match freq {
                crate::models::RecurringFrequency::Daily => " [Daily]".to_string(),
                crate::models::RecurringFrequency::Weekdays => " [Weekdays]".to_string(),
                crate::models::RecurringFrequency::Weekly => " [Weekly]".to_string(),
                crate::models::RecurringFrequency::Monthly => " [Monthly]".to_string(),
                crate::models::RecurringFrequency::Yearly => " [Yearly]".to_string(),
            }
        } else {
            String::new()
        };

        // Show which list the task belongs to
        let list_name = app.lists
            .iter()
            .find(|l| l.id == task.list_id)
            .map(|l| l.name.as_str())
            .unwrap_or("Unknown");

        let completion_info = if let Some(completed_at) = task.completed_at {
            format!(" [Completed: {}]", completed_at.format("%Y-%m-%d"))
        } else {
            String::new()
        };

        let content = format!("{} {} ({}){}{}{}", checkbox, task.title, list_name, due_info, frequency_info, completion_info);
        items.push(ListItem::new(Line::from(Span::styled(content, style))));
        current_index += 1;
    }

    let today = Local::now().format("%A, %B %d, %Y");
    let list_title = format!("My Day - {} ({} active, {} completed)", today, active_tasks.len(), completed_tasks.len());

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title(list_title));

    f.render_widget(list, area);
}
