use slint::{ModelRc, VecModel, Image};
use std::path::Path;
use crate::models::App;
slint::include_modules!();

pub fn apps_to_slint_model(apps: &[App]) -> ModelRc<AppItem> {
    let items: Vec<AppItem> = apps
        .iter()
        .map(|app| AppItem {
            id: app.id.clone().into(),
            name: app.name.clone().into(),
            description: app.description.clone().into(),
            icon: load_icon(&app.icon.clone().unwrap_or_default()),
            category: app.category.clone().into(),
        })
        .collect();
    ModelRc::new(VecModel::from(items))
}

fn load_icon(icon_path: &str) -> Image {
    if icon_path.is_empty() {
        return load_default_icon();
    }

    // Try multiple possible locations
    let mut search_paths = vec![
        icon_path.to_string(),                           // Exact path as given
        format!("assets/{}", icon_path),                 // Relative to assets
        format!("icons/{}", icon_path),                  // Relative to icons folder
    ];

    // Add user icons directory if we can get it
    if let Ok(user_icons) = get_user_icons_dir() {
        search_paths.push(user_icons.join(icon_path).to_string_lossy().to_string());
    }

    for path in &search_paths {
        if let Ok(image) = load_image_from_path(path) {
            return image;
        }
    }

    // If nothing found, return default
    eprintln!("Could not load icon: {}", icon_path);
    load_default_icon()
}

fn load_image_from_path(path: &str) -> Result<Image, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err("File does not exist".into());
    }

    if path.ends_with(".svg") {
        let svg_data = std::fs::read(path)?;
        Ok(Image::load_from_svg_data(&svg_data)?)
    } else {
        Ok(Image::load_from_path(Path::new(path))?)
    }
}

fn get_user_icons_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Use platform-specific app data directory
    let app_data = if cfg!(target_os = "windows") {
        std::env::var("APPDATA")?
    } else if cfg!(target_os = "macos") {
        format!("{}/Library/Application Support", std::env::var("HOME")?)
    } else {
        format!("{}/.local/share", std::env::var("HOME")?)
    };
    
    Ok(Path::new(&app_data).join("YourAppName").join("icons"))
}

fn load_default_icon() -> Image {
    // Try to load a default icon, fallback to empty if not found
    load_image_from_path("assets/icons/default.svg")
        .or_else(|_| load_image_from_path("icons/default.svg"))
        .unwrap_or_else(|_| {
            eprintln!("Warning: Could not load default icon");
            Image::default()
        })
}