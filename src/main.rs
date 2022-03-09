use bevy::prelude::*;

use aircraft::AircraftPlugin;
use bullet::BulletPlugin;
use events::*;
use gun::GunPlugin;
use paratrooper::ParatrooperPlugin;
use score::ScorePlugin;
use terrain::TerrainPlugin;

mod aircraft;
mod bullet;
mod consts;
mod events;
mod gun;
mod paratrooper;
mod score;
mod terrain;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Paratrooper".to_string(),
            width: consts::WINDOW_WIDTH,
            height: consts::WINDOW_HEIGHT,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_event::<BulletCollisionEvent>()
        .add_event::<GunshotEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(AircraftPlugin)
        .add_plugin(ParatrooperPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(TerrainPlugin)
        .add_startup_system(setup_camera)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
