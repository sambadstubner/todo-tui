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
            Constraint::Min(0),    // Note content
            Constraint::Length(3), // Help text
        ])
        .split(area);

    draw_title(f, app, chunks[0]);
    draw_notes(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let title = if let Some(task) = &app.editing_task {
        format!("Edit Notes for: {}", task.title)
    } else {
        "Edit Notes".to_string()
    };

    let title_widget = Paragraph::new(title)
        .style(Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(title_widget, area);
}

fn draw_notes(f: &mut Frame, app: &App, area: Rect) {
    let note_text = if app.input_buffer.is_empty() {
        "Enter your notes here...".to_string()
    } else {
        app.input_buffer.clone()
    };

    let style = if app.input_buffer.is_empty() {
        Style::default().fg(BlulocoTheme::TEXT_MUTED)
    } else {
        Style::default().fg(BlulocoTheme::TEXT_PRIMARY)
    };

    let notes_widget = Paragraph::new(note_text)
        .style(style)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Notes"));

    f.render_widget(notes_widget, area);
}

fn draw_help(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = "Type your notes and press Enter to save.\nPress Esc to cancel.";
    
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(BlulocoTheme::TEXT_SECONDARY))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Help"));

    f.render_widget(help_widget, area);
}
