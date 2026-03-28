use crate::cli::args::Method;
use crate::core::collection::{Collection, CollectionItem};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
    Command,
    Rename,
    Search,
    ConfirmDelete,
    ConfirmQuit,
    CreateItem,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PendingItemType {
    Collection,
    Folder,
    Request,
}

pub struct App {
    pub input_mode: InputMode,
    pub current_layer: UiLayer,
    pub focused_panel: FocusedPanel,
    pub active_request_part: RequestBarPart,
    pub show_method_search: bool,
    pub method_search_query: String,
    pub show_search: bool,
    pub search_query: String,
    pub last_focused_in_layer1: FocusedPanel,
    pub last_focused_layer: UiLayer,
    pub url: String,
    pub method: Method,
    pub command_input: String,
    pub should_quit: bool,
    pub collections: Vec<Collection>,
    pub current_request_id: Option<String>,
    pub active_collection_index: usize, // The collection whose items are shown in APIs panel
    pub active_folder_id: Option<String>, // The folder whose items are shown in APIs panel
    pub selected_collection_index: usize, // Index in the flattened visible collections tree
    pub selected_api_index: usize, // Index in the flattened visible items list
    pub rename_input: String,
    pub pending_item_type: Option<PendingItemType>,
    pub error_message: Option<String>,
}

pub struct VisibleItem {
    pub name: String,
    pub depth: usize,
    pub item_type: VisibleItemType,
}

impl VisibleItem {
    pub fn item_type_depth(&self) -> usize {
        self.depth
    }
}

