use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::AppState;
use bevy::prelude::*;
use std::time::Duration;

use crate::events::*;
use crate::paratrooper::PARATROOPER_SCALE;

const SUBSCORE_COLOR: Color = Color::GOLD;
const FONT_SIZE: f32 = 40.0;

#[derive(Component, Debug, Default, Clone, Copy, Resource)]
pub struct Score {
    pub shots: u32,
    pub aircraft_kills: u32,
    pub aircraft_escapes: u32,
    pub paratrooper_kills: u32,
    pub paratroopers_landed: u32,
    pub parachute_hits: u32,
    pub bomb_kills: u32,
    pub total_score: i32,
}

/// Score credit constants
const SHOT_SCORE: i32 = -1;
const AIRCRAFT_KILL_SCORE: i32 = 10;
const PARATROOPER_KILL_SCORE: i32 = 5;
const BOMB_KILL_SCORE: i32 = 30;

#[derive(Component)]
pub struct ClockText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ScoreBar;

#[derive(Component)]
pub struct BulletText;

#[derive(Component)]
pub struct AircraftText;

#[derive(Component)]
pub struct ParatrooperText;

#[derive(Component)]
pub struct BombText;

/// AppState::InGame time
#[derive(Component, Resource)]
pub struct GameClock {
    duration: Duration,
}

/// Score UI font and textures
#[derive(Resource)]
struct ScoreAssets {
    aircraft: Handle<Image>,
    bomb: Handle<Image>,
    font: Handle<Font>,
    paratrooper: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ScoreAssets {
        aircraft: asset_server.load("images/paraplane1_icon.png"),
        bomb: asset_server.load("images/bomb4_icon.png"),
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        paratrooper: asset_server.load("images/paratrooperfly1_body.png"),
    });
}

fn setup_score_bar(mut commands: Commands, assets: Res<ScoreAssets>) {
    let bar_height = 0.06 * WINDOW_HEIGHT;
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(WINDOW_WIDTH),
                        height: Val::Px(bar_height),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_score_text(parent, assets.font.clone());
                    spawn_clock_text(parent, assets.font.clone());
                    spawn_subscores(parent, assets);
                });
        });
}

fn spawn_clock_text(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.3),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            color: Color::RED,
                        },
                    ),
                    ..Default::default()
                })
                .insert(ClockText);
        });
}

fn spawn_aircraft_subscore(builder: &mut ChildBuilder, font: Handle<Font>, icon: Handle<Image>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.3),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // padding right?
            parent.spawn(ImageBundle {
                image: icon.clone().into(),
                style: Style {
                    margin: UiRect {
                        right: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
            parent
                .spawn(TextBundle::from_section(
                    "000",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: SUBSCORE_COLOR,
                    },
                ))
                .insert(AircraftText);
        });
}

fn spawn_bomb_subscore(builder: &mut ChildBuilder, font: Handle<Font>, icon: Handle<Image>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.3),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: icon.clone().into(),
                style: Style {
                    margin: UiRect {
                        right: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
            parent
                .spawn(TextBundle::from_section(
                    "000",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: SUBSCORE_COLOR,
                    },
                ))
                .insert(BombText);
        });
}

fn spawn_paratrooper_subscore(builder: &mut ChildBuilder, font: Handle<Font>, icon: Handle<Image>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: icon.clone().into(),
                transform: Transform::IDENTITY.with_scale(Vec3::new(
                    PARATROOPER_SCALE,
                    PARATROOPER_SCALE,
                    1.,
                )),
                ..default()
            });
            parent
                .spawn(TextBundle::from_section(
                    "000",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: SUBSCORE_COLOR,
                    },
                ))
                .insert(ParatrooperText);
        });
}

/// Paratroopers
/// Aircraft
/// Bombs
fn spawn_subscores(builder: &mut ChildBuilder, assets: Res<ScoreAssets>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.3),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_paratrooper_subscore(parent, assets.font.clone(), assets.paratrooper.clone());
            spawn_aircraft_subscore(parent, assets.font.clone(), assets.aircraft.clone());
            spawn_bomb_subscore(parent, assets.font.clone(), assets.bomb.clone());
        });
}

/// SCORE 13456
fn spawn_score_text(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.3),

                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "SCORE 00000",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_text_alignment(TextAlignment::Left),
                )
                .insert(ScoreText);
        });
}

