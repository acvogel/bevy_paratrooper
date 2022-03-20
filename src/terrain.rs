use crate::consts;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GROUND_COLOR: Color = Color::rgb(0., 0.68, 0.32);

// TODO
// X. paratroopers don't collide with each other
// 2. ground collider matches the ground sprite
// 3. gun collider
// 4. intersection event paratrooper -> gun collider?
// X. non-bouncing paratroopers (done mostly)

// other project:
// separate the sprite for parachute and paratrooper
// deploy parachute after dropping and modify drag accordingly
// terminal velocity on paratrooper
// parachute despawn after paratrooper landing (collision event detection)

fn setup_ground(mut commands: Commands) {
    let custom_size = Some(Vec2::new(consts::WINDOW_WIDTH, consts::GROUND_THICKNESS));
    // starting point: window bottom + 1/2 thickness
    let y = (-consts::WINDOW_HEIGHT + consts::GROUND_THICKNESS) / 2.;
    //let y = consts::GROUND_Y / 2.; // middle of ground
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: GROUND_COLOR,
            custom_size: custom_size,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., y, 0.)),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(consts::WINDOW_WIDTH / 2.0, consts::GROUND_THICKNESS / 2.0)
            .into(),
        material: ColliderMaterial {
            restitution: 0.,
            restitution_combine_rule: CoefficientCombineRule::Min,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    let body = RigidBodyBundle {
        body_type: RigidBodyTypeComponent(RigidBodyType::Static),
        position: [0., y].into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(sprite_bundle)
        .insert_bundle(body)
        .insert_bundle(collider)
        //.insert(ColliderDebugRender::with_id(0))
        .insert(ColliderPositionSync::Discrete);
}

fn setup_physics(mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 1.;
    configuration.gravity = Vector::y() * -9.81;
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
            .add_startup_system(setup_skyline)
            .add_startup_system(setup_physics)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin);
    }
}
