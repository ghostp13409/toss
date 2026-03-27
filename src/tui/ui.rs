use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::tui::app::{App, FocusedPanel, InputMode};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(3), // Request Bar
            Constraint::Min(0),    // Main Content
            Constraint::Length(1), // Command/Keybinds
        ])
        .split(f.area());

    // 1. Title
    let title = Paragraph::new(" Toss 1.0.0 - Vim-inspired API Client ")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    // 2. Request Bar
    render_request_bar(f, app, chunks[1]);

    // 3. Main Content (Divided into Middle and Bottom)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Top Half (Collections, Props, Body)
            Constraint::Percentage(50), // Bottom Half (APIs, Response, Stat)
        ])
        .split(chunks[2]);

    render_top_half(f, app, main_chunks[0]);
    render_bottom_half(f, app, main_chunks[1]);

    // 4. Footer (Command or Keybinds)
    render_footer(f, app, chunks[3]);
}

fn render_request_bar(f: &mut Frame, app: &App, area: Rect) {
    let method_color = match app.method {
        crate::cli::args::Method::Get => Color::Green,
        crate::cli::args::Method::Post => Color::Yellow,
        crate::cli::args::Method::Put => Color::Blue,
        crate::cli::args::Method::Patch => Color::Magenta,
        crate::cli::args::Method::Delete => Color::Red,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Request ")
        .border_style(if app.focused_panel == FocusedPanel::RequestBar {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(block.inner(area));

    let method_text = Paragraph::new(format!("{:?}", app.method))
        .style(Style::default().fg(method_color).add_modifier(Modifier::BOLD));
    
    let url_text = Paragraph::new(app.url.as_str())
        .style(if app.focused_panel == FocusedPanel::RequestBar && app.input_mode == InputMode::Editing {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });

    f.render_widget(block, area);
    f.render_widget(method_text, layout[0]);
    f.render_widget(url_text, layout[1]);
}

fn render_top_half(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Collections
            Constraint::Percentage(30), // Props
            Constraint::Percentage(50), // Body (Prop Details)
        ])
        .split(area);

    f.render_widget(create_block(" Collections ", app.focused_panel == FocusedPanel::Collections), chunks[0]);
    f.render_widget(create_block(" Props ", app.focused_panel == FocusedPanel::Props), chunks[1]);
    f.render_widget(create_block(" Body (Selected Prop.) ", app.focused_panel == FocusedPanel::PropDetails), chunks[2]);
}

fn render_bottom_half(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // APIs
            Constraint::Percentage(60), // Response
            Constraint::Percentage(20), // Stats
        ])
        .split(area);

    f.render_widget(create_block(" APIs ", app.focused_panel == FocusedPanel::Apis), chunks[0]);
    f.render_widget(create_block(" Response ", app.focused_panel == FocusedPanel::Response), chunks[1]);
    f.render_widget(create_block(" Response Stat ", app.focused_panel == FocusedPanel::ResponseStat), chunks[2]);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    if app.input_mode == InputMode::Command {
        let command_text = format!(":{}", app.command_input);
        let p = Paragraph::new(command_text).style(Style::default().fg(Color::Cyan));
        f.render_widget(p, area);
    } else {
        let help_text = " Tab: Focus | j/k: Nav | Enter: Select | Ctrl+Enter: Send | /: Search | ?: Help | q: Quit ";
        let p = Paragraph::new(help_text).style(Style::default().bg(Color::Blue).fg(Color::White));
        f.render_widget(p, area);
    }
}

fn create_block(title: &'static str, focused: bool) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        })
}
