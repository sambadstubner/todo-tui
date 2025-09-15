pub mod screens;

use crate::app::App;
use crate::theme::BlulocoTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let terminal_height = f.size().height;
    
    // Dynamic footer sizing based on terminal height
    let footer_height = if terminal_height < 20 {
        2 // Smaller footer for very small terminals
    } else if terminal_height < 30 {
        3 // Standard footer for medium terminals
    } else {
        4 // Larger footer for big terminals
    };
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(footer_height), // Dynamic footer
        ])
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = match app.state {
        crate::app::AppState::ListOverview => "Todo TUI - Lists",
        crate::app::AppState::TaskList => {
            if let Some(list) = app.get_current_list() {
                &format!("Todo TUI - {}", list.name)
            } else {
                "Todo TUI - Tasks"
            }
        }
        crate::app::AppState::MyDay => "Todo TUI - My Day",
        crate::app::AppState::TaskEditor => "Todo TUI - Edit Task",
        crate::app::AppState::ListEditor => "Todo TUI - Create List",
        crate::app::AppState::NoteEditor => "Todo TUI - View Notes",
        crate::app::AppState::MoveTask => "Todo TUI - Move Task",
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(BlulocoTheme::TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(header, area);
}

fn draw_main_content(f: &mut Frame, app: &App, area: Rect) {
    match app.state {
        crate::app::AppState::ListOverview => screens::list_overview::draw(f, app, area),
        crate::app::AppState::TaskList => screens::task_list::draw(f, app, area),
        crate::app::AppState::MyDay => screens::my_day::draw(f, app, area),
        crate::app::AppState::TaskEditor => screens::task_editor::draw(f, app, area),
        crate::app::AppState::ListEditor => screens::list_editor::draw(f, app, area),
        crate::app::AppState::NoteEditor => screens::note_editor::draw(f, app, area),
        crate::app::AppState::MoveTask => screens::move_task::draw(f, app, area),
    }
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.state {
        crate::app::AppState::ListOverview => "[↑↓/jk: Navigate] [gg/G: Top/Bottom] [Enter: Select] [Ctrl+N: New List] [Ctrl+Y: My Day] [Ctrl+Q: Quit]",
        crate::app::AppState::TaskList => "[↑↓/jk: Navigate] [gg/G: Top/Bottom] [Enter: Edit] [Space: Toggle] [Ctrl+N: New] [Ctrl+E: Edit] [Ctrl+V: View Notes] [Ctrl+T: Move] [Del/Backspace: Delete] [Ctrl+D: My Day] [Ctrl+M: My Day] [Esc: Back]",
        crate::app::AppState::MyDay => "[↑↓/jk: Navigate] [gg/G: Top/Bottom] [Space: Toggle] [Ctrl+V: View Notes] [Ctrl+T: Move] [Ctrl+D: Remove] [Del/Backspace: Delete] [Esc: Back]",
        crate::app::AppState::TaskEditor => "[Type: Edit] [Enter: Next/Save] [Esc: Cancel]",
        crate::app::AppState::ListEditor => "[Type: List Name] [Enter: Save] [Esc: Cancel]",
        crate::app::AppState::NoteEditor => "[Type: Edit] [Enter: Save] [Esc: Cancel]",
        crate::app::AppState::MoveTask => "[↑↓/jk: Navigate] [gg/G: Top/Bottom] [Enter: Move Here] [Esc: Cancel]",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(BlulocoTheme::TEXT_SECONDARY))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BlulocoTheme::ACCENT_BLUE))
            .style(Style::default().bg(BlulocoTheme::SURFACE)));

    f.render_widget(footer, area);
}
