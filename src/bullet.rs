use bevy::prelude::*;

use crate::gun::Gun;

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

/// Load bullet assets
fn setup_bullets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bullet-sprite.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 4);
    /*let texture_atlas_handle = */
    texture_atlases.add(texture_atlas);
}
fn move_bullets(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation =
            transform.translation + time.delta_seconds() * bullet.speed * transform.local_y();
    }
}

/// Despawn bullets once off screen
fn despawn_bullets() {}

fn shoot_gun(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Gun, &Transform)>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (_gun, transform) in query.iter_mut() {
            let mut bullet_transform = transform.clone();
            bullet_transform.translation =
                bullet_transform.translation + 30. * bullet_transform.local_y();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0., 1.0),
                        custom_size: Some(Vec2::new(10., 10.)),
                        ..Default::default()
                    },
                    transform: bullet_transform,
                    ..Default::default()
                })
                .insert(Bullet { speed: 100. });
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bullets)
            .add_system(move_bullets)
            .add_system(despawn_bullets)
            .add_system(shoot_gun);
    }
}
