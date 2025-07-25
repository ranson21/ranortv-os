// Apple TV-style Media OS Launcher using Slint
use slint::{ComponentHandle, ModelRc, VecModel};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

slint::include_modules!();

#[derive(Debug, Clone, Serialize, Deserialize)]
struct App {
    id: String,
    name: String,
    description: String,
    icon_path: Option<String>,
    executable_path: String,
    installed: bool,
    version: String,
    category: String,
}

#[derive(Debug, Clone)]
struct LauncherState {
    apps: HashMap<String, App>,
    featured_apps: Vec<App>,
    installed_apps: Vec<App>,
    store_apps: Vec<App>,
    apps_directory: String,
    // Removed unused current_focus field
}

impl Default for LauncherState {
    fn default() -> Self {
        let mut state = Self {
            apps: HashMap::new(),
            featured_apps: Vec::new(),
            installed_apps: Vec::new(),
            store_apps: Vec::new(),
            apps_directory: String::from("/apps"),
        };
        state.load_apps();
        state
    }
}

impl LauncherState {
    fn load_apps(&mut self) {
        self.load_installed_apps();
        self.add_builtin_apps();
        self.update_app_lists();
    }

    fn load_installed_apps(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.apps_directory) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(app) = self.load_app_from_directory(&entry.path()) {
                        self.apps.insert(app.id.clone(), app);
                    }
                }
            }
        }
    }

    fn add_builtin_apps(&mut self) {
        let builtin_apps = vec![
            App {
                id: "tv".to_string(),
                name: "TV".to_string(),
                description: "Live TV and streaming".to_string(),
                icon_path: Some("icons/tv.png".to_string()),
                executable_path: "builtin://tv".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Entertainment".to_string(),
            },
            App {
                id: "movies".to_string(),
                name: "Movies".to_string(),
                description: "Movie library".to_string(),
                icon_path: Some("icons/movies.png".to_string()),
                executable_path: "builtin://movies".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Entertainment".to_string(),
            },
            App {
                id: "music".to_string(),
                name: "Music".to_string(),
                description: "Music streaming".to_string(),
                icon_path: Some("icons/music.png".to_string()),
                executable_path: "builtin://music".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Entertainment".to_string(),
            },
            App {
                id: "photos".to_string(),
                name: "Photos".to_string(),
                description: "Photo viewer".to_string(),
                icon_path: Some("icons/photos.png".to_string()),
                executable_path: "builtin://photos".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Media".to_string(),
            },
            App {
                id: "settings".to_string(),
                name: "Settings".to_string(),
                description: "System settings".to_string(),
                icon_path: Some("icons/settings.png".to_string()),
                executable_path: "builtin://settings".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "System".to_string(),
            },
            App {
                id: "app_store".to_string(),
                name: "App Store".to_string(),
                description: "Download apps".to_string(),
                icon_path: Some("icons/app_store.png".to_string()),
                executable_path: "builtin://app_store".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "System".to_string(),
            },
        ];

        for app in builtin_apps {
            self.apps.insert(app.id.clone(), app);
        }
    }

    fn load_app_from_directory(&self, path: &Path) -> Option<App> {
        let metadata_path = path.join("app.json");
        if metadata_path.exists() {
            if let Ok(content) = fs::read_to_string(metadata_path) {
                if let Ok(app) = serde_json::from_str::<App>(&content) {
                    return Some(app);
                }
            }
        }
        None
    }

    fn update_app_lists(&mut self) {
        self.installed_apps = self.apps
            .values()
            .filter(|app| app.installed)
            .cloned()
            .collect();

        self.store_apps = self.apps
            .values()
            .filter(|app| !app.installed)
            .cloned()
            .collect();

        // Featured apps are the first 6 installed apps
        self.featured_apps = self.installed_apps
            .iter()
            .take(6)
            .cloned()
            .collect();
    }

    fn launch_app(&self, app_id: &str) {
        if let Some(app) = self.apps.get(app_id) {
            println!("🚀 Launching app: {}", app.name);
            
            if app.executable_path.starts_with("builtin://") {
                self.handle_builtin_app(&app.executable_path);
            } else {
                self.launch_sandboxed_app(app);
            }
        }
    }

    fn handle_builtin_app(&self, path: &str) {
        match path {
            "builtin://tv" => {
                println!("📺 Opening TV app...");
                // TODO: Implement TV interface
            }
            "builtin://movies" => {
                println!("🎬 Opening Movies app...");
                // TODO: Implement movie library
            }
            "builtin://music" => {
                println!("🎵 Opening Music app...");
                // TODO: Implement music player
            }
            "builtin://photos" => {
                println!("📸 Opening Photos app...");
                // TODO: Implement photo viewer
            }
            "builtin://settings" => {
                println!("⚙️ Opening Settings...");
                // TODO: Implement settings interface
            }
            "builtin://app_store" => {
                println!("🏪 Opening App Store...");
                // TODO: Implement app store
            }
            _ => {
                println!("❓ Unknown builtin app: {}", path);
            }
        }
    }

    fn launch_sandboxed_app(&self, app: &App) {
        let mut cmd = Command::new("unshare");
        cmd.args(&["--net", "--pid", "--fork"])
           .arg(&app.executable_path);
        
        match cmd.spawn() {
            Ok(child) => {
                println!("✅ Launched {} (PID: {:?})", app.name, child.id());
            }
            Err(e) => {
                eprintln!("❌ Failed to launch {}: {}", app.name, e);
            }
        }
    }
}

