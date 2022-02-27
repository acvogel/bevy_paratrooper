use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Score {
    pub shots: u32,
    pub aircraft_kills: u32,
    pub paratrooper_kills: u32,
    //pub start_time: f64, // good to put a clock also?
}

fn print_score(score: Res<Score>) {
    info!(
        "Score: shots {} aa {} ap {}",
        score.shots, score.aircraft_kills, score.paratrooper_kills
    );
}

// UI: simple text somewhere like upper left for now
fn setup_score_ui(mut commands: Commands) {}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_system(print_score);
    }
}
