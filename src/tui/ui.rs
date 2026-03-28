use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::{App, FocusedPanel, InputMode, PendingItemType, RequestBarPart, UiLayer};
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
    if app.input_mode == InputMode::Rename {
        render_rename_popup(f, app);
    }
    if app.input_mode == InputMode::CreateItem {
        render_create_popup(f, app);
    }
    if app.input_mode == InputMode::ConfirmDelete {
        render_delete_confirmation(f, app);
    }
    if app.input_mode == InputMode::ConfirmQuit {
        render_quit_confirmation(f, app);
    }
    if app.show_search {
        render_search_popup(f, app, columns[0]);
    }
    if let Some(error) = &app.error_message {
        render_error_popup(f, error);
    }

    // 5. Cursor positioning
    match app.input_mode {
        InputMode::Editing if app.current_layer == UiLayer::LayerRequestBar && app.active_request_part == RequestBarPart::Url => {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Request ");
            let area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),      // Request Bar
                    Constraint::Length(8),      // Properties
                    Constraint::Percentage(40), // Details
                    Constraint::Min(0),         // Response area
                ])
                .split(columns[1])[0];

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(10), // Method
                    Constraint::Min(0),    // URL
                    Constraint::Length(10), // Send Button
                ])
                .split(block.inner(area));
            
            f.set_cursor_position((
                layout[1].x + app.cursor_position as u16,
                layout[1].y,
            ));
        }
        InputMode::Rename | InputMode::CreateItem => {
            let area = centered_rect(40, 10, f.area());
            f.set_cursor_position((
                area.x + 1 + app.cursor_position as u16,
                area.y + 1,
            ));
        }
        InputMode::Search if app.show_search => {
            let sidebar_area = columns[0];
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(sidebar_area);
            let area = chunks[1];
            f.set_cursor_position((
                area.x + 1 + app.cursor_position as u16,
                area.y + 1,
            ));
        }
        InputMode::Command => {
            f.set_cursor_position((
                chunks[2].x + 1 + app.cursor_position as u16 + 1, // +1 for ':'
                chunks[2].y,
            ));
        }
        _ => {
            if app.show_method_search {
                let area = centered_rect(20, 30, f.area());
                f.set_cursor_position((
                    area.x + 1 + app.cursor_position as u16,
                    area.y + 1,
                ));
            }
        }
    }
}

