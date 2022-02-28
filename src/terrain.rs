use crate::consts;
use bevy::prelude::*;

fn setup_ground(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.68, 0.32),
            custom_size: Some(Vec2::new(1280., 600.)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., consts::GROUND_Y, 0.)),
        ..Default::default()
    });
}

fn setup_skyline(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn background sprite
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("gfx/skylines/city4.png"), // base 367 x 109
        // scale up 2.5x or so
        ..Default::default()
    });
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground)
            .add_startup_system(setup_skyline);
    }
}
