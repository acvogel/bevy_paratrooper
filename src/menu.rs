use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

#[derive(PartialEq, Default, Debug, Clone, Eq, Hash, Resource, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Paused,
}

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct PauseText;

#[derive(Resource)]
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
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                left: Val::Px(200.0),
                top: Val::Px(15.0),
                ..default()
            },
            text: Text::from_section(
                "PARATROOPER",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 125.0,
                    color: Color::RED,
                },
            ),
            ..default()
        })
        .insert(TitleText);
}

fn despawn_title_screen(mut commands: Commands, query: Query<Entity, With<TitleText>>) {
    for title_text in query.iter() {
        commands.entity(title_text).despawn();
    }
}

fn any_key_listener(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.get_just_pressed().count() > 0 {
        next_state.set(AppState::InGame);
    }
}

#[derive(Component)]
pub struct ContinueText;

fn spawn_game_over_text(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                bottom: Val::Px(250.0),
                ..default()
            },
            text: Text::from_section(
                "Press the ANY key to continue.",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 75.0,
                    color: Color::RED,
                },
            ),
            ..default()
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
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    if keyboard_input.just_pressed(KeyCode::Pause) {
        match state.get() {
            AppState::Paused => {
                // Unpause
                rapier_configuration.physics_pipeline_active = true;
                rapier_configuration.query_pipeline_active = true;
                next_state.set(AppState::InGame);
            }
            AppState::InGame => {
                // Pause
                rapier_configuration.physics_pipeline_active = false;
                rapier_configuration.query_pipeline_active = false;
                next_state.set(AppState::Paused);
            }
            _ => (),
        };
    }
}

fn spawn_pause_ui(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                text: Text::from_section(
                    "PAUSED",
                    TextStyle {
                        font: fonts.handle.clone(),
                        font_size: 75.0,
                        color: Color::BLUE,
                    },
                ),
                ..default()
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
        app.add_systems(Startup, load_fonts)
            .add_systems(OnEnter(AppState::MainMenu), setup_title_screen)
            .add_systems(
                Update,
                any_key_listener
                    .run_if(in_state(AppState::MainMenu).or_else(in_state(AppState::GameOver))),
            )
            .add_systems(
                Update,
                pause_listener
                    .run_if(in_state(AppState::InGame).or_else(in_state(AppState::Paused))),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_title_screen)
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_text)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_text)
            .add_systems(OnEnter(AppState::Paused), spawn_pause_ui)
            .add_systems(OnExit(AppState::Paused), despawn_pause_ui);
    }
}
