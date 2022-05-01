use crate::{ExplosionEvent, GibEvent, GunExplosionEvent};
use bevy::prelude::*;

#[derive(Component)]
pub struct Explosion(f64);

#[derive(Component)]
pub struct Gib(f64);

const EXPLOSION_TICK: f32 = 0.1;
const GIB_TICK: f32 = 0.1;

struct ExplosionTextures {
    pub explosion_texture_atlas_handle: Handle<TextureAtlas>,
    pub gib_texture_atlas_handle: Handle<TextureAtlas>,
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
                texture_atlas: explosion_textures.explosion_texture_atlas_handle.clone(),
                transform: event.transform,
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
                texture_atlas: explosion_textures.explosion_texture_atlas_handle.clone(),
                transform: Transform::from_translation(event.translation),
                ..Default::default()
            })
            .insert(Explosion(time.seconds_since_startup()))
            .insert(Timer::from_seconds(EXPLOSION_TICK, true));
    }
}

fn animate_gib_system(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &Gib,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, _explosion, mut timer, mut sprite, gib_texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(gib_texture_atlas_handle).unwrap();
            let num_textures = texture_atlas.textures.len();
            if sprite.index + 1 < num_textures {
                sprite.index += 1;
            } else {
                commands.entity(entity).despawn();
            }
        }
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
    for (entity, _explosion, mut timer, mut sprite, explosion_texture_atlas_handle) in
        query.iter_mut()
    {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(explosion_texture_atlas_handle).unwrap();
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
    // Explosion
    let explosion_texture_handle = asset_server.load("images/airplaneexplosion.png");
    let explosion_texture_atlas =
        TextureAtlas::from_grid(explosion_texture_handle, Vec2::new(128., 128.), 8, 1);
    let explosion_texture_atlas_handle = texture_atlases.add(explosion_texture_atlas);

    // Gib
    let gib_texture_handle = asset_server.load("images/blood1.png");
    let gib_texture_atlas =
        TextureAtlas::from_grid(gib_texture_handle, Vec2::new(128., 128.), 8, 1);
    let gib_texture_atlas_handle = texture_atlases.add(gib_texture_atlas);

    commands.insert_resource(ExplosionTextures {
        explosion_texture_atlas_handle,
        gib_texture_atlas_handle,
    });
}

fn spawn_gib_system(
    mut commands: Commands,
    explosion_textures: Res<ExplosionTextures>,
    time: Res<Time>,
    mut event_reader: EventReader<GibEvent>,
) {
    for event in event_reader.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: explosion_textures.gib_texture_atlas_handle.clone(),
                transform: event.transform,
                ..Default::default()
            })
            .insert(Gib(time.seconds_since_startup()))
            .insert(Timer::from_seconds(GIB_TICK, true));
    }
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_explosion_system)
            .add_system(spawn_explosion_system)
            .add_system(spawn_gun_explosion_system)
            .add_system(spawn_gib_system)
            .add_system(animate_explosion_system)
            .add_system(animate_gib_system);
    }
}
