use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Score {
    pub shots: u32,
    pub aircraft_kills: u32,
    pub aircraft_escapes: u32,
    pub paratrooper_kills: u32,
}

//fn print_score(score: Res<Score>) {
//    info!(
//        "Score: shots {} aa {} ap {}",
//        score.shots, score.aircraft_kills, score.paratrooper_kills
//    );
//}

// UI: simple text somewhere like upper left for now
fn setup_score_ui(mut commands: Commands, time: Res<Time>, asset_server: Res<AssetServer>) {
    let minutes = (time.seconds_since_startup() / 60.).floor();
    let seconds = (time.seconds_since_startup() % 60.).floor();
    let clock_str = format!("{:02}:{:02}", minutes, seconds);

    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(font_handle);

    // Top center Timer 00:00 MM:SS
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(15.),
                left: Val::Px(15.),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            clock_str,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 600.,
                color: Color::ORANGE,
            },
            TextAlignment {
                //horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_startup_system(setup_score_ui);
        //.add_system(print_score);
    }
}
