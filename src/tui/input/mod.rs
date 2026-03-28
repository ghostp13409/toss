use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::app::{App, InputMode, UiLayer, FocusedPanel, RequestBarPart, PendingItemType};
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
        InputMode::Rename => handle_rename_mode(app, key),
        InputMode::Search => handle_search_mode(app, key),
        InputMode::ConfirmDelete => handle_confirm_delete_mode(app, key),
        InputMode::ConfirmQuit => handle_confirm_quit_mode(app, key),
        InputMode::CreateItem => handle_create_item_mode(app, key),
    }
}

fn handle_create_item_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.pending_item_type = None;
            app.rename_input.clear();
        }
        KeyCode::Enter => {
            if let Some(item_type) = app.pending_item_type {
                let name = app.rename_input.trim().to_string();
                match item_type {
                    PendingItemType::Collection => app.add_collection(name),
                    PendingItemType::Folder => app.add_folder(name),
                    PendingItemType::Request => app.add_request(name),
                }
            }
            app.input_mode = InputMode::Normal;
            app.pending_item_type = None;
            app.rename_input.clear();
        }
        KeyCode::Char(c) => app.rename_input.push(c),
        KeyCode::Backspace => {
            app.rename_input.pop();
        }
        _ => {}
    }
}

fn handle_confirm_quit_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
            app.should_quit = true;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_confirm_delete_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => {
            app.delete_selected_item();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_search_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.show_search = false;
            app.search_query.clear();
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.selected_api_index = 0; // Reset index when searching
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.selected_api_index = 0;
        }
        _ => {}
    }
}

