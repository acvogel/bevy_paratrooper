// Disable windows console in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::assault::AssaultPlugin;
use crate::audio::AudioStatePlugin;
use crate::bomber::BomberPlugin;
use crate::cloud::CloudPlugin;
use crate::explosion::ExplosionPlugin;
use crate::menu::{AppState, MenuPlugin};
use aircraft::AircraftPlugin;
use bullet::BulletPlugin;
use events::*;
use gun::GunPlugin;
use paratrooper::ParatrooperPlugin;
use score::ScorePlugin;
use terrain::TerrainPlugin;

mod aircraft;
mod assault;
mod audio;
mod bomber;
mod bullet;
mod cloud;
mod consts;
mod events;
mod explosion;
mod gun;
mod menu;
mod paratrooper;
mod score;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Paratrooper".to_string(),
                resolution: (consts::WINDOW_WIDTH, consts::WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_state::<AppState>()
        .add_plugin(ShapePlugin)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(AircraftPlugin)
        .add_plugin(BomberPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(ParatrooperPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(AudioStatePlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(EventPlugin)
        .add_plugin(AssaultPlugin)
        .add_startup_system(setup_camera)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
