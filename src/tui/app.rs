use crate::cli::args::Method;

#[derive(Debug, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
    Command,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusedPanel {
    RequestBar,
    Collections,
    Props,
    PropDetails,
    Apis,
    Response,
    ResponseStat,
}

pub struct App {
    pub input_mode: InputMode,
    pub focused_panel: FocusedPanel,
    pub url: String,
    pub method: Method,
    pub command_input: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            focused_panel: FocusedPanel::Collections,
            url: String::new(),
            method: Method::Get,
            command_input: String::new(),
            should_quit: false,
        }
    }

    pub fn next_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::RequestBar => FocusedPanel::Collections,
            FocusedPanel::Collections => FocusedPanel::Props,
            FocusedPanel::Props => FocusedPanel::PropDetails,
            FocusedPanel::PropDetails => FocusedPanel::Apis,
            FocusedPanel::Apis => FocusedPanel::Response,
            FocusedPanel::Response => FocusedPanel::ResponseStat,
            FocusedPanel::ResponseStat => FocusedPanel::RequestBar,
        };
    }

    pub fn previous_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::RequestBar => FocusedPanel::ResponseStat,
            FocusedPanel::Collections => FocusedPanel::RequestBar,
            FocusedPanel::Props => FocusedPanel::Collections,
            FocusedPanel::PropDetails => FocusedPanel::Props,
            FocusedPanel::Apis => FocusedPanel::PropDetails,
            FocusedPanel::Response => FocusedPanel::Apis,
            FocusedPanel::ResponseStat => FocusedPanel::Response,
        };
    }
}
