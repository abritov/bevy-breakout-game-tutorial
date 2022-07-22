use bevy::{
    prelude::*,
    input::ElementState,
    input::keyboard::KeyboardInput
};
use iyes_loopless::prelude::*;

use crate::state::GameState;
use crate::common::despawn_entities_with;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Component)]
struct Collider;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..Transform::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn esc_pressed(
    mut keyboard_input_events: EventReader<KeyboardInput>,
) -> bool {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed && key_code == KeyCode::Escape {
                return true;
            }
        }
    }
    false
}

fn return_to_menu(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Menu));
}

#[derive(Component)]
struct GameComponent;

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));

    commands.spawn_bundle(WallBundle::new(WallLocation::Left)).insert(GameComponent);
    commands.spawn_bundle(WallBundle::new(WallLocation::Right)).insert(GameComponent);
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom)).insert(GameComponent);
    commands.spawn_bundle(WallBundle::new(WallLocation::Top)).insert(GameComponent);
}

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Playing, setup_game)
            .add_exit_system(GameState::Playing, despawn_entities_with::<GameComponent>)
            .add_system_set(
                ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(return_to_menu.run_if(esc_pressed))
                .into()
            );
    }
}