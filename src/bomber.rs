use crate::aircraft::{Aircraft, SPAWN_LEFT_X, SPAWN_RIGHT_X, SPAWN_Y_MAX, SPAWN_Y_MIN};
use crate::{
    AppState, BombDropEvent, BulletCollisionEvent, CollisionType, ExplosionEvent, ExplosionType,
};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::MassProperties;
use bevy_rapier2d::prelude::*;

use crate::consts::GRAVITY;
use crate::gun::Gun;
use crate::terrain::Ground;
use rand::Rng;

const BOMBER_SPAWN_PROBABILITY: f32 = 0.003;
const BOMBER_SPEED: f32 = 300.;
const BOMBER_SCALE: f32 = 0.3;
const BOMB_Z: f32 = 1.9;
const BOMB_SCALE: f32 = 0.3;
const BOMB_DAMPING: f32 = 1.0;
const BOMB_AIM_EPSILON: f32 = 5.0;

const BOMB_PAYLOAD: usize = 1;

#[derive(Component)]
struct Bomber {
    num_dropped: usize,
}

#[derive(Component)]
pub struct Bomb;

#[derive(Resource)]
struct BomberTextures {
    bomber_texture_handle: Handle<Image>,
    bomb_texture_atlas_handle: Handle<TextureAtlasLayout>,
    bomb_texture_handle: Handle<Image>,
}

/// Load textures
fn setup_bomber_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let bomb_texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(64., 128.), 4, 1, None, None);
    commands.insert_resource(BomberTextures {
        bomber_texture_handle: asset_server.load("images/bomber.png"),
        bomb_texture_atlas_handle: texture_atlases.add(bomb_texture_atlas),
        bomb_texture_handle: asset_server.load("images/bomb4.png"),
    });
}

/// Will add toggles or whatever else with "waves"
fn spawn_bomber_system(mut commands: Commands, textures: Res<BomberTextures>) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < BOMBER_SPAWN_PROBABILITY {
        let y = rng.gen_range(SPAWN_Y_MIN..SPAWN_Y_MAX);
        let heading_right = rng.gen_bool(0.5);
        let speed = rng.gen_range(0.8..1.3) * BOMBER_SPEED;
        let multiplier = if heading_right { 1.0 } else { -1.0 };
        let velocity = multiplier * speed;
        let transform = if heading_right {
            Transform::from_translation(Vec3::new(SPAWN_LEFT_X, y, 3.))
        } else {
            Transform::from_translation(Vec3::new(SPAWN_RIGHT_X, y, 3.))
        }
        .with_scale(Vec3::new(BOMBER_SCALE, BOMBER_SCALE, 1.));

        let sprite_bundle = SpriteBundle {
            texture: textures.bomber_texture_handle.clone(),
            sprite: Sprite {
                flip_x: !heading_right,
                ..default()
            },
            ..default()
        };

        commands
            .spawn(sprite_bundle)
            .insert(transform)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(412. / 2.0, 114. / 2.0))
            .insert(Sensor)
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(CollisionGroups::new(
                Group::GROUP_3,
                Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
            ))
            .insert(LockedAxes::TRANSLATION_LOCKED_Y)
            .insert(Velocity {
                linvel: Vec2::new(velocity, 0.),
                angvel: 0.0,
            })
            .insert(Aircraft { paratroopers: 0 })
            .insert(Bomber { num_dropped: 0 });
    }
}

/// Should drop the bomb?
fn should_bomb(bomb_transform: &Transform, velocity: &Velocity, gun_transform: &Transform) -> bool {
    let drop_distance = bomb_transform.translation.y - gun_transform.translation.y;
    // Drop time without taking into account damping. Will result in short drops.
    let simple_impact_time = (-2.0 * drop_distance / GRAVITY).sqrt();
    let pos_x = bomb_transform.translation.x - gun_transform.translation.x
        + 0.4 * velocity.linvel.x * (BOMB_DAMPING * simple_impact_time + 1.0).ln() / BOMB_DAMPING;
    (pos_x - gun_transform.translation.x).abs() < BOMB_AIM_EPSILON
}