fn handle_rename_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.input_mode = InputMode::Normal,
        KeyCode::Enter => {
            app.rename_selected_item();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => app.rename_input.push(c),
        KeyCode::Backspace => {
            app.rename_input.pop();
        }
        _ => {}
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.input_mode = InputMode::ConfirmQuit,
        KeyCode::Tab => app.next_panel(),
        
        // Navigation (j/k)
        KeyCode::Char('j') | KeyCode::Down => {
            match app.focused_panel {
                FocusedPanel::Collections => {
                    let max_idx = app.get_visible_collections().len().saturating_sub(1);
                    if app.selected_collection_index < max_idx {
                        app.selected_collection_index += 1;
                        app.update_active_scope_from_tree();
                    }
                }
                FocusedPanel::Apis => {
                    let visible_items = app.get_visible_items();
                    if app.selected_api_index < visible_items.len().saturating_sub(1) {
                        app.selected_api_index += 1;
                    }
                }
                FocusedPanel::Response => {
                    // Navigate response if needed
                }
                _ => {}
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            match app.focused_panel {
                FocusedPanel::Collections => {
                    if app.selected_collection_index > 0 {
                        app.selected_collection_index -= 1;
                        app.update_active_scope_from_tree();
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
                    RequestBarPart::Send => { 
                        // Simulate Send: Focus response
                        app.current_layer = UiLayer::Layer4;
                        app.focused_panel = FocusedPanel::Response;
                    },
                    RequestBarPart::Url => app.input_mode = InputMode::Editing,
                }
            } else if app.focused_panel == FocusedPanel::Apis {
                let visible_items = app.get_visible_items();
                if let Some(item) = visible_items.get(app.selected_api_index) {
                    match &item.item_type {
                        crate::tui::app::VisibleItemType::Folder { .. } => {
                            app.toggle_folder();
                        }
                        crate::tui::app::VisibleItemType::Request { method, id, .. } => {
                            app.save_current_request();
                            app.current_request_id = Some(id.clone());
                            app.method = *method;
                            let id_clone = id.clone();
                            if let Some(col) = app.collections.get_mut(app.active_collection_index) {
                                if let Some(req) = col.find_request_mut(&id_clone) {
                                    app.url = req.url.clone();
                                }
                            }
                            app.focus_request_bar();
                        }
                        _ => {}
                    }
                }
            } else if app.focused_panel == FocusedPanel::Collections {
                let visible_collections = app.get_visible_collections();
                if let Some(item) = visible_collections.get(app.selected_collection_index) {
                    match &item.item_type {
                        crate::tui::app::VisibleItemType::Collection { .. } | crate::tui::app::VisibleItemType::Folder { .. } => {
                            app.toggle_folder();
                        }
                        crate::tui::app::VisibleItemType::Request { method, id, .. } => {
                            // Support direct selection from Collections panel
                            app.save_current_request();
                            app.current_request_id = Some(id.clone());
                            app.method = *method;
                            let id_clone = id.clone();
                            for col in &mut app.collections {
                                if let Some(req) = col.find_request_mut(&id_clone) {
                                    app.url = req.url.clone();
                                    break;
                                }
                            }
                            app.focus_request_bar();
                        }
                    }
                }
            } else {
                app.drill_down();
            }
        }
        
        KeyCode::Esc | KeyCode::Char('h') => {
            app.pop_up();
        }

        KeyCode::Char('p') => {
            if app.current_request_id.is_some() {
                app.current_layer = UiLayer::Layer2;
                app.focused_panel = FocusedPanel::Properties;
            }
        }

        KeyCode::Char(' ') => {
            if app.focused_panel == FocusedPanel::Apis || app.focused_panel == FocusedPanel::Collections {
                app.toggle_folder();
            }
        }

        KeyCode::Char('/') => {
            if app.focused_panel == FocusedPanel::Apis || app.focused_panel == FocusedPanel::Collections {
                app.input_mode = InputMode::Search;
                app.show_search = true;
                app.search_query.clear();
            }
        }

        KeyCode::Char('e') => {
            app.focus_request_bar();
        }

        KeyCode::Char('a') => {
            if app.focused_panel == FocusedPanel::Apis || app.focused_panel == FocusedPanel::Collections {
                app.input_mode = InputMode::CreateItem;
                app.pending_item_type = Some(PendingItemType::Request);
                app.rename_input.clear();
            }
        }

        KeyCode::Char('f') => {
            if app.focused_panel == FocusedPanel::Apis || app.focused_panel == FocusedPanel::Collections {
                app.input_mode = InputMode::CreateItem;
                app.pending_item_type = Some(PendingItemType::Folder);
                app.rename_input.clear();
            }
        }

        KeyCode::Char('n') => {
            if app.focused_panel == FocusedPanel::Collections {
                app.input_mode = InputMode::CreateItem;
                app.pending_item_type = Some(PendingItemType::Collection);
                app.rename_input.clear();
            }
        }

        KeyCode::Char('r') => {
            app.input_mode = InputMode::Rename;
            app.rename_input.clear();
            if app.focused_panel == FocusedPanel::Collections {
                let visible_collections = app.get_visible_collections();
                if let Some(item) = visible_collections.get(app.selected_collection_index) {
                    app.rename_input = item.name.clone();
                }
            } else if app.focused_panel == FocusedPanel::Apis {
                let visible_items = app.get_visible_items();
                if let Some(item) = visible_items.get(app.selected_api_index) {
                    app.rename_input = item.name.clone();
                }
            }
        }

        KeyCode::Char('d') => {
            app.input_mode = InputMode::ConfirmDelete;
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
        KeyCode::Esc => {
            app.save_current_request();
            app.pop_up();
        }
        KeyCode::Enter => {
            if app.current_layer == UiLayer::LayerRequestBar && app.active_request_part == RequestBarPart::Url {
                app.save_current_request();
                app.active_request_part = RequestBarPart::Send;
                app.input_mode = InputMode::Normal;
            } else {
                app.save_current_request();
                app.pop_up();
            }
        },
        KeyCode::Tab => {
            app.save_current_request();
            app.next_panel();
        }
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
            let cmd = app.command_input.clone();
            if cmd.starts_with("import ") {
                let path = &cmd[7..];
                app.import_collection(path);
            } else {
                match cmd.as_str() {
                    "q" | "quit" => app.input_mode = InputMode::ConfirmQuit,
                    "save" => {
                        app.save_collections();
                    }
                    _ => {}
                }
            }
            app.input_mode = InputMode::Normal;
            app.command_input.clear();
        }
        KeyCode::Char(c) => app.command_input.push(c),
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        _ => {}
    }
}
