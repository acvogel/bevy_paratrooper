use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Score {
    pub shots: u32,
    pub aircraft_kills: u32,
    pub aircraft_escapes: u32,
    pub paratrooper_kills: u32,
}

#[derive(Component)]
pub struct ClockText;

//fn print_score(score: Res<Score>) {
//    info!(
//        "Score: shots {} aa {} ap {}",
//        score.shots, score.aircraft_kills, score.paratrooper_kills
//    );
//}

fn update_clock(time: Res<Time>, mut query: Query<&mut Text, With<ClockText>>) {
    info!("update clock");
    for mut text in query.iter_mut() {
        text.sections[0].value = get_clock_string(time.seconds_since_startup());
    }
}

fn get_clock_string(seconds_since_startup: f64) -> String {
    let minutes = (seconds_since_startup / 60.).floor();
    let seconds = (seconds_since_startup % 60.).floor();
    format!("{:02}:{:02}", minutes, seconds)
}

// UI: simple text somewhere like upper left for now
fn setup_score_ui(mut commands: Commands, time: Res<Time>, asset_server: Res<AssetServer>) {
    // Top center Timer MM:SS
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.),
                    left: Val::Px(15.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                get_clock_string(time.seconds_since_startup()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.,
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
    // TODO stats on shots, kills, escapes. should be icons using the same sprites, up in the corner or bottom.
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_startup_system(setup_score_ui)
            .add_system(update_clock);
    }
}
