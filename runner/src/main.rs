use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod ui;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<ui::UiState>()
        .add_system(ui::update_ui_scale_factor.system())
        .add_system(ui::ui_nightgraph.system())
        .run();
}
