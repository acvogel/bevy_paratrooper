use crate::bomber::Bomb;
use crate::{
    AppState, BulletCollisionEvent, CollisionType, ExplosionEvent, ExplosionType, GibEvent,
    GunExplosionEvent, GunshotEvent,
};
use bevy::audio::{AudioSink, PlaybackMode, Volume};
use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Component)]
struct GunshotAudio;
#[derive(Component)]
struct BombExplosionAudio;
#[derive(Component)]
struct ScreamAudio;
#[derive(Component)]
struct BaseExplosionAudio;
#[derive(Component)]
struct AircraftExplosionAudio;
#[derive(Component)]
struct CurrentMusicAudio;
#[derive(Component)]
struct WhistleAudio;

fn play_menu_music(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    current_music: Query<&AudioSink, With<CurrentMusicAudio>>,
) {
    // Stop current music, if any.
    // todo(adam): need to remove component?
    if let Ok(sink) = current_music.get_single() {
        sink.stop();
    }

    // Start menu music
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/565_tocf_mono_intro.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
            ..default()
        },
        CurrentMusicAudio,
    ));
}

fn stop_menu_music(music_query: Query<&AudioSink, With<CurrentMusicAudio>>) {
    for audio_sink in music_query {
        audio_sink.pause();
    }
}

fn play_level_music(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    current_music: Query<(&AudioSink, With<CurrentMusicAudio>)>,
) {
    // Stop all current music
    for sink in current_music {
        sink.stop();
    }

    // Start level music
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/565_tocf_mono_level_1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
            ..default()
        },
        CurrentMusicAudio,
    ));
}

fn gunshot_listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    events: EventReader<GunshotEvent>,
) {
    if !events.is_empty() {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/sfx_weapon_singleshot20.wav"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new_relative(0.3),
                    ..default()
                },
                ..default()
            },
            GunshotAudio,
        ))
    }
}

/// Starts whistling on Bomb spawn
fn bomb_spawned_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    bomb_query: Query<(Entity, Added<Bomb>)>,
) {
    for entity in bomb_query {
        // Attach AudioBundle to Bomb entity
        commands.entity(entity).insert({
            (
                AudioBundle {
                    source: asset_server.load("audio/falling-bomb-41038.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                    ..default()
                },
                WhistleAudio,
            )
        });
    }
}

/// Stop whistling when bombs despawn
//fn whistler_despawn_listener(
//    mut removed_whistlers: RemovedComponents<Whistler>,
//    whistles: Res<Whistles>,
//    audio_sinks: Res<Assets<AudioSink>>,
//) {
//    for entity in removed_whistlers.iter() {
//        if let Some(whistle_handle) = whistles.0.get(&entity) {
//            if let Some(sink) = audio_sinks.get(whistle_handle) {
//                sink.stop();
//            }
//        }
//    }
//}

/// Spawn bomb explosion sound
fn bomb_explosion_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut events: EventReader<ExplosionEvent>,
) {
    for _event in events.filter(|&e| e.explosion_type == ExplosionType::Bomb) {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/bomb_explosion.wav"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
                ..default()
            },
            BombExplosionAudio,
        ))
    }
}

/// Aircraft explosion
fn explosion_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut events: EventReader<BulletCollisionEvent>,
) {
    for event in events {
        match event.collision_type {
            CollisionType::Aircraft => commands.spawn((
                AudioBundle {
                    source: asset_server.load("audio/sfx_exp_double2.wav"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                    ..default()
                },
                AircraftExplosionAudio,
            )),
            CollisionType::Paratrooper => {
                let scream_path = scream_audio_paths()
                    .choose(&mut rand::thread_rng())
                    .expect("Scream audio path not found.");
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load(scream_path),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    },
                    ScreamAudio,
                ))
            }
        }
    }
}

/// Paratrooper death
fn gib_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut events: EventReader<GibEvent>,
) {
    for _event in events {
        let scream_path = scream_audio_paths()
            .choose(&mut rand::thread_rng())
            .expect("Scream audio path not found.");
        commands.spawn((
            AudioBundle {
                source: asset_server.load(scream_path),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
                ..default()
            },
            ScreamAudio,
        ))
    }
}

/// Relative file paths to scream audio files
fn scream_audio_paths() -> Vec<String> {
    (1..=14)
        .map(|i| format!("audio/screams/sfx_deathscream_human{}.wav", i))
        .collect()
}

/// End of game explosion
fn base_explosion_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    events: EventReader<GunExplosionEvent>,
) {
    if !events.is_empty() {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/sfx_exp_long4.wav"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
                ..default()
            },
            BaseExplosionAudio,
        ))
    }
}

pub struct AudioStatePlugin;

impl Plugin for AudioStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(OnEnter(AppState::MainMenu), play_menu_music)
            .add_system(OnExit(AppState::MainMenu), stop_menu_music)
            .add_system(OnEnter(AppState::InGame), play_level_music)
            .add_system(
                Update,
                (
                    gunshot_listener,
                    gib_listener,
                    base_explosion_listener,
                    bomb_spawned_listener,
                    bomb_explosion_listener,
                    explosion_listener,
                    //whistler_despawn_listener,// todo: does Bomb despawn also stop audio?
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