fn render_error_popup(f: &mut Frame, error: &str) {
    let area = centered_rect(60, 20, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Error ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    
    let p = Paragraph::new(error)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}

fn render_create_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(40, 10, f.area());
    f.render_widget(Clear, area);

    let title = match app.pending_item_type {
        Some(PendingItemType::Collection) => " Create Collection ",
        Some(PendingItemType::Folder) => " Create Folder ",
        Some(PendingItemType::Request) => " Create Request ",
        None => " Create Item ",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    
    let p = Paragraph::new(app.rename_input.as_str())
        .block(block);
    f.render_widget(p, area);
}

fn render_quit_confirmation(f: &mut Frame, _app: &App) {
    let area = centered_rect(30, 10, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Confirm Quit ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    
    let p = Paragraph::new("Quit application? (y/n)")
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}

fn render_delete_confirmation(f: &mut Frame, app: &App) {
    let area = centered_rect(30, 10, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Confirm Delete ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    
    let text = if app.focused_panel == FocusedPanel::Collections {
        "Delete entire collection? (y/n)"
    } else {
        "Delete selected item? (y/n)"
    };
    
    let p = Paragraph::new(text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}

fn render_search_popup(f: &mut Frame, app: &App, sidebar_area: Rect) {
    // Position search popup at the bottom of the sidebar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(sidebar_area);

    let area = chunks[1];
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Filter (/) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    
    let p = Paragraph::new(app.search_query.as_str())
        .block(block);
    f.render_widget(p, area);
}

fn render_rename_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(40, 10, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Rename ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    
    let p = Paragraph::new(app.rename_input.as_str())
        .block(block);
    f.render_widget(p, area);
}

fn get_method_color(method_str: &str) -> Color {
    match method_str.to_uppercase().as_str() {
        "GET" => Color::Green,
        "POST" => Color::Yellow,
        "PUT" => Color::Blue,
        "PATCH" => Color::Magenta,
        "DELETE" => Color::Red,
        _ => Color::Reset,
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

    // Collections Panel using visible collections (all collections and their items)
    let visible_collections = app.get_visible_collections();
    let collections_items: Vec<ListItem> = visible_collections.iter().enumerate().map(|(i, item)| {
        let indent = "  ".repeat(item.item_type_depth());
        let is_selected = app.focused_panel == FocusedPanel::Collections && i == app.selected_collection_index;
        
        match &item.item_type {
            crate::tui::app::VisibleItemType::Collection { expanded } => {
                let icon = if *expanded { "▼" } else { "▶" };
                let style = if is_selected {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::REVERSED | Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Cyan)
                };
                ListItem::new(format!("{}{} {} {}", indent, icon, "📦", item.name)).style(style)
            }
            crate::tui::app::VisibleItemType::Folder { expanded } => {
                let icon = if *expanded { "▼" } else { "▶" };
                let style = if is_selected {
                    Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{} {} {}", indent, icon, "📁", item.name)).style(style)
            }
            crate::tui::app::VisibleItemType::Request { method, .. } => {
                let color = get_method_enum_color(*method);
                let style = if is_selected {
                    Style::default().fg(color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
                } else {
                    Style::default().fg(color)
                };
                ListItem::new(format!("{}{} {:?}  {}", indent, "  ", method, item.name)).style(style)
            }
        }
    }).collect();

    let collections_list = List::new(collections_items)
        .block(create_block(" Collections ", app.focused_panel == FocusedPanel::Collections));
    f.render_widget(collections_list, chunks[0]);

    // APIs Panel using visible items
    let visible_items = app.get_visible_items();
    let api_items: Vec<ListItem> = visible_items.iter().enumerate().map(|(i, item)| {
        let indent = "  ".repeat(item.item_type_depth());
        let is_selected = app.focused_panel == FocusedPanel::Apis && i == app.selected_api_index;
        
        match &item.item_type {
            crate::tui::app::VisibleItemType::Collection { .. } => {
                // Should not happen in APIs panel currently
                ListItem::new(format!("{}📦 {}", indent, item.name))
            }
            crate::tui::app::VisibleItemType::Folder { expanded } => {
                let icon = if *expanded { "▼" } else { "▶" };
                let style = if is_selected {
                    Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{} {} {}", indent, icon, "📁", item.name)).style(style)
            }
            crate::tui::app::VisibleItemType::Request { method, .. } => {
                let color = get_method_enum_color(*method);
                let style = if is_selected {
                    Style::default().fg(color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
                } else {
                    Style::default().fg(color)
                };
                ListItem::new(format!("{}{} {:?}  {}", indent, "  ", method, item.name)).style(style)
            }
        }
    }).collect();

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
        Style::default().fg(method_color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
    } else {
        Style::default().fg(method_color).add_modifier(Modifier::BOLD)
    };
    let method_text = Paragraph::new(format!(" {:?} ", app.method)).style(method_style);
    
    // URL
    let url_style = if is_bar_focused && app.active_request_part == RequestBarPart::Url {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    };
    let url_text = Paragraph::new(app.url.as_str()).style(url_style);

    // Send Button
    let send_style = if is_bar_focused && app.active_request_part == RequestBarPart::Send {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED | Modifier::BOLD)
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
                Style::default().fg(color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
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
    } else if app.input_mode == InputMode::Rename {
        "Enter: Confirm | Esc: Cancel".to_string()
    } else if app.input_mode == InputMode::CreateItem {
        "Enter: Create (Empty for default) | Esc: Cancel".to_string()
    } else if app.input_mode == InputMode::Search {
        format!("Filter: {} (Esc: Clear, Enter: Keep)", app.search_query)
    } else if app.input_mode == InputMode::ConfirmDelete {
        "ARE YOU SURE? (y/n)".to_string()
    } else {
        match app.focused_panel {
            FocusedPanel::Collections | FocusedPanel::Apis => {
                "Tab: Cycle | Enter: Open | /: Filter | Space: Toggle | a: Req | f: Folder | n: Collection | r: Rename | d: Delete".to_string()
            }
            FocusedPanel::RequestBar => "[Request] Tab: Cycle Controls | Enter: Trigger | Esc: Back".to_string(),
            _ => "Tab: Cycle | Esc: Back | Ctrl+Enter: Send".to_string(),
        }
    };
    
    let p = Paragraph::new(text).style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_widget(p, area);
}

fn create_block(title: &'static str, focused: bool) -> Block<'static> {
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
