use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::AppState;
use bevy::prelude::*;
use std::time::Duration;

use crate::events::*;

#[derive(Component, Debug, Default, Clone, Copy)]
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
const BOMBER_KILL_SCORE: i32 = 10;
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
#[derive(Component)]
pub struct GameClock {
    duration: Duration,
}

/// Score UI font and textures
struct ScoreAssets {
    font: Handle<Font>,
    airplane: Handle<Image>,
    paratrooper: Handle<Image>,
    bullet: Handle<Image>,
    bomber: Handle<Image>,
    bomb: Handle<Image>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ScoreAssets {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        airplane: asset_server.load("images/paraplane1.png"),
        paratrooper: asset_server.load("images/paratrooperfly1_body.png"),
        bullet: asset_server.load("images/bullet.png"),
        bomber: asset_server.load("images/bomber.png"),
        bomb: asset_server.load("images/bomb4.png"), // XXX need to slice off first texture atlas member
    });
}

/// called on enter InGame, at least initially
fn setup_score_bar(mut commands: Commands, score_assets: Res<ScoreAssets>) {
    let bar_height = 0.06 * WINDOW_HEIGHT;
    let icon_node_width = 190.0;

    // other possible hacky way: explicitly set locations via transform, don't use flexbox alignment?
    //

    commands
        // Root node
        .spawn_bundle(NodeBundle {
            //node: Node {
            //    size: Vec2::new(WINDOW_WIDTH, bar_height),
            //},
            style: Style {
                //size: Size::new(Val::Percent(100.0), Val::Percent(6.0)),
                size: Size::new(Val::Px(WINDOW_WIDTH), Val::Px(bar_height)),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(ScoreBar)
        // Bottom left score
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        //size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                        size: Size::new(Val::Px(150.0), Val::Px(bar_height)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::BLUE.into(),
                    ..Default::default()
                })
                // Score: 00000
                .with_children(|parent| {
                    parent
                        .spawn_bundle(
                            TextBundle::from_section(
                                "Score: 0000",
                                TextStyle {
                                    font: score_assets.font.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_alignment(TextAlignment::CENTER_LEFT),
                        )
                        .insert(ScoreText);
                });

            // Subcomponent for icons
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(icon_node_width * 5.0), Val::Percent(100.0)),
                        //justify_content: JustifyContent::FlexEnd,
                        //justify_content: JustifyContent::SpaceBetween,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::DARK_GREEN.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Bullets fired
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                //size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                                size: Size::new(Val::Px(icon_node_width), Val::Px(bar_height)),
                                //justify_content: JustifyContent::FlexStart,
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::YELLOW_GREEN.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(ImageBundle {
                                image: score_assets.bullet.clone().into(),
                                transform: Transform::identity()
                                    .with_scale(Vec3::new(0.25, 0.25, 1.0)),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "0000",
                                    TextStyle {
                                        font: score_assets.font.clone(),
                                        font_size: 30.0,
                                        color: Color::GOLD,
                                    },
                                ))
                                .insert(BulletText);
                        });
                    // Aircraft downed
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                //size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                                size: Size::new(Val::Px(icon_node_width), Val::Px(bar_height)),
                                justify_content: JustifyContent::FlexStart,
                                //align_items: AlignItems::Center,
                                //align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            color: Color::GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(ImageBundle {
                                image: score_assets.airplane.clone().into(),
                                //style: Style {
                                //    //justify_content: JustifyContent::FlexEnd,
                                //    //justify_content: JustifyContent::SpaceAround,
                                //    ..Default::default()
                                //},
                                transform: Transform::identity()
                                    .with_scale(Vec3::new(0.15, 0.15, 1.0)),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "000",
                                        TextStyle {
                                            font: score_assets.font.clone(),
                                            font_size: 30.0,
                                            color: Color::GOLD,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::CENTER_LEFT), //.with_style(Style {
                                                                                      //    //justify_content: JustifyContent::SpaceAround,
                                                                                      //    //align_self: AlignSelf::Center,
                                                                                      //    ..Default::default()
                                                                                      //}),
                                )
                                .insert(AircraftText);
                        });
                    // Paratroopers shot
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                //size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                                size: Size::new(Val::Px(icon_node_width), Val::Px(bar_height)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::BLACK.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(ImageBundle {
                                image: score_assets.paratrooper.clone().into(),
                                transform: Transform::identity()
                                    .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "000",
                                    TextStyle {
                                        font: score_assets.font.clone(),
                                        font_size: 30.0,
                                        color: Color::GOLD,
                                    },
                                ))
                                .insert(ParatrooperText);
                        });
                    // Bombs shot
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                //size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                                size: Size::new(Val::Px(icon_node_width), Val::Px(bar_height)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::RED.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(ImageBundle {
                                image: score_assets.bomb.clone().into(),
                                transform: Transform::identity()
                                    .with_scale(Vec3::new(0.15, 0.15, 1.0)),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle::from_section(
                                    "000",
                                    TextStyle {
                                        font: score_assets.font.clone(),
                                        font_size: 30.0,
                                        color: Color::GOLD,
                                    },
                                ))
                                .insert(BombText);
                        });
                });
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
            score_text.sections[0].value = format!("Score: {:05}", score.total_score);
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
    mut app_state: ResMut<State<AppState>>,
) {
    if events.iter().next().is_some() {
        app_state.set(AppState::GameOver).unwrap();
    }
}

fn update_clock_ui(
    game_clock: Res<GameClock>,
    mut query: Query<&mut Text, With<ClockText>>,
    score: Res<Score>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = get_clock_string(game_clock.duration.as_secs_f64(), *score);
    }
}

fn setup_game_clock(mut commands: Commands) {
    commands.insert_resource(GameClock {
        duration: Duration::ZERO,
    });
}

fn update_game_clock(mut game_clock: ResMut<GameClock>, time: Res<Time>) {
    game_clock.duration += Duration::from_secs_f64(time.delta_seconds_f64());
}

fn get_clock_string(seconds_since_startup: f64, score: Score) -> String {
    let minutes = (seconds_since_startup / 60.).floor();
    let seconds = (seconds_since_startup % 60.).floor();
    let clock_str = format!(
        "{:02}:{:02}\nShots: {}\nPlanes: {}\nTroops: {}",
        minutes, seconds, score.shots, score.aircraft_kills, score.paratrooper_kills
    );

    clock_str
}

// UI: simple text somewhere like upper left for now
fn setup_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Top center Timer MM:SS
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(15.),
                    left: Val::Px(15.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.,
                    color: Color::RED,
                },
            ),
            ..Default::default()
        })
        .insert(ClockText);
}

fn despawn_score_ui(mut commands: Commands, query: Query<Entity, With<ClockText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn game_over(mut game_clock: ResMut<GameClock>, mut score: ResMut<Score>) {
    game_clock.duration = Duration::ZERO;
    *score = Score::default();
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_startup_system(load_assets)
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(setup_score_bar)
                    .with_system(setup_game_clock), //.with_system(setup_score_ui),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(kill_listener_system)
                    .with_system(gib_listener_system)
                    .with_system(gun_listener_system)
                    .with_system(landing_listener_system)
                    .with_system(gun_explosion_listener_system)
                    .with_system(update_game_clock)
                    .with_system(update_score_bar), //.with_system(update_clock_ui),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                    //.with_system(despawn_score_ui)
                    .with_system(despawn_score_bar),
            )
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(game_over));
    }
}
