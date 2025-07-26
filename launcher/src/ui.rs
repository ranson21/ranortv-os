use slint::{ModelRc, VecModel};
use crate::models::App;
slint::include_modules!();

pub fn apps_to_slint_model(apps: &[App]) -> ModelRc<AppItem> {
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

