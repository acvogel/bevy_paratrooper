use bevy::prelude::*;

use aircraft::AircraftPlugin;
use bullet::BulletPlugin;
use gun::GunPlugin;
use score::ScorePlugin;

mod aircraft;
mod bullet;
mod gun;
mod score;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Paratrooper".to_string(),
            width: 1280.,
            height: 720.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(AircraftPlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_ground(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.68, 0.32),
            custom_size: Some(Vec2::new(1280., 600.)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., -500., 0.)),
        ..Default::default()
    });
}
