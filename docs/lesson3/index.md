# Урок 3. Состояние и переходы

Ранее я упоминал, что переход между состояниями сложная вещь.
Но нам все равно придется это сделать, чтобы в игре было что-то, помимо меню.
Начать стоит с определения, какие состояния и сцены будут в игре?
Очевидно, первое это главное меню. Второе это состояние игры. 
Третье состояние активируется при проигрыше. 

Сделаем _enum_ в новом файле `state.rs`, чтобы была возможность использовать состояние во всех файлах:
```rust
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}
```
А также, я добавил файл `common.rs`, в котором будут часто используемые функции:
```rust
use bevy::prelude::*;

pub fn despawn_entities_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
```

Теперь, мы можем отображать разные сцены, в зависимости от состояния. Чтобы включить условный рендеринг меню, нужно внести следующие правки в файл `menu.rs`:

Добавить импорты:
```rust
use crate::state::GameState;
use crate::common::despawn_entities_with;
```

Добавить `ConditionSet`:
```rust
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Menu, menu_setup)
            .add_exit_system(GameState::Menu, despawn_entities_with::<MainMenu>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Menu)
                    .with_system(button_change_state)
                    .with_system(on_level1_button_pressed.run_if(on_pressed::<Level1Button>))
                    .with_system(on_exit_button_pressed.run_if(on_pressed::<ExitButton>))
                    .into()
            );
    }
}
```
Этот код делает следующее, при переходе в состояние `GameState::Menu`, вызывается система `menu_setup`.
Когда состояние `GameState::Menu` переходит в другое, вызывается система `despawn_entities_with::<MainMenu>`, которая удаляет ECS сущность `MainMenu`.
Следующие три системы объявлены внутри `ConditionSet`, который запускает их только мы находимся в меню. 
То-есть, эти системы не будут работать в игре и нагружать процессор.

Последний шаг — проинициализировать состояние в движке, с помощью метода `add_loopless_state`:

Добавить импорты:
```rust
mod common;
mod state;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
```

Добавить `GameState`:
```rust
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
```

Пока что не видно никаких изменений. Чтобы увидеть разницу, я добавлю `GamePlugin`, в котором начну описывать игровую логику.
Сначала, сделайте файл `game.rs`:
```rust
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
```
Этот код определяет структуру стен, которые будет в нашей игре. 

Далее уже знакомый вам код, система, которая определяет, нажата ли кнопка _ESC_ для перехода в меню и сам обработчик нажатия.
```rust
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
```
Сделаем систему `setup_game`, которая будет создавать стены и помечать их маркерами (`GameComponent`), чтобы потом можно было удалить все сущности, связанные с игрой:
```rust
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
```
И в заключении, `GamePlugin`, который запускает свои системы только в том случае, если установлено состояние `GameState::Playing`.

Теперь, если я нажму кнопку `Level 1`, то попаду в игру, а меню исчезнет.
Чтобы вернуться обратно в меню, надо нажать _ESC_, но перед этим убрав глобальные обработчик из движка:
```rust
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
        .add_plugin(game::GamePlugin)
        .run();
}
```
Теперь вместо строки `.add_system(bevy::input::system::exit_on_esc_system)` я поставил `.add_plugin(game::GamePlugin)`, чтобы добавить новый плагин в игровой цикл.
Не забудьте добавить объявление модуля `mod game` в начале файла.

По возвращению обратно в меню, цвет фона остается таким же, как в игре. Не очень красиво.
Чтобы это исправить, добавьте следующую строку в `menu.rs`:
```rust
fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor::default());
    ...
```

Обработчик кнопки `Level 1` тоже получил обновление:
```rust
fn on_level1_button_pressed(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing));
}
```
Здесь мы записываем новое состояние, чтобы движок понял, что нужно сделать переход.

Чтобы игру можно было закрыть кнопкой _ESC_ из главного меню, я добавил обработчик в `ConditionSet`:
```rust
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
```

В следующем уроке я покажу как добавить шар, который будет летать по экрану.