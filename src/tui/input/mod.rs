use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::app::{App, InputMode, UiLayer, FocusedPanel};

pub fn handle_input(app: &mut App, key: KeyEvent) {
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
        
        // Drill Down (Enter or l)
        KeyCode::Enter | KeyCode::Char('l') => {
            app.drill_down();
        }
        
        // Pop Up (Esc or h)
        KeyCode::Esc | KeyCode::Char('h') => {
            app.pop_up();
        }

        KeyCode::Char(':') => {
            app.input_mode = InputMode::Command;
            app.command_input.clear();
        }

        // Enter Editing Mode
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
        KeyCode::Esc | KeyCode::Enter => app.input_mode = InputMode::Normal,
        KeyCode::Char(c) => {
            // Placeholder for editing logic
            if app.focused_panel == FocusedPanel::Details {
                // app.details_content.push(c);
            }
        }
        KeyCode::Backspace => {
            // app.details_content.pop();
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
