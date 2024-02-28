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
    air_explosion_texture_atlas_handle: Handle<TextureAtlasLayout>,
    air_explosion_texture_handle: Handle<Image>,
    gib_texture_atlas_handle: Handle<TextureAtlasLayout>,
    gib_texture_handle: Handle<Image>,
    ground_explosion_texture_atlas_handle: Handle<TextureAtlasLayout>,
    ground_explosion_texture_handle: Handle<Image>,
}

const EXPLOSION_TICK: f32 = 0.1;
const GIB_TICK: f32 = 0.1;

fn spawn_explosion_system(
    mut commands: Commands,
    explosion_textures: Res<ExplosionTextures>,
    time: Res<Time>,
    mut event_reader: EventReader<ExplosionEvent>,
) {
    for event in event_reader.read() {
        let (explosion_texture_atlas, explosion_texture) = match event.explosion_type {
            ExplosionType::Bomb => (
                explosion_textures
                    .ground_explosion_texture_atlas_handle
                    .clone(),
                explosion_textures.ground_explosion_texture_handle.clone(),
            ),
            ExplosionType::Aircraft | ExplosionType::Bullet => (
                explosion_textures
                    .air_explosion_texture_atlas_handle
                    .clone(),
                explosion_textures.air_explosion_texture_handle.clone(),
            ),
        };
        commands
            .spawn(SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: explosion_texture_atlas,
                    index: 0,
                },
                texture: explosion_texture,
                transform: event.transform,
                ..default()
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
    for event in event_reader.read() {
        commands
            .spawn(SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: explosion_textures
                        .air_explosion_texture_atlas_handle
                        .clone(),
                    index: 0,
                },
                texture: explosion_textures.air_explosion_texture_handle.clone(),
                transform: Transform::from_translation(event.translation),
                ..default()
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
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(Entity, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (entity, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let num_textures = texture_atlases.get(&atlas.layout).unwrap().len();
            if atlas.index + 1 < num_textures {
                atlas.index += 1;
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn setup_explosion_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Air Explosion
    let air_explosion_texture_handle = asset_server.load("images/airplaneexplosion.png");
    let air_explosion_texture_atlas =
        TextureAtlasLayout::from_grid(Vec2::new(128., 128.), 8, 1, None, None);
    let air_explosion_texture_atlas_handle = texture_atlases.add(air_explosion_texture_atlas);

    // Bomb Explosion
    let ground_explosion_texture_handle = asset_server.load("images/ground_explosion.png");
    let ground_explosion_textrue_atlas =
        TextureAtlasLayout::from_grid(Vec2::new(128., 128.), 8, 1, None, None);
    let ground_explosion_texture_atlas_handle = texture_atlases.add(ground_explosion_textrue_atlas);

    // Gib
    let gib_texture_handle = asset_server.load("images/blood1.png");
    let gib_texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(128., 128.), 8, 1, None, None);
    let gib_texture_atlas_handle = texture_atlases.add(gib_texture_atlas);

    commands.insert_resource(ExplosionTextures {
        air_explosion_texture_atlas_handle,
        air_explosion_texture_handle,
        gib_texture_atlas_handle,
        gib_texture_handle,
        ground_explosion_texture_atlas_handle,
        ground_explosion_texture_handle,
    });
}

fn spawn_gib_system(
    mut commands: Commands,
    explosion_textures: Res<ExplosionTextures>,
    time: Res<Time>,
    mut event_reader: EventReader<GibEvent>,
) {
    for event in event_reader.read() {
        commands
            .spawn(SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: explosion_textures.gib_texture_atlas_handle.clone(),
                    index: 0,
                },
                texture: explosion_textures.gib_texture_handle.clone(),
                transform: event.transform,
                ..default()
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
        app.add_systems(Startup, setup_explosion_system)
            .add_systems(
                Update,
                (
                    spawn_explosion_system,
                    spawn_gun_explosion_system,
                    spawn_gib_system,
                    animate_explosion_system,
                )
                    .run_if(not(in_state(AppState::Paused))),
            );
    }
}
