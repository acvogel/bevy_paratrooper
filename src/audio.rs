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
struct MainMenuMusic;
#[derive(Component)]
struct LevelMusic;
#[derive(Component)]
struct WhistleAudio;

/*
  Music control flow:
    * enter MainMenu: start menu music
    * exit MainMenu: stop menu music
    * enter InGame: Start in-game music if not already playing
    * enter GameOver: stop level music, if playing
    * enter pause: stop all sound
    * exit pause: start all paused sound

  Audio control flow:
    * Event listeners for explosions, gibs, gunshots
    * Play anytime except for pause
*/

fn play_menu_music(mut commands: Commands, asset_server: ResMut<AssetServer>) {
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
        MainMenuMusic,
    ));
}

fn stop_menu_music(
    mut commands: Commands,
    music_query: Query<(Entity, &AudioSink), With<MainMenuMusic>>,
) {
    for (entity, audio_sink) in music_query.iter() {
        audio_sink.stop();
        commands.entity(entity).despawn();
    }
}

fn play_level_music(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    current_music: Query<&AudioSink, With<LevelMusic>>,
) {
    if current_music.is_empty() {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/565_tocf_mono_level_1.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    ..default()
                },
                ..default()
            },
            LevelMusic,
        ));
    }
}

fn stop_level_music(
    mut commands: Commands,
    level_music: Query<(Entity, &AudioSink), With<LevelMusic>>,
) {
    for (entity, audio_sink) in level_music.iter() {
        audio_sink.stop();
        commands.entity(entity).despawn();
    }
}

fn gunshot_listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<GunshotEvent>,
) {
    for _ in events.read() {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/sfx_weapon_singleshot20.wav"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(0.3),
                    ..default()
                },
                ..default()
            },
            GunshotAudio,
        ));
    }
}

/// Starts whistling on Bomb spawn
fn bomb_spawned_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    bomb_query: Query<Entity, Added<Bomb>>,
) {
    for entity in bomb_query.iter() {
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

/// Spawn bomb explosion sound
fn bomb_explosion_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut events: EventReader<ExplosionEvent>,
) {
    for _event in events
        .read()
        .filter(|&e| e.explosion_type == ExplosionType::Bomb)
    {
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
        ));
    }
}

/// Aircraft explosion
fn explosion_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut events: EventReader<BulletCollisionEvent>,
) {
    for event in events.read() {
        match event.collision_type {
            CollisionType::Aircraft => {
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/sfx_exp_double2.wav"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    },
                    AircraftExplosionAudio,
                ));
            }
            CollisionType::Paratrooper => {
                let scream_path = scream_audio_paths()
                    .choose(&mut rand::thread_rng())
                    .expect("Scream audio path not found.")
                    .to_string();
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
                ));
            }
            _ => (),
        }
    }
}

/// Paratrooper death
fn gib_listener(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut events: EventReader<GibEvent>,
) {
    for _event in events.read() {
        let scream_path = scream_audio_paths()
            .choose(&mut rand::thread_rng())
            .expect("Scream audio path not found.")
            .to_string();
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
        ));
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
    mut events: EventReader<GunExplosionEvent>,
) {
    if !events.is_empty() {
        events.clear();
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
        ));
    }
}

/// Pause all active playing audio in pause (including music), resume otherwise
fn pause_all_audio(query: Query<&AudioSink>) {
    for audio_sink in query
        .iter()
        .filter(|&audio_sink| !audio_sink.is_paused() && !audio_sink.empty())
    {
        audio_sink.pause();
    }
}

fn play_all_audio(query: Query<&AudioSink>) {
    for audio_sink in query.iter().filter(|&audio_sink| audio_sink.is_paused()) {
        audio_sink.play();
    }
}

pub struct AudioStatePlugin;

impl Plugin for AudioStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), play_menu_music)
            .add_systems(OnExit(AppState::MainMenu), stop_menu_music)
            .add_systems(OnEnter(AppState::InGame), play_level_music)
            .add_systems(
                Update,
                (
                    gunshot_listener,
                    gib_listener,
                    base_explosion_listener,
                    bomb_spawned_listener,
                    bomb_explosion_listener,
                    explosion_listener,
                ),
            )
            .add_systems(OnEnter(AppState::Paused), pause_all_audio)
            .add_systems(OnExit(AppState::Paused), play_all_audio)
            .add_systems(OnEnter(AppState::GameOver), stop_level_music);
    }
}
