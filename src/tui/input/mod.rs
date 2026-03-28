use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::app::{App, InputMode, UiLayer, FocusedPanel, RequestBarPart};
use crate::cli::args::Method;

pub fn handle_input(app: &mut App, key: KeyEvent) {
    if app.show_method_search {
        handle_method_search_input(app, key);
        return;
    }

    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Editing => handle_editing_mode(app, key),
        InputMode::Command => handle_command_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Tab => app.next_panel(),
        
        // Navigation (j/k)
        KeyCode::Char('j') | KeyCode::Down => {
            match app.focused_panel {
                FocusedPanel::Collections => {
                    if app.selected_collection_index < app.collections.len().saturating_sub(1) {
                        app.selected_collection_index += 1;
                        app.selected_api_index = 0; // Reset API index when changing collection
                    }
                }
                FocusedPanel::Apis => {
                    if let Some(col) = app.collections.get(app.selected_collection_index) {
                        if app.selected_api_index < col.items.len().saturating_sub(1) {
                            app.selected_api_index += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            match app.focused_panel {
                FocusedPanel::Collections => {
                    if app.selected_collection_index > 0 {
                        app.selected_collection_index -= 1;
                        app.selected_api_index = 0; // Reset API index when changing collection
                    }
                }
                FocusedPanel::Apis => {
                    if app.selected_api_index > 0 {
                        app.selected_api_index -= 1;
                    }
                }
                _ => {}
            }
        }
        
        KeyCode::Enter | KeyCode::Char('l') => {
            if app.current_layer == UiLayer::LayerRequestBar {
                match app.active_request_part {
                    RequestBarPart::Method => {
                        app.show_method_search = true;
                        app.method_search_query.clear();
                    },
                    RequestBarPart::Send => { /* Trigger Send logic */ },
                    RequestBarPart::Url => app.input_mode = InputMode::Editing,
                }
            } else if app.focused_panel == FocusedPanel::Apis {
                // Load selected request into app state
                if let Some(col) = app.collections.get(app.selected_collection_index) {
                    if let Some(item) = col.items.get(app.selected_api_index) {
                        if let crate::core::collection::CollectionItem::Request(req) = item {
                            app.url = req.url.clone();
                            app.method = req.method;
                            app.current_request_id = Some(req.id.clone());
                            // Load headers/body into state later
                        }
                    }
                }
                app.drill_down();
            } else {
                app.drill_down();
            }
        }
        
        KeyCode::Esc | KeyCode::Char('h') => {
            app.pop_up();
        }

        KeyCode::Char('e') => {
            app.focus_request_bar();
        }

        KeyCode::Char('a') => {
            if let Some(col) = app.collections.get_mut(app.selected_collection_index) {
                let new_req = crate::core::collection::Request {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "New Request".to_string(),
                    method: Method::Get,
                    url: "https://".to_string(),
                    headers: std::collections::HashMap::new(),
                    body: None,
                };
                col.items.push(crate::core::collection::CollectionItem::Request(new_req.clone()));
                app.selected_api_index = col.items.len() - 1;
                app.url = new_req.url;
                app.method = new_req.method;
                app.current_request_id = Some(new_req.id);
                app.focus_request_bar();
            }
        }

        KeyCode::Char('d') => {
            if app.focused_panel == FocusedPanel::Apis {
                if let Some(col) = app.collections.get_mut(app.selected_collection_index) {
                    if !col.items.is_empty() && app.selected_api_index < col.items.len() {
                        col.items.remove(app.selected_api_index);
                        app.selected_api_index = app.selected_api_index.saturating_sub(1);
                    }
                }
            }
        }

        KeyCode::Char(':') => {
            app.input_mode = InputMode::Command;
            app.command_input.clear();
        }

        KeyCode::Char('i') => {
            if app.focused_panel == FocusedPanel::Details {
                app.input_mode = InputMode::Editing;
            }
        }
        
        _ => {}
    }
}

fn handle_editing_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.pop_up(),
        KeyCode::Enter => {
            if app.current_layer == UiLayer::LayerRequestBar && app.active_request_part == RequestBarPart::Url {
                app.active_request_part = RequestBarPart::Send;
                app.input_mode = InputMode::Normal;
            } else {
                app.pop_up();
            }
        },
        KeyCode::Tab => app.next_panel(),
        KeyCode::Char(c) => {
            if app.focused_panel == FocusedPanel::RequestBar && app.active_request_part == RequestBarPart::Url {
                app.url.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.focused_panel == FocusedPanel::RequestBar && app.active_request_part == RequestBarPart::Url {
                app.url.pop();
            }
        }
        _ => {}
    }
}

fn handle_method_search_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.show_method_search = false;
            app.method_search_query.clear();
        }
        KeyCode::Enter => {
            let all_methods = vec!["GET", "POST", "PUT", "PATCH", "DELETE"];
            let filtered: Vec<&str> = all_methods.into_iter()
                .filter(|m| m.contains(&app.method_search_query.to_uppercase()))
                .collect();
            
            if let Some(first) = filtered.first() {
                app.method = match *first {
                    "GET" => Method::Get,
                    "POST" => Method::Post,
                    "PUT" => Method::Put,
                    "PATCH" => Method::Patch,
                    "DELETE" => Method::Delete,
                    _ => Method::Get,
                };
            }
            app.show_method_search = false;
            app.method_search_query.clear();
        }
        KeyCode::Char(c) => {
            app.method_search_query.push(c);
        }
        KeyCode::Backspace => {
            app.method_search_query.pop();
        }
        _ => {}
    }
}

fn handle_command_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.input_mode = InputMode::Normal,
        KeyCode::Enter => {
            match app.command_input.as_str() {
                "q" | "quit" => app.should_quit = true,
                _ => {}
            }
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => app.command_input.push(c),
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        _ => {}
    }
}
