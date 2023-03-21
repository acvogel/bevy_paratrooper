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
            node: Default::default(),
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(200.0),
                    top: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "PARATROOPER",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 125.0,
                    color: Color::RED,
                },
            ),
            ..Default::default()
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
                position: UiRect {
                    left: Val::Px(50.0),
                    bottom: Val::Px(250.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "Press the ANY key to continue.",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 75.0,
                    color: Color::RED,
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
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    if keyboard_input.just_pressed(KeyCode::Pause) {
        match state.0 {
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
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                text: Text::from_section(
                    "PAUSED",
                    TextStyle {
                        font: fonts.handle.clone(),
                        font_size: 75.0,
                        color: Color::BLUE,
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
            .add_system(setup_title_screen.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(any_key_listener.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(despawn_title_screen.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(spawn_game_over_text.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(any_key_listener.in_set(OnUpdate(AppState::GameOver)))
            .add_system(despawn_game_over_text.in_schedule(OnExit(AppState::GameOver)))
            .add_system(spawn_pause_ui.in_schedule(OnEnter(AppState::Paused)))
            .add_system(despawn_pause_ui.in_schedule(OnExit(AppState::Paused)));
    }
}
