use crate::{ExplosionEvent, GunExplosionEvent};
use bevy::prelude::*;

#[derive(Component)]
pub struct Explosion(f64);

const EXPLOSION_TICK: f32 = 0.1;

struct ExplosionTextures {
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

fn spawn_explosion_system(
    mut commands: Commands,
    explosion_textures: Res<ExplosionTextures>,
    time: Res<Time>,
    mut event_reader: EventReader<ExplosionEvent>,
) {
    for event in event_reader.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: explosion_textures.texture_atlas_handle.clone(),
                transform: event.transform.clone(),
                ..Default::default()
            })
            .insert(Explosion(time.seconds_since_startup()))
            .insert(Timer::from_seconds(EXPLOSION_TICK, true));
    }
}

fn spawn_gun_explosion_system(
    mut commands: Commands,
    explosion_textures: Res<ExplosionTextures>,
    time: Res<Time>,
    mut event_reader: EventReader<GunExplosionEvent>,
) {
    for event in event_reader.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: explosion_textures.texture_atlas_handle.clone(),
                transform: Transform::from_translation(event.translation),
                ..Default::default()
            })
            .insert(Explosion(time.seconds_since_startup()))
            .insert(Timer::from_seconds(EXPLOSION_TICK, true));
    }
}

fn animate_explosion_system(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &Explosion,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, _explosion, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let num_textures = texture_atlas.textures.len();
            if sprite.index + 1 < num_textures {
                sprite.index += 1;
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn setup_explosion_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("airplaneexplosion.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(ExplosionTextures {
        texture_atlas_handle: texture_atlas_handle,
    });
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_explosion_system)
            .add_system(spawn_explosion_system)
            .add_system(spawn_gun_explosion_system)
            .add_system(animate_explosion_system);
    }
}
