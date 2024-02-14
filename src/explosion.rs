use crate::{AppState, ExplosionEvent, ExplosionType, GibEvent, GunExplosionEvent};
use bevy::prelude::*;

#[derive(Component)]
pub struct Explosion(f64);

#[derive(Component)]
pub struct Gib(f64);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
struct ExplosionTextures {
    air_explosion_texture_atlas_handle: Handle<TextureAtlas>,
    gib_texture_atlas_handle: Handle<TextureAtlas>,
    ground_explosion_texture_atlas_handle: Handle<TextureAtlas>,
}

const EXPLOSION_TICK: f32 = 0.1;
const GIB_TICK: f32 = 0.1;

fn spawn_explosion_system(
    mut commands: Commands,
    explosion_textures: Res<ExplosionTextures>,
    time: Res<Time>,
    mut event_reader: EventReader<ExplosionEvent>,
) {
    for event in event_reader.iter() {
        let explosion_texture_atlas = match event.explosion_type {
            ExplosionType::Bomb => &explosion_textures.ground_explosion_texture_atlas_handle,
            ExplosionType::Aircraft | ExplosionType::Bullet => {
                &explosion_textures.air_explosion_texture_atlas_handle
            }
        };
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: explosion_texture_atlas.clone(),
                transform: event.transform,
                ..Default::default()
            })
            .insert(Explosion(time.elapsed_seconds_f64()))
            .insert(event.explosion_type)
            .insert(AnimationTimer(Timer::from_seconds(
                EXPLOSION_TICK,
                TimerMode::Repeating,
            )));
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
            .spawn(SpriteSheetBundle {
                texture_atlas: explosion_textures
                    .air_explosion_texture_atlas_handle
                    .clone(),
                transform: Transform::from_translation(event.translation),
                ..Default::default()
            })
            .insert(Explosion(time.elapsed_seconds_f64()))
            .insert(AnimationTimer(Timer::from_seconds(
                EXPLOSION_TICK,
                TimerMode::Repeating,
            )));
    }
}

fn animate_explosion_system(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, mut timer, mut sprite, animation_texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(animation_texture_atlas_handle).unwrap();
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
    // Air Explosion
    let air_explosion_texture_handle = asset_server.load("images/airplaneexplosion.png");
    let air_explosion_texture_atlas = TextureAtlas::from_grid(
        air_explosion_texture_handle,
        Vec2::new(128., 128.),
        8,
        1,
        None,
        None,
    );
    let air_explosion_texture_atlas_handle = texture_atlases.add(air_explosion_texture_atlas);

    // Bomb Explosion
    let ground_explosion_texture_handle = asset_server.load("images/ground_explosion.png");
    let ground_explosion_textrue_atlas = TextureAtlas::from_grid(
        ground_explosion_texture_handle,
        Vec2::new(128., 128.),
        8,
        1,
        None,
        None,
    );
    let ground_explosion_texture_atlas_handle = texture_atlases.add(ground_explosion_textrue_atlas);

    // Gib
    let gib_texture_handle = asset_server.load("images/blood1.png");
    let gib_texture_atlas =
        TextureAtlas::from_grid(gib_texture_handle, Vec2::new(128., 128.), 8, 1, None, None);
    let gib_texture_atlas_handle = texture_atlases.add(gib_texture_atlas);

    commands.insert_resource(ExplosionTextures {
        air_explosion_texture_atlas_handle,
        gib_texture_atlas_handle,
        ground_explosion_texture_atlas_handle,
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
            .spawn(SpriteSheetBundle {
                texture_atlas: explosion_textures.gib_texture_atlas_handle.clone(),
                transform: event.transform,
                ..Default::default()
            })
            .insert(Gib(time.elapsed_seconds_f64()))
            .insert(AnimationTimer(Timer::from_seconds(
                GIB_TICK,
                TimerMode::Repeating,
            )));
    }
}

#[allow(dead_code)]
/// Gib, Explosion components
fn despawn(mut commands: Commands, query: Query<Entity, Or<(With<Gib>, With<Explosion>)>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Startup, setup_explosion_system).add_systems(
            Update,
            (
                spawn_explosion_system,
                spawn_gun_explosion_system,
                spawn_gib_system,
                animate_explosion_system,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
