use crate::consts;
use bevy::prelude::*;

const GROUND_COLOR: Color = Color::rgb(0., 0.68, 0.32);

fn setup_ground(mut commands: Commands) {
    let custom_size = Some(Vec2::new(1280., consts::GROUND_THICKNESS));
    // starting point: window bottom + 1/2 thickness
    let y = (-consts::WINDOW_HEIGHT + consts::GROUND_THICKNESS) / 2.;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: GROUND_COLOR,
            custom_size: custom_size,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 0.)),
        ..Default::default()
    });
}

fn setup_skyline(mut commands: Commands, asset_server: Res<AssetServer>) {
    let width = 367.;
    let height = 109.;
    let scale_multiplier = consts::WINDOW_WIDTH / width;
    let scale = Vec3::splat(scale_multiplier);
    let y = consts::GROUND_Y + 0.5 * height * scale_multiplier;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("gfx/skylines/city4.png"),
        transform: Transform {
            translation: Vec3::new(0., y, 0.),
            scale: scale,
            ..Default::default()
        },
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
