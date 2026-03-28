use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::app::{App, FocusedPanel, InputMode, RequestBarPart, UiLayer};
use crate::cli::args::Method;

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

    // 4. Overlays
    if app.show_method_search {
        render_method_search(f, app);
    }
}

fn get_method_color(method_str: &str) -> Color {
    match method_str.to_uppercase().as_str() {
        "GET" => Color::Green,
        "POST" => Color::Yellow,
        "PUT" => Color::Blue,
        "PATCH" => Color::Magenta,
        "DELETE" => Color::Red,
        _ => Color::White,
    }
}

fn get_method_enum_color(method: Method) -> Color {
    match method {
        Method::Get => Color::Green,
        Method::Post => Color::Yellow,
        Method::Put => Color::Blue,
        Method::Patch => Color::Magenta,
        Method::Delete => Color::Red,
    }
}

fn render_left_column(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Collections
            Constraint::Percentage(50), // APIs
        ])
        .split(area);

    let collections_items: Vec<ListItem> = app.collections.iter().enumerate().map(|(i, col)| {
        let style = if app.focused_panel == FocusedPanel::Collections && i == app.selected_collection_index {
            Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        ListItem::new(format!(" > {}", col.name)).style(style)
    }).collect();

    let collections_list = List::new(collections_items)
        .block(create_block(" Collections ", app.focused_panel == FocusedPanel::Collections));
    f.render_widget(collections_list, chunks[0]);

    // APIs Panel (scoped to selected collection for now)
    let mut api_items = Vec::new();
    if let Some(col) = app.collections.get(app.selected_collection_index) {
        for item in &col.items {
            match item {
                crate::core::collection::CollectionItem::Request(req) => {
                    let color = get_method_enum_color(req.method);
                    let style = if app.focused_panel == FocusedPanel::Apis && api_items.len() == app.selected_api_index {
                        Style::default().bg(color).fg(Color::Black).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(color)
                    };
                    api_items.push(ListItem::new(format!(" {:?}  {}", req.method, req.name)).style(style));
                }
                crate::core::collection::CollectionItem::Folder(f) => {
                    api_items.push(ListItem::new(format!(" v {}", f.name)).style(Style::default().add_modifier(Modifier::DIM)));
                    // In future, recursively show folder contents if expanded
                }
            }
        }
    }

    let apis_list = List::new(api_items)
        .block(create_block(" APIs ", app.focused_panel == FocusedPanel::Apis));
    f.render_widget(apis_list, chunks[1]);
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
    let is_bar_focused = app.current_layer == UiLayer::LayerRequestBar;
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Request ")
        .border_style(if is_bar_focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        });

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(10), // Method
            Constraint::Min(0),    // URL
            Constraint::Length(10), // Send Button
        ])
        .split(block.inner(area));

    // Method Badge
    let method_color = get_method_enum_color(app.method);
    let method_style = if is_bar_focused && app.active_request_part == RequestBarPart::Method {
        Style::default().bg(method_color).fg(Color::Black).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(method_color).add_modifier(Modifier::BOLD)
    };
    let method_text = Paragraph::new(format!(" {:?} ", app.method)).style(method_style);
    
    // URL
    let url_style = if is_bar_focused && app.active_request_part == RequestBarPart::Url {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    } else {
        Style::default()
    };
    let url_text = Paragraph::new(app.url.as_str()).style(url_style);

    // Send Button
    let send_style = if is_bar_focused && app.active_request_part == RequestBarPart::Send {
        Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Yellow)
    };
    let send_button = Paragraph::new(" [ Send ] ").style(send_style);

    f.render_widget(block, area);
    f.render_widget(method_text, layout[0]);
    f.render_widget(url_text, layout[1]);
    f.render_widget(send_button, layout[2]);
}

fn render_method_search(f: &mut Frame, app: &App) {
    let area = centered_rect(20, 30, f.area());
    f.render_widget(Clear, area); // Clear the background
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search box
            Constraint::Min(0),    // Results
        ])
        .split(area);

    // Search Box
    let search_block = Block::default()
        .title(" Search Method ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let search_text = Paragraph::new(app.method_search_query.as_str())
        .block(search_block);
    f.render_widget(search_text, chunks[0]);

    // Results logic
    let all_methods = vec!["GET", "POST", "PUT", "PATCH", "DELETE"];
    let filtered_methods: Vec<&str> = all_methods.into_iter()
        .filter(|m| m.contains(&app.method_search_query.to_uppercase()))
        .collect();

    let list_items: Vec<ListItem> = filtered_methods.iter()
        .enumerate()
        .map(|(i, m)| {
            let color = get_method_color(m);
            let style = if i == 0 { // Highlight the top match
                Style::default().bg(color).fg(Color::Black).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };
            ListItem::new(*m).style(style)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));
    f.render_widget(list, chunks[1]);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let text = if app.input_mode == InputMode::Command {
        format!(":{}", app.command_input)
    } else {
        match app.focused_panel {
            FocusedPanel::Collections | FocusedPanel::Apis => "[Sidebar] Tab: Cycle | Enter: Select | e: Edit URL",
            FocusedPanel::RequestBar => "[Request] Tab: Cycle Controls | Enter: Trigger | Esc: Back",
            _ => "Tab: Cycle | Esc: Back | Ctrl+Enter: Send",
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
