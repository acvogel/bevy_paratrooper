use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::aircraft::Aircraft;
use crate::gun::Gun;
use crate::paratrooper::Paratrooper;
use crate::score::Score;

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

/// Load bullet assets
fn setup_bullets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bullet-sprite.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 4);
    /*let texture_atlas_handle = */
    texture_atlases.add(texture_atlas);

    // try bullet
    let bullet_handle: Handle<Image> = asset_server.load("bullet.png");
    commands.insert_resource(bullet_handle);

    //commands
    //    .spawn_bundle(SpriteBundle {
    //        texture: bullet_handle.clone(),
    //        sprite: Sprite {
    //            custom_size: Some(Vec2::splat(128.)),
    //            ..Default::default()
    //        },
    //        transform: Transform::from_translation(Vec3::new(0., 50., 3.)),
    //        ..Default::default()
    //    })
    //    .insert(Bullet { speed: 100. });
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
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Gun, &Transform)>,
    time: Res<Time>,
    mut score: ResMut<Score>,
    //texture_atlas: Res<TextureAtlas>,
    //texture_atlases: Mut<Assets<TextureAtlas>>,
) {
    let bullet_handle: Handle<Image> = asset_server.get_handle("bullet.png");

    //let bullet_handle = asset_server.get_handle("bullet-sprite.png");
    //let bullet_index = texture_atlas.get_texture_index(&bullet_handle).unwrap();
    //commands
    //    .spawn_bundle(SpriteSheetBundle {
    //        transform: Transform {
    //            translation: Vec3::new(200., 200., 0.),
    //            scale: Vec3::splat(1.),
    //            ..Default::default()
    //        },
    //        sprite: TextureAtlasSprite::new(bullet_index),
    //        texture_atlas: texture_atlas, //atlas_handle,
    //        ..Default::default()
    //    })
    //    .insert(Bullet { speed: 100. });
    if keyboard_input.pressed(KeyCode::Space) {
        for (mut gun, transform) in query.iter_mut() {
            // check can fire
            if time.seconds_since_startup() - gun.last_fired > 0.5 {
                gun.last_fired = time.seconds_since_startup();
                score.shots += 1;

                let mut bullet_transform = transform.clone();
                bullet_transform.translation =
                    bullet_transform.translation + 30. * bullet_transform.local_y();
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: bullet_handle.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(24.)),
                            ..Default::default()
                        },
                        // Rectangle bullets
                        //sprite: Sprite {
                        //    color: Color::rgb(0.0, 0., 1.0),
                        //    custom_size: Some(Vec2::new(10., 10.)),
                        //    ..Default::default()
                        //},
                        transform: bullet_transform,
                        ..Default::default()
                    })
                    .insert(Bullet { speed: 100. });
            }
        }
    }
}

fn collision_system(
    mut commands: Commands,
    mut aircraft: Query<(Entity, &Aircraft, &Transform)>,
    mut bullets: Query<(Entity, &Bullet, &Transform)>,
    mut paratroopers: Query<(Entity, &Paratrooper, &Transform)>,
    mut score: ResMut<Score>,
) {
    for (bullet_entity, _bullet, bullet_transform) in bullets.iter_mut() {
        let mut despawn_bullet = false;

        // Shoot Aircraft
        for (aircraft_entity, _aircraft, aircraft_transform) in aircraft.iter_mut() {
            let collision_check = collide(
                aircraft_transform.translation,
                Vec2::new(30., 10.), // TODO use sprite values
                bullet_transform.translation,
                Vec2::new(24., 24.),
            );
            if let Some(_collision) = collision_check {
                println!("aircraft hit");
                despawn_bullet = true;
                commands.entity(aircraft_entity).despawn();
                score.aircraft_kills += 1;
            }
        }

        // Shoot Paratroopers
        for (paratrooper_entity, _paratrooper, paratrooper_transform) in paratroopers.iter_mut() {
            let collision_check = collide(
                paratrooper_transform.translation,
                0.5 * Vec2::new(89., 123.), // XXX paratrooper size, hit box is way too big
                bullet_transform.translation,
                Vec2::new(24., 24.),
            );
            if let Some(_collision) = collision_check {
                info!("paratrooper hit");
                despawn_bullet = true;
                commands.entity(paratrooper_entity).despawn();
                score.paratrooper_kills += 1;
            }
        }

        if despawn_bullet {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bullets)
            .add_system(move_bullets)
            .add_system(despawn_bullets)
            .add_system(shoot_gun)
            .add_system(collision_system);
    }
}
