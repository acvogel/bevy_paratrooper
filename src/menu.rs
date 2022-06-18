use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    Paused,
}

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct PauseText;

struct FontHandles {
    handle: Handle<Font>,
}

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandles {
        handle: asset_server.load("fonts/FiraSans-Bold.ttf"),
    });
}

/// Draw a big text sprite in the top middle
fn setup_title_screen(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands
        .spawn_bundle(TextBundle {
            node: Default::default(),
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(200.0),
                    top: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "PARATROOPER",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 125.0,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            calculated_size: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
        })
        .insert(TitleText);
}

fn despawn_title_screen(mut commands: Commands, query: Query<Entity, With<TitleText>>) {
    for title_text in query.iter() {
        commands.entity(title_text).despawn();
    }
}

fn any_key_listener(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard_input.get_just_pressed().count() > 0 {
        app_state.set(AppState::InGame).unwrap();
    }
}

#[derive(Component)]
pub struct ContinueText;

fn spawn_game_over_text(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(50.0),
                    bottom: Val::Px(250.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Press the ANY key to continue.",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 75.0,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(ContinueText);
}

fn despawn_game_over_text(mut commands: Commands, query: Query<Entity, With<ContinueText>>) {
    if let Some(handle) = query.iter().next() {
        commands.entity(handle).despawn();
    }
}

/// Pause the game, only while in game
fn pause_listener(
    mut state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    if keyboard_input.just_pressed(KeyCode::Pause) {
        match state.current() {
            AppState::Paused => {
                // Unpause
                rapier_configuration.physics_pipeline_active = true;
                rapier_configuration.query_pipeline_active = true;
                state.pop().unwrap();
            }
            AppState::InGame => {
                // Pause
                rapier_configuration.physics_pipeline_active = false;
                rapier_configuration.query_pipeline_active = false;
                state.push(AppState::Paused).unwrap();
            }
            _ => (),
        };
    }
}

fn spawn_pause_ui(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                text: Text::with_section(
                    "PAUSED",
                    TextStyle {
                        font: fonts.handle.clone(),
                        font_size: 75.0,
                        color: Color::BLUE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(PauseText);
}

fn despawn_pause_ui(mut commands: Commands, query: Query<Entity, With<PauseText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_fonts)
            .add_system(pause_listener)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_title_screen))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(any_key_listener))
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu).with_system(despawn_title_screen),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver).with_system(spawn_game_over_text),
            )
            .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(any_key_listener))
            .add_system_set(
                SystemSet::on_exit(AppState::GameOver).with_system(despawn_game_over_text),
            )
            .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(spawn_pause_ui))
            .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(despawn_pause_ui));
    }
}
