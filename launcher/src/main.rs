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
use chrono::{DateTime, Local};

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
        
        // Set initial background based on first featured app
        if let Some(first_app) = state_ref.featured_apps.first() {
            let bg_path = &first_app.background;
            ui.set_background_image(slint::Image::load_from_path(std::path::Path::new(&bg_path)).unwrap_or_default());
        }
    }

    // Set up datetime timer - store it to keep it alive
    let ui_weak = ui.as_weak();
    let datetime_timer = slint::Timer::default();
    datetime_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
        if let Some(ui) = ui_weak.upgrade() {
            let now: DateTime<Local> = Local::now();
            let date_str = now.format("%A, %B %d").to_string();
            let time_str = now.format("%l:%M %p").to_string();
            
            println!("Setting date: {}, time: {}", date_str, time_str);
            
            ui.set_current_date(date_str.into());
            ui.set_current_time(time_str.into());
        }
    });

    // Wire app launch
    {
        let state_clone = state.clone();
        ui.on_launch_app(move |app_id| {
            launch_app(&state_clone.borrow(), &app_id);
        });
    }

    // Navigation handling with background updates
    {
        let ui_weak = ui.as_weak();
        let state_clone = state.clone();
        ui.on_navigate(move |direction| {
            if let Some(ui) = ui_weak.upgrade() {
                handle_navigation(&ui, &direction);
                
                // Update background when focus changes
                let state_ref = state_clone.borrow();
                let current_tab = ui.get_current_tab();
                let focused_app = match current_tab {
                    0 => {
                        let focus = ui.get_featured_focus() as usize;
                        state_ref.featured_apps.get(focus)
                    }
                    1 => {
                        let focus = ui.get_apps_focus() as usize;
                        state_ref.installed_apps.get(focus)
                    }
                    2 => {
                        let focus = ui.get_store_focus() as usize;
                        state_ref.store_apps.get(focus)
                    }
                    _ => None
                };
                
                if let Some(app) = focused_app {
                    let bg_path = &app.background;
                    if let Ok(image) = slint::Image::load_from_path(std::path::Path::new(&bg_path)) {
                        ui.set_background_image(image);
                    }
                }
            }
        });
    }

    // Handle focus changes for background updates
    {
        let ui_weak = ui.as_weak();
        let state_clone = state.clone();
        ui.on_focus_changed(move |tab, index| {
            if let Some(ui) = ui_weak.upgrade() {
                let state_ref = state_clone.borrow();
                let focused_app = match tab {
                    0 => state_ref.featured_apps.get(index as usize),
                    1 => state_ref.installed_apps.get(index as usize),
                    2 => state_ref.store_apps.get(index as usize),
                    _ => None
                };
                
                if let Some(app) = focused_app {
                    let bg_path = &app.background;
                    if let Ok(image) = slint::Image::load_from_path(std::path::Path::new(&bg_path)) {
                        ui.set_background_image(image);
                    }
                }
            }
        });
    }

    // Keep the timer alive by storing it
    std::mem::forget(datetime_timer);

    ui.run()
}