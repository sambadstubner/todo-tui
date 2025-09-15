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
            Constraint::Length(3), // Input field
            Constraint::Min(0),    // Help text
        ])
        .split(area);

    draw_title(f, app, chunks[0]);
    draw_input(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);
}

fn draw_title(f: &mut Frame, _app: &App, area: Rect) {
    let title_widget = Paragraph::new("Create New List")
        .style(Style::default().fg(BlulocoTheme::FOCUS).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(title_widget, area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let input_text = if app.input_buffer.is_empty() {
        "Enter list name...".to_string()
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
            .title("List Name"));

    f.render_widget(input_widget, area);
}

fn draw_help(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = "Type the list name and press Enter to create the list.\nPress Esc to cancel.";
    
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(BlulocoTheme::TEXT_SECONDARY))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE))
            .title("Help"));

    f.render_widget(help_widget, area);
}
