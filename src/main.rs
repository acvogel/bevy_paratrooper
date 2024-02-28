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
        .insert_state(AppState::MainMenu)
        .add_plugins(ShapePlugin)
        .add_plugins(GunPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(AircraftPlugin)
        .add_plugins(BomberPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(ParatrooperPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(AudioStatePlugin)
        .add_plugins(ExplosionPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(CloudPlugin)
        .add_plugins(EventPlugin)
        .add_plugins(AssaultPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
