use bevy::{
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust & Bevy Breakout game!".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}