// Convert our apps to Slint's AppItem format
fn apps_to_slint_model(apps: &[App]) -> ModelRc<AppItem> {
    let items: Vec<AppItem> = apps
        .iter()
        .map(|app| AppItem {
            id: app.id.clone().into(),
            name: app.name.clone().into(),
            description: app.description.clone().into(),
            icon_path: app.icon_path.clone().unwrap_or_default().into(),
            category: app.category.clone().into(),
        })
        .collect();
    
    ModelRc::new(VecModel::from(items))
}

fn main() -> Result<(), slint::PlatformError> {
    // Initialize Slint platform
    println!("🎬 Starting RanorTV Launcher...");
    
    let ui = AppWindow::new()?;
    let state = Rc::new(std::cell::RefCell::new(LauncherState::default()));

    // Initialize the UI with app data
    {
        let state_ref = state.borrow();
        ui.set_featured_apps(apps_to_slint_model(&state_ref.featured_apps));
        ui.set_installed_apps(apps_to_slint_model(&state_ref.installed_apps));
        ui.set_store_apps(apps_to_slint_model(&state_ref.store_apps));
        
        println!("📱 Loaded {} installed apps", state_ref.installed_apps.len());
        println!("🌟 Featured {} apps", state_ref.featured_apps.len());
    }

    // Handle app launches
    {
        let state_clone = state.clone();
        ui.on_launch_app(move |app_id| {
            let state_ref = state_clone.borrow();
            state_ref.launch_app(&app_id);
        });
    }

    // Handle navigation
    {
        ui.on_navigate(move |direction| {
            match direction.as_str() {
                "up" => {
                    println!("⬆️ Navigate up");
                    // TODO: Implement focus management
                }
                "down" => {
                    println!("⬇️ Navigate down");
                    // TODO: Implement focus management
                }
                "left" => {
                    println!("⬅️ Navigate left");
                    // TODO: Implement focus management
                }
                "right" => {
                    println!("➡️ Navigate right");
                    // TODO: Implement focus management
                }
                "select" => {
                    println!("✅ Select current item");
                    // TODO: Launch focused app
                }
                _ => {}
            }
        });
    }

    // Handle tab switching
    {
        ui.on_switch_tab(move |tab| {
            println!("📑 Switched to tab: {}", tab);
            // Tab switching is handled by the UI automatically
        });
    }

    // Handle app store refresh
    {
        let state_clone = state.clone();
        let ui_weak = ui.as_weak();
        ui.on_refresh_store(move || {
            println!("🔄 Refreshing app store...");
            
            // Mock adding new apps
            let mut state_ref = state_clone.borrow_mut();
            
            // Add some mock store apps
            let mock_apps = vec![
                App {
                    id: "spotify".to_string(),
                    name: "Spotify".to_string(),
                    description: "Music streaming service".to_string(),
                    icon_path: None,
                    executable_path: "/apps/spotify/spotify".to_string(),
                    installed: false,
                    version: "1.0.0".to_string(),
                    category: "Music".to_string(),
                },
                App {
                    id: "youtube".to_string(),
                    name: "YouTube".to_string(),
                    description: "Video streaming platform".to_string(),
                    icon_path: None,
                    executable_path: "/apps/youtube/youtube".to_string(),
                    installed: false,
                    version: "2.1.0".to_string(),
                    category: "Entertainment".to_string(),
                },
            ];
            
            for app in mock_apps {
                state_ref.apps.insert(app.id.clone(), app);
            }
            
            state_ref.update_app_lists();
            
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_store_apps(apps_to_slint_model(&state_ref.store_apps));
                println!("📦 Added {} apps to store", state_ref.store_apps.len());
            }
        });
    }

    
    println!("🎯 RanorTV Launcher ready!");
    println!("🎮 Use arrow keys to navigate, Enter to select");
    println!("🖱️ Click on apps to launch them");
    
    ui.run()
}