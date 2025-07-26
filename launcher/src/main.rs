mod models;
mod loader;
mod actions;
mod ui;
mod navigation;

use std::rc::Rc;
use std::cell::RefCell;

use slint::ComponentHandle;
use models::LauncherState;
use actions::launch_app;
use ui::{apps_to_slint_model, AppWindow};
use navigation::handle_navigation;

use std::fs;

fn get_screen_resolution() -> Option<(u32, u32)> {
    let res = fs::read_to_string("/sys/class/graphics/fb0/virtual_size").ok()?;
    let parts: Vec<&str> = res.trim().split(',').collect();
    if parts.len() == 2 {
        let w = parts[0].parse().ok()?;
        let h = parts[1].parse().ok()?;
        Some((w, h))
    } else {
        None
    }
}

fn main() -> Result<(), slint::PlatformError> {
    println!("🎬 Starting RanorTV Launcher...");
    let ui = AppWindow::new()?;
    ui.window().set_fullscreen(true);

    // Set size to match actual resolution
    if let Some((w, h)) = get_screen_resolution() {
      println!("{}", h);
      println!("{}", w);
    }

    let state = Rc::new(RefCell::new(LauncherState::default()));

    // Set initial app lists
    {
        let state_ref = state.borrow();
        ui.set_featured_apps(apps_to_slint_model(&state_ref.featured_apps));
        ui.set_installed_apps(apps_to_slint_model(&state_ref.installed_apps));
        ui.set_store_apps(apps_to_slint_model(&state_ref.store_apps));
    }

    // Wire app launch
    {
        let state_clone = state.clone();
        ui.on_launch_app(move |app_id| {
            launch_app(&state_clone.borrow(), &app_id);
        });
    }

    // Navigation handling
    {
        let ui_weak = ui.as_weak();
        ui.on_navigate(move |direction| {
            if let Some(ui) = ui_weak.upgrade() {
                handle_navigation(&ui, &direction);
            }
        });
    }

    // Other callbacks (focus, refresh_store) are similarly isolated...

    ui.run()
}
