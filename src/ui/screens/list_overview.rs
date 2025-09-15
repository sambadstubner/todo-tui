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
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_lists(f, app, chunks[0]);
    draw_info(f, app, chunks[1]);
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
            .title("Lists"));

    f.render_widget(list, area);
}

fn draw_info(f: &mut Frame, app: &App, area: Rect) {
    let total_tasks = app.tasks.len();
    let completed_tasks = app.tasks.iter().filter(|t| t.is_completed).count();
    let my_day_tasks = app.get_my_day_tasks().len();
    let overdue_tasks = app.tasks.iter().filter(|t| t.is_overdue()).count();

    let info_text = format!(
        "Welcome to Todo TUI!\n\n\
        Total Tasks: {}\n\
        Completed: {}\n\
        My Day: {}\n\
        Overdue: {}\n\n\
        Select a list to view tasks, or create a new list with Ctrl+N.\n\
        Use Ctrl+M to view your My Day tasks.",
        total_tasks, completed_tasks, my_day_tasks, overdue_tasks
    );

    let info = Paragraph::new(info_text)
        .style(Style::default().fg(BlulocoTheme::TEXT_PRIMARY))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Overview"));

    f.render_widget(info, area);
}
