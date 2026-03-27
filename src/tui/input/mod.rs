use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::app::{App, InputMode, FocusedPanel};

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
        KeyCode::BackTab => app.previous_panel(),
        KeyCode::Char(':') => {
            app.input_mode = InputMode::Command;
            app.command_input.clear();
        }
        KeyCode::Char('i') => {
            if app.focused_panel == FocusedPanel::RequestBar {
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
            if app.focused_panel == FocusedPanel::RequestBar {
                app.url.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.focused_panel == FocusedPanel::RequestBar {
                app.url.pop();
            }
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