pub enum VisibleItemType {
    Collection { expanded: bool },
    Folder { expanded: bool },
    Request { method: Method, id: String },
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
            show_search: false,
            search_query: String::new(),
            last_focused_in_layer1: FocusedPanel::Collections,
            last_focused_layer: UiLayer::Layer1,
            url: "https://httpbin.org/get".to_string(),
            method: Method::Get,
            command_input: String::new(),
            should_quit: false,
            collections: Vec::new(),
            current_request_id: None,
            active_collection_index: 0,
            active_folder_id: None,
            selected_collection_index: 0,
            selected_api_index: 0,
            rename_input: String::new(),
            pending_item_type: None,
            error_message: None,
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
                if self.focused_panel == FocusedPanel::Collections {
                    self.focused_panel = FocusedPanel::Apis;
                    self.last_focused_in_layer1 = FocusedPanel::Apis;
                    self.selected_api_index = 0;
                } else {
                    self.focus_request_bar();
                }
            }
            UiLayer::Layer2 => {
                self.current_layer = UiLayer::Layer3;
                self.focused_panel = FocusedPanel::Details;
            }
            _ => {}
        }
    }

    pub fn update_active_scope_from_tree(&mut self) {
        let visible_collections = self.get_visible_collections();
        if let Some(_item) = visible_collections.get(self.selected_collection_index) {
            let mut current_idx = 0;
            for (i, col) in self.collections.iter().enumerate() {
                if current_idx == self.selected_collection_index {
                    self.active_collection_index = i;
                    self.active_folder_id = None;
                    return;
                }
                
                if col.expanded {
                    let mut found_id = None;
                    let mut sub_idx = current_idx + 1;
                    if self.find_container_id_at_index(&col.items, &mut sub_idx, self.selected_collection_index, &mut found_id) {
                        self.active_collection_index = i;
                        self.active_folder_id = found_id;
                        return;
                    }
                    current_idx += 1 + Self::count_visible_items_recursive(&col.items);
                } else {
                    current_idx += 1;
                }
            }
        }
    }

    fn find_container_id_at_index(&self, items: &[CollectionItem], current_idx: &mut usize, target_idx: usize, found_id: &mut Option<String>) -> bool {
        for item in items {
            if *current_idx == target_idx {
                match item {
                    CollectionItem::Folder(f) => *found_id = Some(f.id.clone()),
                    CollectionItem::Request(_) => {
                        // Keep previous container or parent folder id would be better but let's stay on current
                    }
                }
                return true;
            }
            *current_idx += 1;
            if let CollectionItem::Folder(f) = item {
                if f.expanded {
                    let prev_found = found_id.clone();
                    *found_id = Some(f.id.clone());
                    if self.find_container_id_at_index(&f.items, current_idx, target_idx, found_id) {
                        return true;
                    }
                    *found_id = prev_found;
                }
            }
        }
        false
    }

    pub fn pop_up(&mut self) {
        if self.show_method_search {
            self.show_method_search = false;
            self.method_search_query.clear();
            return;
        }

        if self.error_message.is_some() {
            self.error_message = None;
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
                self.current_layer = self.last_focused_layer;
                self.focused_panel = match self.current_layer {
                    UiLayer::Layer1 => self.last_focused_in_layer1,
                    UiLayer::Layer2 => FocusedPanel::Properties,
                    UiLayer::LayerRequestBar => FocusedPanel::RequestBar,
                    _ => FocusedPanel::Collections,
                };
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
                if self.active_request_part == RequestBarPart::Url {
                    self.input_mode = InputMode::Editing;
                } else {
                    self.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        }
    }

    pub fn get_visible_collections(&self) -> Vec<VisibleItem> {
        let mut visible_items = Vec::new();
        for col in &self.collections {
            visible_items.push(VisibleItem {
                name: col.name.clone(),
                depth: 0,
                item_type: VisibleItemType::Collection { expanded: col.expanded },
            });
            if col.expanded {
                for item in &col.items {
                    Self::collect_visible_items_recursive(item, 1, &mut visible_items);
                }
            }
        }
        visible_items
    }

    pub fn get_visible_items(&self) -> Vec<VisibleItem> {
        let mut visible_items = Vec::new();
        if let Some(col) = self.collections.get(self.active_collection_index) {
            let items_to_flatten = if let Some(folder_id) = &self.active_folder_id {
                Self::find_folder_items(&col.items, folder_id).unwrap_or(&col.items)
            } else {
                &col.items
            };

            if self.show_search && !self.search_query.is_empty() {
                for item in items_to_flatten {
                    self.collect_search_results_recursive(item, 0, &mut visible_items);
                }
            } else {
                for item in items_to_flatten {
                    Self::collect_visible_items_recursive(item, 0, &mut visible_items);
                }
            }
        }
        visible_items
    }

    fn find_folder_items<'a>(items: &'a [CollectionItem], folder_id: &str) -> Option<&'a Vec<CollectionItem>> {
        for item in items {
            if let CollectionItem::Folder(f) = item {
                if f.id == folder_id {
                    return Some(&f.items);
                }
                if let Some(found) = Self::find_folder_items(&f.items, folder_id) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn collect_visible_items_recursive(item: &CollectionItem, depth: usize, visible_items: &mut Vec<VisibleItem>) {
        match item {
            CollectionItem::Folder(f) => {
                visible_items.push(VisibleItem {
                    name: f.name.clone(),
                    depth,
                    item_type: VisibleItemType::Folder { expanded: f.expanded },
                });
                if f.expanded {
                    for sub_item in &f.items {
                        Self::collect_visible_items_recursive(sub_item, depth + 1, visible_items);
                    }
                }
            }
            CollectionItem::Request(r) => {
                visible_items.push(VisibleItem {
                    name: r.name.clone(),
                    depth,
                    item_type: VisibleItemType::Request { method: r.method, id: r.id.clone() },
                });
            }
        }
    }

    fn collect_search_results_recursive(&self, item: &CollectionItem, depth: usize, visible_items: &mut Vec<VisibleItem>) -> bool {
        let name = match item {
            CollectionItem::Folder(f) => &f.name,
            CollectionItem::Request(r) => &r.name,
        };

        let matches_self = name.to_lowercase().contains(&self.search_query.to_lowercase());
        let mut child_matches = false;

        let mut sub_results = Vec::new();
        if let CollectionItem::Folder(f) = item {
            for sub_item in &f.items {
                if self.collect_search_results_recursive(sub_item, depth + 1, &mut sub_results) {
                    child_matches = true;
                }
            }
        }

        if matches_self || child_matches {
            match item {
                CollectionItem::Folder(f) => {
                    visible_items.push(VisibleItem {
                        name: f.name.clone(),
                        depth,
                        item_type: VisibleItemType::Folder { expanded: true },
                    });
                }
                CollectionItem::Request(r) => {
                    visible_items.push(VisibleItem {
                        name: r.name.clone(),
                        depth,
                        item_type: VisibleItemType::Request { method: r.method, id: r.id.clone() },
                    });
                }
            }
            visible_items.extend(sub_results);
            return true;
        }

        false
    }

    pub fn toggle_folder(&mut self) {
        if self.focused_panel == FocusedPanel::Collections {
            let visible_items = self.get_visible_collections();
            if let Some(item) = visible_items.get(self.selected_collection_index) {
                match item.item_type {
                    VisibleItemType::Collection { .. } => {
                        let mut current_idx = 0;
                        for col in &mut self.collections {
                            if current_idx == self.selected_collection_index {
                                col.expanded = !col.expanded;
                                return;
                            }
                            current_idx += 1;
                            if col.expanded {
                                current_idx += Self::count_visible_items_recursive(&col.items);
                            }
                        }
                    }
                    VisibleItemType::Folder { .. } => {
                        let target_name = item.name.clone();
                        let target_depth = item.depth;
                        let mut current_idx = 0;
                        let selected_collection_index = self.selected_collection_index;
                        
                        for col in &mut self.collections {
                            current_idx += 1;
                            if col.expanded {
                                for it in &mut col.items {
                                    if Self::find_and_toggle_folder_recursive(it, 1, target_depth, &target_name, &mut current_idx, selected_collection_index) {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            return;
        }

        let visible_items = self.get_visible_items();
        if let Some(item) = visible_items.get(self.selected_api_index) {
            if let VisibleItemType::Folder { .. } = item.item_type {
                let target_name = item.name.clone();
                let target_depth = item.depth;
                let mut current_idx = 0;
                let selected_api_index = self.selected_api_index;
                
                if let Some(col) = self.collections.get_mut(self.active_collection_index) {
                    for it in &mut col.items {
                        if Self::find_and_toggle_folder_recursive(it, 0, target_depth, &target_name, &mut current_idx, selected_api_index) {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn count_visible_items_recursive(items: &[CollectionItem]) -> usize {
        let mut count = 0;
        for item in items {
            count += 1;
            if let CollectionItem::Folder(f) = item {
                if f.expanded {
                    count += Self::count_visible_items_recursive(&f.items);
                }
            }
        }
        count
    }

    fn find_and_toggle_folder_recursive(item: &mut CollectionItem, current_depth: usize, target_depth: usize, target_name: &str, current_idx: &mut usize, selected_api_index: usize) -> bool {
        match item {
            CollectionItem::Folder(f) => {
                if current_depth == target_depth && f.name == target_name && *current_idx == selected_api_index {
                    f.expanded = !f.expanded;
                    return true;
                }
                *current_idx += 1;
                if f.expanded {
                    for sub_item in &mut f.items {
                        if Self::find_and_toggle_folder_recursive(sub_item, current_depth + 1, target_depth, target_name, current_idx, selected_api_index) {
                            return true;
                        }
                    }
                }
            }
            CollectionItem::Request(_) => {
                *current_idx += 1;
            }
        }
        false
    }

    pub fn rename_selected_item(&mut self) {
        if self.focused_panel == FocusedPanel::Collections {
            let visible_items = self.get_visible_collections();
            if let Some(item) = visible_items.get(self.selected_collection_index) {
                match &item.item_type {
                    VisibleItemType::Collection { .. } => {
                        let mut current_idx = 0;
                        for col in &mut self.collections {
                            if current_idx == self.selected_collection_index {
                                col.name = self.rename_input.clone();
                                return;
                            }
                            current_idx += 1;
                            if col.expanded {
                                current_idx += Self::count_visible_items_recursive(&col.items);
                            }
                        }
                    }
                    VisibleItemType::Folder { .. } | VisibleItemType::Request { .. } => {
                        let target_name = item.name.clone();
                        let target_depth = item.depth;
                        let mut current_idx = 0;
                        let selected_tree_index = self.selected_collection_index;
                        let new_name = self.rename_input.clone();
                        
                        for col in &mut self.collections {
                            current_idx += 1;
                            if col.expanded {
                                for it in &mut col.items {
                                    if Self::find_and_rename_item_recursive(it, 1, target_depth, &target_name, &mut current_idx, selected_tree_index, &new_name) {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if self.focused_panel == FocusedPanel::Apis {
            let visible_items = self.get_visible_items();
            if let Some(item) = visible_items.get(self.selected_api_index) {
                let target_name = item.name.clone();
                let target_depth = item.depth;
                let selected_api_index = self.selected_api_index;
                let new_name = self.rename_input.clone();
                
                if let Some(col) = self.collections.get_mut(self.active_collection_index) {
                    let mut current_idx = 0;
                    for it in &mut col.items {
                        if Self::find_and_rename_item_recursive(it, 0, target_depth, &target_name, &mut current_idx, selected_api_index, &new_name) {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn find_and_rename_item_recursive(item: &mut CollectionItem, current_depth: usize, target_depth: usize, target_name: &str, current_idx: &mut usize, selected_api_index: usize, new_name: &str) -> bool {
        match item {
            CollectionItem::Folder(f) => {
                if current_depth == target_depth && f.name == target_name && *current_idx == selected_api_index {
                    f.name = new_name.to_string();
                    return true;
                }
                let was_expanded = f.expanded;
                *current_idx += 1;
                if was_expanded {
                    for sub_item in &mut f.items {
                        if Self::find_and_rename_item_recursive(sub_item, current_depth + 1, target_depth, target_name, current_idx, selected_api_index, new_name) {
                            return true;
                        }
                    }
                }
            }
            CollectionItem::Request(r) => {
                if current_depth == target_depth && r.name == target_name && *current_idx == selected_api_index {
                    r.name = new_name.to_string();
                    return true;
                }
                *current_idx += 1;
            }
        }
        false
    }

    pub fn add_collection(&mut self, name: String) {
        let name = if name.is_empty() { "New Collection".to_string() } else { name };
        self.collections.push(Collection::new(name));
    }

    pub fn add_request(&mut self, name: String) {
        let name = if name.is_empty() { "New Request".to_string() } else { name };
        
        if self.focused_panel == FocusedPanel::Collections {
            let visible_items = self.get_visible_collections();
            if let Some(selected_item) = visible_items.get(self.selected_collection_index) {
                let target_name = selected_item.name.clone();
                let target_depth = selected_item.depth;
                let selected_tree_index = self.selected_collection_index;
                
                let new_req = crate::core::collection::Request {
                    id: uuid::Uuid::new_v4().to_string(),
                    name,
                    method: Method::Get,
                    url: "https://".to_string(),
                    headers: std::collections::HashMap::new(),
                    body: None,
                };

                let mut current_idx = 0;
                for col in &mut self.collections {
                    if current_idx == selected_tree_index {
                        col.items.push(CollectionItem::Request(new_req));
                        col.expanded = true;
                        return;
                    }
                    current_idx += 1;
                    if col.expanded {
                        for item in &mut col.items {
                            if Self::do_find_and_add_recursive(item, 1, target_depth, &target_name, &mut current_idx, selected_tree_index, CollectionItem::Request(new_req.clone())) {
                                return;
                            }
                        }
                    }
                }
            }
            return;
        }

        let visible_items = self.get_visible_items();
        let selected_api_index = self.selected_api_index;
        
        let mut target_folder_info = None;
        if let Some(selected_item) = visible_items.get(selected_api_index) {
            if let VisibleItemType::Folder { .. } = selected_item.item_type {
                target_folder_info = Some((selected_item.name.clone(), selected_item.depth));
            }
        }

        if let Some(col) = self.collections.get_mut(self.active_collection_index) {
            let new_req = crate::core::collection::Request {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                method: Method::Get,
                url: "https://".to_string(),
                headers: std::collections::HashMap::new(),
                body: None,
            };
            
            if let Some((target_name, target_depth)) = target_folder_info {
                let mut current_idx = 0;
                for item in &mut col.items {
                    if Self::do_find_and_add_recursive(item, 0, target_depth, &target_name, &mut current_idx, selected_api_index, CollectionItem::Request(new_req.clone())) {
                        return;
                    }
                }
            }
            
            col.items.push(CollectionItem::Request(new_req));
        }
    }

    pub fn add_folder(&mut self, name: String) {
        let name = if name.is_empty() { "New Folder".to_string() } else { name };

        if self.focused_panel == FocusedPanel::Collections {
            let visible_items = self.get_visible_collections();
            if let Some(selected_item) = visible_items.get(self.selected_collection_index) {
                let target_name = selected_item.name.clone();
                let target_depth = selected_item.depth;
                let selected_tree_index = self.selected_collection_index;
                
                let new_folder = crate::core::collection::Folder::new(name);

                let mut current_idx = 0;
                for col in &mut self.collections {
                    if current_idx == selected_tree_index {
                        col.items.push(CollectionItem::Folder(new_folder));
                        col.expanded = true;
                        return;
                    }
                    current_idx += 1;
                    if col.expanded {
                        for item in &mut col.items {
                            if Self::do_find_and_add_recursive(item, 1, target_depth, &target_name, &mut current_idx, selected_tree_index, CollectionItem::Folder(new_folder.clone())) {
                                return;
                            }
                        }
                    }
                }
            }
            return;
        }

        let visible_items = self.get_visible_items();
        let selected_api_index = self.selected_api_index;
        
        let mut target_folder_info = None;
        if let Some(selected_item) = visible_items.get(selected_api_index) {
            if let VisibleItemType::Folder { .. } = selected_item.item_type {
                target_folder_info = Some((selected_item.name.clone(), selected_item.depth));
            }
        }

        if let Some(col) = self.collections.get_mut(self.active_collection_index) {
            let new_folder = crate::core::collection::Folder::new(name);
            
            if let Some((target_name, target_depth)) = target_folder_info {
                let mut current_idx = 0;
                for item in &mut col.items {
                    if Self::do_find_and_add_recursive(item, 0, target_depth, &target_name, &mut current_idx, selected_api_index, CollectionItem::Folder(new_folder.clone())) {
                        return;
                    }
                }
            }
            
            col.items.push(CollectionItem::Folder(new_folder));
        }
    }

    fn do_find_and_add_recursive(item: &mut CollectionItem, current_depth: usize, target_depth: usize, target_name: &str, current_idx: &mut usize, selected_api_index: usize, new_item: CollectionItem) -> bool {
        match item {
            CollectionItem::Folder(f) => {
                if current_depth == target_depth && f.name == target_name && *current_idx == selected_api_index {
                    f.items.push(new_item);
                    f.expanded = true;
                    return true;
                }
                let was_expanded = f.expanded;
                *current_idx += 1;
                if was_expanded {
                    for sub_item in &mut f.items {
                        if Self::do_find_and_add_recursive(sub_item, current_depth + 1, target_depth, target_name, current_idx, selected_api_index, new_item.clone()) {
                            return true;
                        }
                    }
                }
            }
            CollectionItem::Request(_) => {
                *current_idx += 1;
            }
        }
        false
    }

    pub fn delete_selected_item(&mut self) {
        if self.focused_panel == FocusedPanel::Collections {
            let visible_items = self.get_visible_collections();
            if let Some(item) = visible_items.get(self.selected_collection_index) {
                match &item.item_type {
                    VisibleItemType::Collection { .. } => {
                        let mut current_idx = 0;
                        let mut to_remove = None;
                        for (i, _) in self.collections.iter().enumerate() {
                            if current_idx == self.selected_collection_index {
                                to_remove = Some(i);
                                break;
                            }
                            current_idx += 1;
                            if self.collections[i].expanded {
                                current_idx += Self::count_visible_items_recursive(&self.collections[i].items);
                            }
                        }
                        if let Some(i) = to_remove {
                            self.collections.remove(i);
                            self.selected_collection_index = self.selected_collection_index.saturating_sub(1);
                        }
                    }
                    VisibleItemType::Folder { .. } | VisibleItemType::Request { .. } => {
                        let target_name = item.name.clone();
                        let target_depth = item.depth;
                        let selected_tree_index = self.selected_collection_index;
                        
                        let mut current_idx = 0;
                        for col in &mut self.collections {
                            current_idx += 1;
                            if col.expanded {
                                let mut to_remove = None;
                                for (i, it) in col.items.iter_mut().enumerate() {
                                    if Self::do_find_and_delete_recursive(it, 1, target_depth, &target_name, &mut current_idx, selected_tree_index) {
                                        to_remove = Some(i);
                                        break;
                                    }
                                }
                                if let Some(i) = to_remove {
                                    col.items.remove(i);
                                    self.selected_collection_index = self.selected_collection_index.saturating_sub(1);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        } else if self.focused_panel == FocusedPanel::Apis {
            let visible_items = self.get_visible_items();
            let selected_api_index = self.selected_api_index;
            
            if let Some(item) = visible_items.get(selected_api_index) {
                let target_name = item.name.clone();
                let target_depth = item.depth;
                
                if let Some(col) = self.collections.get_mut(self.active_collection_index) {
                    let mut current_idx = 0;
                    let mut to_remove = None;
                    for (i, it) in col.items.iter_mut().enumerate() {
                        if Self::do_find_and_delete_recursive(it, 0, target_depth, &target_name, &mut current_idx, selected_api_index) {
                            to_remove = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = to_remove {
                        col.items.remove(i);
                        self.selected_api_index = self.selected_api_index.saturating_sub(1);
                    }
                }
            }
        }
    }

    fn do_find_and_delete_recursive(item: &mut CollectionItem, current_depth: usize, target_depth: usize, target_name: &str, current_idx: &mut usize, selected_api_index: usize) -> bool {
        match item {
            CollectionItem::Folder(f) => {
                if current_depth == target_depth && f.name == target_name && *current_idx == selected_api_index {
                    return true;
                }
                let was_expanded = f.expanded;
                *current_idx += 1;
                if was_expanded {
                    let mut to_remove = None;
                    for (i, sub_item) in f.items.iter_mut().enumerate() {
                        if Self::do_find_and_delete_recursive(sub_item, current_depth + 1, target_depth, target_name, current_idx, selected_api_index) {
                            to_remove = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = to_remove {
                        f.items.remove(i);
                        return false; 
                    }
                }
            }
            CollectionItem::Request(r) => {
                if current_depth == target_depth && r.name == target_name && *current_idx == selected_api_index {
                    return true;
                }
                *current_idx += 1;
            }
        }
        false
    }

    pub fn save_current_request(&mut self) {
        if let Some(req_id) = &self.current_request_id {
            let req_id = req_id.clone();
            let url = self.url.clone();
            let method = self.method;
            if let Some(col) = self.collections.get_mut(self.active_collection_index) {
                if let Some(req) = col.find_request_mut(&req_id) {
                    req.url = url;
                    req.method = method;
                }
            }
        }
    }

    pub fn save_collections(&mut self) {
        let persistence = crate::core::persistence::PersistenceManager::new();
        if let Err(e) = persistence.save_collections(&self.collections) {
            self.error_message = Some(format!("Save failed: {}", e));
        }
    }

    pub fn import_collection(&mut self, path: &str) {
        match crate::core::import::import_collection(path) {
            Ok(col) => {
                self.collections.push(col);
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Import failed: {}", e));
            }
        }
    }

    pub fn load_sample_data(&mut self) {
        use crate::core::collection::{Folder, Request};
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

        let req3 = Request {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Root Request".to_string(),
            method: Method::Get,
            url: "https://httpbin.org/get".to_string(),
            headers: HashMap::new(),
            body: None,
        };
        collection.items.push(CollectionItem::Request(req3));
        
        self.collections.push(collection);
    }
}
