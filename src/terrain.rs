use crate::consts;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GROUND_COLOR: Color = Color::rgb(0., 0.68, 0.32);

#[derive(Component)]
pub struct Ground;

fn setup_ground(mut commands: Commands) {
    let custom_size = Some(Vec2::new(consts::WINDOW_WIDTH, consts::GROUND_THICKNESS));
    let y = (-consts::WINDOW_HEIGHT + consts::GROUND_THICKNESS) / 2.;

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: GROUND_COLOR,
            custom_size,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 1.5)),
        ..Default::default()
    };
    commands
        .spawn(sprite_bundle)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            consts::WINDOW_WIDTH / 2.0,
            consts::GROUND_THICKNESS / 2.0,
        ))
        .insert(
            ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC
                | ActiveCollisionTypes::DYNAMIC_STATIC,
        )
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Ground);
}

fn setup_physics(mut configuration: ResMut<RapierConfiguration>) {
    configuration.gravity = Vec2::Y * consts::GRAVITY;
}

fn setup_skyline(mut commands: Commands, asset_server: Res<AssetServer>) {
    let width = 367.;
    let height = 109.;
    let scale_multiplier = consts::WINDOW_WIDTH / width;
    let scale = Vec3::splat(scale_multiplier);
    let y = consts::GROUND_Y + 0.5 * height * scale_multiplier;
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/city4.png"),
        transform: Transform {
            translation: Vec3::new(0., y, 0.),
            scale,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ground, setup_skyline, setup_physics))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.));
    }
}
