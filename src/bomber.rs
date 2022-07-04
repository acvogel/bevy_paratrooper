use crate::aircraft::{
    AIRCRAFT_SCALE, AIRCRAFT_SPAWN_PROBABILITY, AIRCRAFT_SPEED, SPAWN_LEFT_X, SPAWN_RIGHT_X,
    SPAWN_Y_MAX, SPAWN_Y_MIN,
};
use crate::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::gun::Gun;
use rand::Rng;

const BOMBER_SPEED: f32 = 200.;
const BOMBER_SCALE: f32 = 0.3;
const BOMB_Z: f32 = 1.9;
const BOMB_SCALE: f32 = 0.5;
const BOMB_DAMPING: f32 = 1.0;

#[derive(Component)]
struct Bomber {
    num_dropped: usize,
}

#[derive(Component)]
struct Bomb;

struct BomberTextures {
    bomber: Handle<Image>,
    bomb: Handle<TextureAtlas>,
}

/// Load textures
fn setup_bomber_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let bomb_texture_atlas = TextureAtlas::from_grid(
        asset_server.load("images/bomb4.png"),
        Vec2::new(64., 128.),
        4,
        1,
    );
    commands.insert_resource(BomberTextures {
        bomber: asset_server.load("images/bomber.png"),
        bomb: texture_atlases.add(bomb_texture_atlas),
    });
}

/// Will add toggles or whatever else with "waves"
fn spawn_bomber_system(mut commands: Commands, textures: Res<BomberTextures>) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < AIRCRAFT_SPAWN_PROBABILITY {
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
            texture: textures.bomber.clone(),
            sprite: Sprite {
                flip_x: !heading_right,
                ..Default::default()
            },
            ..Default::default()
        };

        commands
            .spawn()
            .insert_bundle(sprite_bundle)
            .insert(transform)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(412. / 2.0, 114. / 2.0))
            .insert(Sensor(true))
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(CollisionGroups::new(0b0100, 0b1110))
            .insert(LockedAxes::TRANSLATION_LOCKED_Y)
            .insert(Velocity {
                linvel: Vec2::new(velocity, 0.),
                angvel: 0.0,
            })
            .insert(Bomber { num_dropped: 0 });
    }
}

/// Set them up the bomb
fn spawn_bombs(
    mut commands: Commands,
    mut bomber_query: Query<(&mut Bomber, &Transform, &Velocity)>,
    bomber_textures: Res<BomberTextures>,
    gun_query: Query<(&Gun, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    for (_gun, _gun_transform) in gun_query.iter() {
        for (mut bomber, bomber_transform, velocity) in bomber_query.iter_mut() {
            if bomber.num_dropped == 0 && rng.gen_range(0.0..1.0) < 0.02 {
                bomber.num_dropped += 1;
                let heading = velocity.linvel.x.signum();
                let bomb_pos = Vec2::new(
                    bomber_transform.translation.x - heading * 35.,
                    bomber_transform.translation.y - 25.,
                );

                commands
                    .spawn()
                    .insert(RigidBody::Dynamic)
                    .insert(Sensor(true))
                    .insert_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(1),
                        texture_atlas: bomber_textures.bomb.clone(),
                        ..Default::default()
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
                    .insert(MassProperties {
                        mass: 10.0,
                        principal_inertia: 0.5,
                        ..Default::default()
                    })
                    .insert(Velocity {
                        linvel: velocity.linvel.clone(),
                        angvel: heading * -1.5,
                    })
                    .insert(Collider::cuboid(
                        BOMB_SCALE * 64.0 / 2.0,
                        BOMB_SCALE * 128. / 2.0,
                    ))
                    .insert(Bomb);
            }
        }
    }
}

pub struct BomberPlugin;

impl Plugin for BomberPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bomber_system).add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_bomber_system)
                .with_system(spawn_bombs),
        );
    }
}