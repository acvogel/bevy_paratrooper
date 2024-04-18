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
pub struct FontHandles {
    pub handle: Handle<Font>,
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
                left: Val::Px(200.),
                top: Val::Px(15.),
                ..default()
            },
            text: Text::from_section(
                "PARATROOPER",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 125.,
                    color: Color::RED,
                },
            ),
            ..default()
        })
        .insert(TitleText);
}

/// Press ENTER to start
fn setup_instructions(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(90.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Press ENTER to start",
                    TextStyle {
                        font: fonts.handle.clone(),
                        font_size: 50.,
                        color: Color::BLUE,
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Percent(10.),
                        ..default()
                    },
                    ..default()
                }),
                TitleText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Move ← →\nShoot SPACE",
                    TextStyle {
                        font: fonts.handle.clone(),
                        font_size: 30.,
                        color: Color::WHITE,
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Percent(5.),
                        ..default()
                    },
                    ..default()
                }),
                TitleText,
            ));
        });
}

fn despawn_title_screen(mut commands: Commands, query: Query<Entity, With<TitleText>>) {
    for title_text in query.iter() {
        commands.entity(title_text).despawn();
    }
}

/// Start game with keyboard Enter or gamepad Start
fn start_key_listener(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let keyboard_start = keyboard_inputs.just_pressed(KeyCode::Enter);
    let gamepad_start = gamepads
        .iter()
        .find(|&gamepad| {
            button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::Start))
        })
        .is_some();
    if keyboard_start || gamepad_start {
        next_state.set(AppState::InGame);
    }
}

#[derive(Component)]
pub struct ContinueText;

fn spawn_game_over_text(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands.spawn((
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                bottom: Val::Px(250.),
                ..default()
            },
            text: Text::from_section(
                "Press ENTER to continue.",
                TextStyle {
                    font: font_handles.handle.clone(),
                    font_size: 75.,
                    color: Color::RED,
                },
            ),
            ..default()
        },
        ContinueText,
    ));
}

fn despawn_game_over_text(mut commands: Commands, query: Query<Entity, With<ContinueText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Pause the game, only while in game
fn pause_listener(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    let keyboard_pause = keyboard_input.just_pressed(KeyCode::Pause);
    let gamepad_pause = gamepads
        .iter()
        .find(|&gamepad| {
            button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::Start))
        })
        .is_some();
    if keyboard_pause || gamepad_pause {
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
                        font_size: 75.,
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
            .add_systems(
                OnEnter(AppState::MainMenu),
                (setup_title_screen, setup_instructions),
            )
            .add_systems(
                Update,
                start_key_listener
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
