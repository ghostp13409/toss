use crate::cli::args::Method;
use crate::core::collection::Collection;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
    Command,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UiLayer {
    Layer1, // Sidebar (Collections/APIs)
    Layer2, // Right Column (Properties)
    Layer3, // Right Column (Details Editor)
    Layer4, // Right Column (Response)
    LayerRequestBar, // URL/Method/Send Bar
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusedPanel {
    Collections,
    Apis,
    Properties,
    Details,
    Response,
    RequestBar,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RequestBarPart {
    Method,
    Url,
    Send,
}

pub struct App {
    pub input_mode: InputMode,
    pub current_layer: UiLayer,
    pub focused_panel: FocusedPanel,
    pub active_request_part: RequestBarPart,
    pub show_method_search: bool,
    pub method_search_query: String,
    pub last_focused_in_layer1: FocusedPanel,
    pub last_focused_layer: UiLayer,
    pub url: String,
    pub method: Method,
    pub command_input: String,
    pub should_quit: bool,
    pub collections: Vec<Collection>,
    pub current_request_id: Option<String>,
    pub selected_collection_index: usize,
    pub selected_api_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            current_layer: UiLayer::Layer1,
            focused_panel: FocusedPanel::Collections,
            active_request_part: RequestBarPart::Url,
            show_method_search: false,
            method_search_query: String::new(),
            last_focused_in_layer1: FocusedPanel::Collections,
            last_focused_layer: UiLayer::Layer1,
            url: "https://httpbin.org/get".to_string(),
            method: Method::Get,
            command_input: String::new(),
            should_quit: false,
            collections: Vec::new(),
            current_request_id: None,
            selected_collection_index: 0,
            selected_api_index: 0,
        }
    }

    pub fn focus_request_bar(&mut self) {
        if self.current_layer != UiLayer::LayerRequestBar {
            self.last_focused_layer = self.current_layer;
        }
        self.current_layer = UiLayer::LayerRequestBar;
        self.focused_panel = FocusedPanel::RequestBar;
        self.active_request_part = RequestBarPart::Url; // Default to URL
        self.input_mode = InputMode::Editing;
    }

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

    pub fn pop_up(&mut self) {
        if self.show_method_search {
            self.show_method_search = false;
            self.method_search_query.clear();
            return;
        }

        match self.current_layer {
            UiLayer::LayerRequestBar => {
                self.current_layer = self.last_focused_layer;
                self.focused_panel = match self.current_layer {
                    UiLayer::Layer1 => self.last_focused_in_layer1,
                    UiLayer::Layer2 => FocusedPanel::Properties,
                    UiLayer::Layer3 => FocusedPanel::Details,
                    UiLayer::Layer4 => FocusedPanel::Response,
                    _ => FocusedPanel::Collections,
                };
                self.input_mode = InputMode::Normal;
            }
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

    pub fn next_panel(&mut self) {
        match self.current_layer {
            UiLayer::Layer1 => {
                self.focused_panel = match self.focused_panel {
                    FocusedPanel::Collections => FocusedPanel::Apis,
                    FocusedPanel::Apis => FocusedPanel::Collections,
                    _ => FocusedPanel::Collections,
                };
                self.last_focused_in_layer1 = self.focused_panel;
            }
            UiLayer::LayerRequestBar => {
                self.active_request_part = match self.active_request_part {
                    RequestBarPart::Method => RequestBarPart::Url,
                    RequestBarPart::Url => RequestBarPart::Send,
                    RequestBarPart::Send => RequestBarPart::Method,
                };
                // Auto-enter editing if URL is focused
                if self.active_request_part == RequestBarPart::Url {
                    self.input_mode = InputMode::Editing;
                } else {
                    self.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        }
    }

    pub fn load_sample_data(&mut self) {
        use crate::core::collection::{CollectionItem, Folder, Request};
        use std::collections::HashMap;

        let mut collection = Collection::new("Test Collection".to_string());
        
        let mut folder = Folder::new("Test Folder".to_string());
        folder.expanded = true;

        let req1 = Request {
            id: uuid::Uuid::new_v4().to_string(),
            name: "GET TestAPI".to_string(),
            method: Method::Get,
            url: "https://httpbin.org/get".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let req2 = Request {
            id: uuid::Uuid::new_v4().to_string(),
            name: "POST TestAPI2".to_string(),
            method: Method::Post,
            url: "https://httpbin.org/post".to_string(),
            headers: HashMap::new(),
            body: Some("{\"hello\":\"world\"}".to_string()),
        };

        folder.items.push(CollectionItem::Request(req1));
        folder.items.push(CollectionItem::Request(req2));

        collection.items.push(CollectionItem::Folder(folder));
        
        self.collections.push(collection);
    }
}
