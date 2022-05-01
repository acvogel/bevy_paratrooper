use crate::consts::WINDOW_WIDTH;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const NUM_CLOUDS: usize = 7;
const CLOUD_SPEED: f32 = 40.;
const CLOUD_SPAWN_PROBABILITY: f32 = 0.01;
const CLOUD_MIN_Y: f32 = -100.0;
const CLOUD_MAX_Y: f32 = 400.0;
const CLOUD_SCALE: f32 = 0.4;
const SPAWN_LEFT_X: f32 = -WINDOW_WIDTH / 2.0 - 50.;
const SPAWN_RIGHT_X: f32 = WINDOW_WIDTH / 2.0 + 50.;

#[derive(Component)]
pub struct Cloud;

struct CloudTextures {
    cloud_handles: Vec<Handle<Image>>,
}

fn setup_cloud_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let mut cloud_handles = Vec::<Handle<Image>>::new();
    for i in 1..=NUM_CLOUDS {
        let path = format!("images/cloud{}.png", i);
        cloud_handles.push(asset_server.load(&path));
    }
    commands.insert_resource(CloudTextures { cloud_handles })
}

fn spawn_cloud_system(mut commands: Commands, textures: Res<CloudTextures>) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < CLOUD_SPAWN_PROBABILITY {
        // Spawn random cloud
        let cloud_idx = rng.gen_range(0..NUM_CLOUDS);
        let cloud_handle = &textures.cloud_handles[cloud_idx];

        // Random direction
        let heading_right = rng.gen_bool(0.5);
        let velocity = if heading_right {
            CLOUD_SPEED
        } else {
            -CLOUD_SPEED
        };

        let y = rng.gen_range(CLOUD_MIN_Y..=CLOUD_MAX_Y);
        let x = if heading_right {
            SPAWN_LEFT_X
        } else {
            SPAWN_RIGHT_X
        };

        let sprite_bundle = SpriteBundle {
            texture: cloud_handle.clone(),
            transform: Transform {
                scale: Vec3::new(CLOUD_SCALE, CLOUD_SCALE, 1.0),
                translation: Vec3::new(0., 0., 8.),
                ..Default::default()
            },
            ..Default::default()
        };

        let rigid_body_bundle = RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
            position: [x, y].into(),
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(velocity, 0.0).into(),
                angvel: 0.0,
            }
            .into(),
            ..Default::default()
        };

        commands
            .spawn_bundle(sprite_bundle)
            .insert_bundle(rigid_body_bundle)
            .insert(RigidBodyPositionSync::Discrete)
            .insert(Cloud);
    }
}

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cloud_system)
            .add_system(spawn_cloud_system);
    }
}
