use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_path: Option<String>,
    pub executable_path: String,
    pub installed: bool,
    pub version: String,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct LauncherState {
    pub apps: HashMap<String, App>,
    pub featured_apps: Vec<App>,
    pub installed_apps: Vec<App>,
    pub store_apps: Vec<App>,
    pub apps_directory: String,
}

impl Default for LauncherState {
    fn default() -> Self {
        let mut state = Self {
            apps: HashMap::new(),
            featured_apps: Vec::new(),
            installed_apps: Vec::new(),
            store_apps: Vec::new(),
            apps_directory: "/apps".to_string(),
        };
        crate::loader::load_apps(&mut state);
        state
    }
}
