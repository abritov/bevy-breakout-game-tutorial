mod menu;
mod common;
mod state;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn initial_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust & Bevy Breakout game!".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_loopless_state(state::GameState::Menu)
        .add_startup_system(initial_setup)
        .add_plugin(menu::MenuPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}