use slint::Model;
use crate::ui::AppWindow;

pub fn handle_navigation(ui: &AppWindow, direction: &str) {
    let current_tab = ui.get_current_tab();
    match direction {
        "left" => {
            println!("⬅️ Navigate left");

            // Move focus left
            match current_tab {
                0 => {
                    let current_focus = ui.get_featured_focus();
                    if current_focus > 0 {
                        ui.set_featured_focus(current_focus - 1);
                    }
                }
                1 => {
                    let current_focus = ui.get_apps_focus();
                    if current_focus > 0 {
                        ui.set_apps_focus(current_focus - 1);
                    }
                }
                2 => {
                    let current_focus = ui.get_store_focus();
                    if current_focus > 0 {
                        ui.set_store_focus(current_focus - 1);
                    }
                }
                _ => {}
            }
        }
        "right" => {
            println!("➡️ Navigate right");
            // Move focus right
            match current_tab {
                0 => {
                    let current_focus = ui.get_featured_focus();
                    let max_apps = ui.get_featured_apps().row_count() as i32;
                    if current_focus < max_apps - 1 {
                        ui.set_featured_focus(current_focus + 1);
                    }
                }
                1 => {
                    let current_focus = ui.get_apps_focus();
                    let max_apps = ui.get_installed_apps().row_count() as i32;
                    if current_focus < max_apps - 1 {
                        ui.set_apps_focus(current_focus + 1);
                    }
                }
                2 => {
                    let current_focus = ui.get_store_focus();
                    let max_apps = ui.get_store_apps().row_count() as i32;
                    if current_focus < max_apps - 1 {
                        ui.set_store_focus(current_focus + 1);
                    }
                }
                _ => {}
            }
        }
        "select" => {
            println!("✅ Select current item");
            // Launch the currently focused app
            match current_tab {
                0 => {
                    let focus = ui.get_featured_focus() as usize;
                    let apps = ui.get_featured_apps();
                    if let Some(app) = apps.row_data(focus) {
                        ui.invoke_launch_app(app.id);
                    }
                }
                1 => {
                    let focus = ui.get_apps_focus() as usize;
                    let apps = ui.get_installed_apps();
                    if let Some(app) = apps.row_data(focus) {
                        ui.invoke_launch_app(app.id);
                    }
                }
                2 => {
                    let focus = ui.get_store_focus() as usize;
                    let apps = ui.get_store_apps();
                    if let Some(app) = apps.row_data(focus) {
                        ui.invoke_launch_app(app.id);
                    }
                }
                _ => {}
            }
        }
        "up" => println!("⬆️ Navigate up"),
        "down" => println!("⬇️ Navigate down"),
        _ => {}
    }
}
