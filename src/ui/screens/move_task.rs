use crate::app::App;
use crate::theme::BlulocoTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Lists
        ])
        .split(area);

    draw_title(f, app, chunks[0]);
    draw_lists(f, app, chunks[1]);
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let task_name = if let Some(task) = &app.moving_task {
        format!("Move '{}' to:", task.title)
    } else {
        "Move Task".to_string()
    };

    let title_widget = Paragraph::new(task_name)
        .style(Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(title_widget, area);
}

fn draw_lists(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .lists
        .iter()
        .enumerate()
        .map(|(i, list)| {
            let style = if i == app.selected_task_index {
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
            .title("Select Destination List"));

    f.render_widget(list, area);
}
