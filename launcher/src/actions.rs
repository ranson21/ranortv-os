use std::process::Command;
use crate::models::{App, LauncherState};

pub fn launch_app(state: &LauncherState, app_id: &str) {
    if let Some(app) = state.apps.get(app_id) {
        println!("🚀 Launching app: {}", app.name);

        if app.executable_path.starts_with("builtin://") {
            handle_builtin_app(&app.executable_path);
        } else {
            launch_sandboxed_app(app);
        }
    }
}

fn handle_builtin_app(path: &str) {
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

fn launch_sandboxed_app(app: &App) {
    let mut cmd = Command::new("unshare");
    cmd.args(&["--net", "--pid", "--fork"])
        .arg(&app.executable_path);

    match cmd.spawn() {
        Ok(child) => println!("✅ Launched {} (PID: {:?})", app.name, child.id()),
        Err(e) => eprintln!("❌ Failed to launch {}: {}", app.name, e),
    }
}
