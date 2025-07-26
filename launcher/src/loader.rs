use std::fs;
use std::path::Path;
use crate::models::{App, LauncherState};

pub fn load_apps(state: &mut LauncherState) {
    load_installed_apps(state);
    add_builtin_apps(state);
    update_app_lists(state);
}

fn load_installed_apps(state: &mut LauncherState) {
    if let Ok(entries) = fs::read_dir(&state.apps_directory) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(app) = load_app_from_directory(&entry.path()) {
                    state.apps.insert(app.id.clone(), app);
                }
            }
        }
    }
}

fn add_builtin_apps(state: &mut LauncherState) {
    let builtin_apps = vec![
            App {
                id: "movies".to_string(),
                name: "Movies & TV".to_string(),
                description: "Movie library".to_string(),
                icon: Some("icons/movies.svg".to_string()),
                executable_path: "builtin://movies".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Entertainment".to_string(),
            },
            App {
                id: "music".to_string(),
                name: "Music".to_string(),
                description: "Music streaming".to_string(),
                icon: Some("icons/music.svg".to_string()),
                executable_path: "builtin://music".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Entertainment".to_string(),
            },
            App {
                id: "photos".to_string(),
                name: "Photos".to_string(),
                description: "Photo viewer".to_string(),
                icon: Some("icons/photos.svg".to_string()),
                executable_path: "builtin://photos".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "Media".to_string(),
            },
            App {
                id: "settings".to_string(),
                name: "Settings".to_string(),
                description: "System settings".to_string(),
                icon: Some("icons/settings.svg".to_string()),
                executable_path: "builtin://settings".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "System".to_string(),
            },
            App {
                id: "app_store".to_string(),
                name: "App Store".to_string(),
                description: "Download apps".to_string(),
                icon: Some("icons/app_store.svg".to_string()),
                executable_path: "builtin://app_store".to_string(),
                installed: true,
                version: "1.0.0".to_string(),
                category: "System".to_string(),
            },
    ];
    for app in builtin_apps {
        state.apps.insert(app.id.clone(), app);
    }
}

fn load_app_from_directory(path: &Path) -> Option<App> {
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

pub fn update_app_lists(state: &mut LauncherState) {
    state.installed_apps = state.apps.values().filter(|a| a.installed).cloned().collect();
    state.store_apps = state.apps.values().filter(|a| !a.installed).cloned().collect();
    state.featured_apps = state.installed_apps.iter().take(6).cloned().collect();
}
