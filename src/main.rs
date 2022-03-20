use bevy::prelude::*;
//use bevy_kira_audio::AudioPlugin;

//use crate::audio::AudioStatePlugin;
use crate::explosion::ExplosionPlugin;
use aircraft::AircraftPlugin;
use bullet::BulletPlugin;
use events::*;
use gun::GunPlugin;
use paratrooper::ParatrooperPlugin;
use score::ScorePlugin;
use terrain::TerrainPlugin;

mod aircraft;
mod audio;
mod bullet;
mod consts;
mod events;
mod explosion;
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
        .add_event::<LandingEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(AircraftPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(ParatrooperPlugin)
        .add_plugin(ScorePlugin)
        //.add_plugin(AudioStatePlugin)
        //.add_plugin(AudioPlugin)
        .add_plugin(ExplosionPlugin)
        .add_startup_system(setup_camera)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //let mut camera = OrthographicCameraBundle::new_2d();
    //camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    //commands.spawn_bundle(PointLightBundle {
    //    transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
    //    point_light: PointLight {
    //        intensity: 100_000_000_.0,
    //        range: 6000.0,
    //        ..Default::default()
    //    },
    //    ..Default::default()
    //});
    //commands.spawn_bundle(camera);
}
