use bevy::prelude::*;

use crate::events::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Score {
    pub shots: u32,
    pub aircraft_kills: u32,
    pub aircraft_escapes: u32,
    pub paratrooper_kills: u32,
    pub paratroopers_landed: u32,
}

#[derive(Component)]
pub struct ClockText;

fn kill_listener_system(mut events: EventReader<BulletCollisionEvent>, mut score: ResMut<Score>) {
    for bullet_collision_event in events.iter() {
        match bullet_collision_event.collision_type {
            CollisionType::Paratrooper => score.paratrooper_kills += 1,
            CollisionType::Aircraft => score.aircraft_kills += 1,
        }
    }
}

fn gun_listener_system(mut events: EventReader<GunshotEvent>, mut score: ResMut<Score>) {
    for _gunshot in events.iter() {
        score.shots += 1;
    }
}

fn landing_listener_system(mut events: EventReader<LandingEvent>, mut score: ResMut<Score>) {
    for _landing in events.iter() {
        score.paratroopers_landed += 1;
    }
}

fn update_clock(time: Res<Time>, mut query: Query<&mut Text, With<ClockText>>, score: Res<Score>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = get_clock_string(time.seconds_since_startup(), *score);
    }
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
fn setup_score_ui(mut commands: Commands, _time: Res<Time>, asset_server: Res<AssetServer>) {
    // Top center Timer MM:SS
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.),
                    left: Val::Px(15.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                //get_clock_string(time.seconds_since_startup(), Score::default()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ClockText);
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_startup_system(setup_score_ui)
            .add_system(kill_listener_system)
            .add_system(gun_listener_system)
            .add_system(landing_listener_system)
            .add_system(update_clock);
    }
}
