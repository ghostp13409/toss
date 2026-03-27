use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::{App, FocusedPanel, InputMode};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Min(0),    // Main Content
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    // 1. Title
    let title = Paragraph::new(" Toss 1.0.0 ")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(title, chunks[0]);

    // 2. Main Columns (30/70)
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Left Column
            Constraint::Percentage(70), // Right Column
        ])
        .split(chunks[1]);

    render_left_column(f, app, columns[0]);
    render_right_column(f, app, columns[1]);

    // 3. Footer
    render_footer(f, app, chunks[2]);
}

fn render_left_column(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Collections
            Constraint::Percentage(50), // APIs
        ])
        .split(area);

    f.render_widget(
        create_block(" Collections ", app.focused_panel == FocusedPanel::Collections),
        chunks[0],
    );
    f.render_widget(
        create_block(" APIs ", app.focused_panel == FocusedPanel::Apis),
        chunks[1],
    );
}

fn render_right_column(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Request Bar
            Constraint::Length(8),      // Properties
            Constraint::Percentage(40), // Details
            Constraint::Min(0),         // Response area
        ])
        .split(area);

    render_request_bar(f, app, chunks[0]);
    f.render_widget(create_block(" Properties ", app.focused_panel == FocusedPanel::Properties), chunks[1]);
    f.render_widget(create_block(" Property Details / Editor ", app.focused_panel == FocusedPanel::Details), chunks[2]);

    let response_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80), // Response Body
            Constraint::Percentage(20), // Stats
        ])
        .split(chunks[3]);

    f.render_widget(create_block(" Response ", app.focused_panel == FocusedPanel::Response), response_area[0]);
    f.render_widget(create_block(" Stat ", false), response_area[1]);
}

fn render_request_bar(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Request ")
        .border_style(Style::default());

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(block.inner(area));

    let method_text = Paragraph::new(format!("{:?}", app.method))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
    
    let url_text = Paragraph::new(app.url.as_str())
        .style(if app.input_mode == InputMode::Editing {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });

    f.render_widget(block, area);
    f.render_widget(method_text, layout[0]);
    f.render_widget(url_text, layout[1]);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let text = if app.input_mode == InputMode::Command {
        format!(":{}", app.command_input)
    } else {
        match app.focused_panel {
            FocusedPanel::Collections | FocusedPanel::Apis => "[Collections] j/k: Nav | Enter/l: Select | Tab: Switch",
            FocusedPanel::Properties => "[Properties] j/k: Nav | Enter/l: Edit | Esc/h: Back",
            FocusedPanel::Details => "[Editor] i: Edit | Esc/h: Back",
            _ => "j/k: Nav | Esc: Back | Ctrl+Enter: Send",
        }.to_string()
    };
    
    let p = Paragraph::new(text).style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(p, area);
}

fn create_block(title: &'static str, focused: bool) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(if focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        })
}
