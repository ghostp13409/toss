use crate::cli::args::Method;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
    Command,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UiLayer {
    Layer1, // Left Column (Collections, APIs)
    Layer2, // Right Column Top (Request Bar, Properties)
    Layer3, // Right Column Middle (Property Details/Editor)
    Layer4, // Right Column Bottom (Response)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusedPanel {
    Collections,
    Apis,
    Properties,
    Details,
    Response,
}

pub struct App {
    pub input_mode: InputMode,
    pub current_layer: UiLayer,
    pub focused_panel: FocusedPanel,
    pub last_focused_in_layer1: FocusedPanel,
    pub url: String,
    pub method: Method,
    pub command_input: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            current_layer: UiLayer::Layer1,
            focused_panel: FocusedPanel::Collections,
            last_focused_in_layer1: FocusedPanel::Collections,
            url: "https://httpbin.org/get".to_string(),
            method: Method::Get,
            command_input: String::new(),
            should_quit: false,
        }
    }

    /// Drill down into the next logical layer (Enter/l)
    pub fn drill_down(&mut self) {
        match self.current_layer {
            UiLayer::Layer1 => {
                self.current_layer = UiLayer::Layer2;
                self.focused_panel = FocusedPanel::Properties;
            }
            UiLayer::Layer2 => {
                self.current_layer = UiLayer::Layer3;
                self.focused_panel = FocusedPanel::Details;
            }
            _ => {}
        }
    }

    /// Pop up to the previous logical layer (Esc/h)
    pub fn pop_up(&mut self) {
        match self.current_layer {
            UiLayer::Layer2 => {
                self.current_layer = UiLayer::Layer1;
                self.focused_panel = self.last_focused_in_layer1;
            }
            UiLayer::Layer3 => {
                self.current_layer = UiLayer::Layer2;
                self.focused_panel = FocusedPanel::Properties;
            }
            UiLayer::Layer4 => {
                self.current_layer = UiLayer::Layer2;
                self.focused_panel = FocusedPanel::Properties;
            }
            _ => {}
        }
    }

    /// Cycle focus within the current layer (Tab)
    pub fn next_panel(&mut self) {
        if self.current_layer == UiLayer::Layer1 {
            self.focused_panel = match self.focused_panel {
                FocusedPanel::Collections => FocusedPanel::Apis,
                FocusedPanel::Apis => FocusedPanel::Collections,
                _ => FocusedPanel::Collections,
            };
            self.last_focused_in_layer1 = self.focused_panel;
        }
    }
}