/// Set them up the bomb
fn spawn_bombs(
    mut commands: Commands,
    mut bomber_query: Query<(&mut Bomber, &Transform, &Velocity)>,
    bomber_textures: Res<BomberTextures>,
    gun_query: Query<(&Gun, &Transform)>,
    mut event_writer: EventWriter<BombDropEvent>,
) {
    for (_gun, gun_transform) in gun_query.iter() {
        for (mut bomber, bomber_transform, velocity) in bomber_query.iter_mut() {
            if bomber.num_dropped < BOMB_PAYLOAD
                && should_bomb(bomber_transform, velocity, gun_transform)
            {
                event_writer.send(BombDropEvent);

                bomber.num_dropped += 1;
                let heading = velocity.linvel.x.signum();
                let bomb_pos = Vec2::new(
                    bomber_transform.translation.x - heading * 35.,
                    bomber_transform.translation.y - 25.,
                );

                commands
                    .spawn(RigidBody::Dynamic)
                    .insert(Sensor)
                    .insert(SpriteSheetBundle {
                        atlas: TextureAtlas {
                            layout: bomber_textures.bomb_texture_atlas_handle.clone(),
                            index: 0,
                        },

                        texture: bomber_textures.bomb_texture_handle.clone(),
                        ..default()
                    })
                    .insert(Transform {
                        translation: Vec3::new(bomb_pos.x, bomb_pos.y, BOMB_Z),
                        scale: Vec3::new(BOMB_SCALE, BOMB_SCALE, 1.0),
                        rotation: Quat::from_rotation_z(heading * std::f32::consts::FRAC_PI_2),
                        ..Default::default()
                    })
                    .insert(Damping {
                        linear_damping: BOMB_DAMPING,
                        angular_damping: 1.0,
                    })
                    .insert(GravityScale(10.0))
                    .insert(
                        bevy_rapier2d::prelude::AdditionalMassProperties::MassProperties(
                            MassProperties {
                                mass: 10.0,
                                principal_inertia: 0.5,
                                ..Default::default()
                            },
                        ),
                    )
                    .insert(Velocity {
                        linvel: velocity.linvel.clone(),
                        angvel: heading * -1.5,
                    })
                    .insert(Collider::cuboid(
                        BOMB_SCALE * 64.0 / 2.0,
                        BOMB_SCALE * 128. / 2.0,
                    ))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Bomb);
            }
        }
    }
}

/// Bombs collisions. Gun: game over. Ground: bomb explode. Just use the same animations for now.
fn bomb_bullet_collision_system(
    mut commands: Commands,
    mut events: EventReader<BulletCollisionEvent>,
    mut event_writer: EventWriter<ExplosionEvent>,
    bombs: Query<&Transform, With<Bomb>>,
) {
    for event in events.read() {
        if event.collision_type == CollisionType::Bomb {
            if let Ok(transform) = bombs.get(event.target_entity) {
                event_writer.send(ExplosionEvent {
                    transform: transform.with_rotation(Quat::IDENTITY),
                    // TODO mid-air should be different animation
                    explosion_type: ExplosionType::Bomb,
                });
                commands.entity(event.target_entity).despawn_recursive();
            }
        }
    }
}

/// Bombs explode when they hit the ground
fn bomb_terrain_collision_system(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<ExplosionEvent>,
    bomb_query: Query<&Transform, With<Bomb>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    for &event in events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            if let Some((bomb_entity, _ground_entity)) = match (entity1, entity2) {
                (e1, e2) if bomb_query.contains(e1) && ground_query.contains(e2) => Some((e1, e2)),
                (e1, e2) if bomb_query.contains(e2) && ground_query.contains(e1) => Some((e2, e1)),
                _ => None,
            } {
                let bomb_transform = bomb_query.get(bomb_entity).unwrap();
                event_writer.send(ExplosionEvent {
                    transform: bomb_transform.with_rotation(Quat::IDENTITY),
                    explosion_type: ExplosionType::Bomb,
                });
                commands.entity(bomb_entity).despawn_recursive();
            }
        }
    }
}

fn despawn_bomber_system(
    mut commands: Commands,
    mut bombers: Query<Entity, Or<(With<Bomber>, With<Bomb>)>>,
) {
    for entity in bombers.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct BomberPlugin;

impl Plugin for BomberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bomber_system)
            .add_systems(
                Update,
                (
                    spawn_bomber_system,
                    bomb_bullet_collision_system,
                    bomb_terrain_collision_system,
                    spawn_bombs,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::MainMenu), despawn_bomber_system)
            .add_systems(OnEnter(AppState::GameOver), despawn_bomber_system);
    }
}
