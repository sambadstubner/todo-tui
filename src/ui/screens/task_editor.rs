use crate::app::App;
use crate::theme::BlulocoTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Current field label
            Constraint::Length(3), // Input field
            Constraint::Min(0),    // Help text
        ])
        .split(area);

    draw_title(f, app, chunks[0]);
    draw_field_label(f, app, chunks[1]);
    draw_input(f, app, chunks[2]);
    draw_help(f, app, chunks[3]);
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let title = match app.task_editor_mode {
        crate::app::TaskEditorMode::Create => "Create New Task",
        crate::app::TaskEditorMode::Edit => "Edit Task",
    };

    let title_widget = Paragraph::new(title)
        .style(Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(title_widget, area);
}

fn draw_field_label(f: &mut Frame, app: &App, area: Rect) {
    let field_name = match app.task_editor_state {
        crate::app::TaskEditorState::Title => "Task Title",
        crate::app::TaskEditorState::DueDate => "Due Date (YYYY-MM-DD, 'today', 'tomorrow', etc.)",
        crate::app::TaskEditorState::Frequency => "Frequency (daily, weekdays, weekly, monthly, yearly, or 'none')",
        crate::app::TaskEditorState::Notes => "Notes (optional)",
    };

    let label_widget = Paragraph::new(field_name)
        .style(Style::default().fg(BlulocoTheme::TEXT_PRIMARY))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(label_widget, area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let placeholder = match app.task_editor_state {
        crate::app::TaskEditorState::Title => "Enter task title...",
        crate::app::TaskEditorState::DueDate => "Enter due date or press Enter to skip...",
        crate::app::TaskEditorState::Frequency => "Enter frequency or press Enter to skip...",
        crate::app::TaskEditorState::Notes => "Enter notes or press Enter to skip...",
    };

    let input_text = if app.input_buffer.is_empty() {
        placeholder.to_string()
    } else {
        app.input_buffer.clone()
    };

    let style = if app.input_buffer.is_empty() {
        Style::default().fg(BlulocoTheme::TEXT_MUTED)
    } else {
        Style::default().fg(BlulocoTheme::TEXT_PRIMARY)
    };

    let input_widget = Paragraph::new(input_text)
        .style(style)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Input"));

    f.render_widget(input_widget, area);
}

fn draw_help(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.task_editor_state {
        crate::app::TaskEditorState::Title => "Enter the task title and press Enter to continue.\nPress Esc to cancel.",
        crate::app::TaskEditorState::DueDate => "Enter due date (YYYY-MM-DD) or relative date like 'today', 'tomorrow'.\nPress Enter to skip or continue to next field.",
        crate::app::TaskEditorState::Frequency => "Enter frequency: 'daily', 'weekdays', 'weekly', 'monthly', 'yearly', or 'none'.\nPress Enter to skip or continue to next field.",
        crate::app::TaskEditorState::Notes => "Enter optional notes for this task.\nPress Enter to save the task or skip notes.",
    };
    
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(BlulocoTheme::TEXT_SECONDARY))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Help"));

    f.render_widget(help_widget, area);
}
