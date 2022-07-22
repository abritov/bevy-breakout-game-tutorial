use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::GameState;
use crate::common::despawn_entities_with;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct Level1Button;

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor::default());

    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: Rect::all(Val::Px(8.0)),
        margin: Rect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };
    let button_textstyle = TextStyle {
        font: asset_server.load("FiraMono-Medium.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    let menu = commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: Rect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .id();

    let level1_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            ..Default::default()
        })
        .with_children(|btn| {
            btn.spawn_bundle(TextBundle {
                text: Text::with_section("Level 1", button_textstyle.clone(), Default::default()),
                ..Default::default()
            });
        })
        .insert(Level1Button)
        .id();

    let button_exit = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            ..Default::default()
        })
        .with_children(|btn| {
            btn.spawn_bundle(TextBundle {
                text: Text::with_section("Exit", button_textstyle.clone(), Default::default()),
                ..Default::default()
            });
        })
        .insert(ExitButton)
        .id();

    commands
        .entity(menu)
        .push_children(&[level1_button, button_exit]);
}

fn button_change_state(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::rgb(0.75, 0.75, 0.75));
            }
            Interaction::Hovered => {
                *color = UiColor(Color::rgb(0.8, 0.8, 0.8));
            }
            Interaction::None => {
                *color = UiColor(Color::rgb(1.0, 1.0, 1.0));
            }
        }
    }
}

fn on_pressed<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }
    false
}

fn on_level1_button_pressed(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing));
}

fn on_exit_button_pressed(mut ev: EventWriter<bevy::app::AppExit>) {
    ev.send(bevy::app::AppExit);
}

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Menu, menu_setup)
            .add_exit_system(GameState::Menu, despawn_entities_with::<MainMenu>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Menu)
                    .with_system(button_change_state)
                    .with_system(bevy::input::system::exit_on_esc_system)
                    .with_system(on_level1_button_pressed.run_if(on_pressed::<Level1Button>))
                    .with_system(on_exit_button_pressed.run_if(on_pressed::<ExitButton>))
                    .into()
            );
    }
}