fn update_score_bar(
    mut set: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<BulletText>>,
        Query<&mut Text, With<AircraftText>>,
        Query<&mut Text, With<ParatrooperText>>,
        Query<&mut Text, With<BombText>>,
    )>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for mut score_text in set.p0().iter_mut() {
            score_text.sections[0].value = format!("SCORE {:05}", score.total_score);
        }
        for mut bullet_text in set.p1().iter_mut() {
            bullet_text.sections[0].value = format!("{:04}", score.shots);
        }
        for mut aircraft_text in set.p2().iter_mut() {
            aircraft_text.sections[0].value = format!("{:03}", score.aircraft_kills);
        }
        for mut paratrooper_text in set.p3().iter_mut() {
            paratrooper_text.sections[0].value = format!("{:03}", score.paratrooper_kills);
        }
        for mut bomb_text in set.p4().iter_mut() {
            bomb_text.sections[0].value = format!("{:03}", score.bomb_kills);
        }
    }
}

fn despawn_score_bar(mut commands: Commands, query: Query<Entity, With<ScoreBar>>) {
    for score_bar in query.iter() {
        commands.entity(score_bar).despawn_recursive();
    }
}

/// Update score on bullet kills
fn kill_listener_system(mut events: EventReader<BulletCollisionEvent>, mut score: ResMut<Score>) {
    for bullet_collision_event in events.iter() {
        match bullet_collision_event.collision_type {
            CollisionType::Aircraft => {
                score.aircraft_kills += 1;
                score.total_score += AIRCRAFT_KILL_SCORE;
            }
            CollisionType::Parachute => score.parachute_hits += 1,
            CollisionType::Bomb => {
                score.bomb_kills += 1;
                score.total_score += BOMB_KILL_SCORE;
            }
            CollisionType::Paratrooper => (), // GibEvent covers
        }
    }
}

fn gib_listener_system(mut events: EventReader<GibEvent>, mut score: ResMut<Score>) {
    for _e in events.iter() {
        score.paratrooper_kills += 1;
        score.total_score += PARATROOPER_KILL_SCORE;
    }
}

fn gun_listener_system(mut events: EventReader<GunshotEvent>, mut score: ResMut<Score>) {
    for _gunshot in events.iter() {
        score.shots += 1;
        // Shots don't take score below 0
        score.total_score = (score.total_score + SHOT_SCORE).max(0);
    }
}

fn landing_listener_system(mut events: EventReader<LandingEvent>, mut score: ResMut<Score>) {
    for _landing in events.iter() {
        score.paratroopers_landed += 1;
    }
}

fn gun_explosion_listener_system(
    mut events: EventReader<GunExplosionEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if events.iter().next().is_some() {
        next_state.set(AppState::GameOver);
    }
}

fn update_clock_ui(game_clock: Res<GameClock>, mut query: Query<&mut Text, With<ClockText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = get_clock_string(game_clock.duration);
    }
}

fn get_clock_string(duration: Duration) -> String {
    let minutes = (duration.as_secs_f32() / 60.).floor();
    let seconds = (duration.as_secs_f32() % 60.).floor();
    format!("{:02}:{:02}", minutes, seconds)
}

fn setup_game_clock(mut commands: Commands) {
    commands.insert_resource(GameClock {
        duration: Duration::ZERO,
    });
}

fn update_game_clock(mut game_clock: ResMut<GameClock>, time: Res<Time>) {
    game_clock.duration += Duration::from_secs_f64(time.delta_seconds_f64());
}

fn game_over(mut game_clock: ResMut<GameClock>, mut score: ResMut<Score>) {
    game_clock.duration = Duration::ZERO;
    *score = Score::default();
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_system(Startup, setup)
            .add_systems(
                OnEnter(AppState::InGame),
                (setup_game_clock, setup_score_bar),
            )
            .add_systems(
                Update,
                (
                    kill_listener_system,
                    gib_listener_system,
                    gun_listener_system,
                    landing_listener_system,
                    gun_explosion_listener_system,
                    update_game_clock,
                    update_clock_ui,
                    update_score_bar,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_system(OnExit(AppState::InGame), despawn_score_bar)
            .add_system(OnEnter(AppState::GameOver), game_over);
    }
}
