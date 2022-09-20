use crate::game_state::GameState;
use crate::inputs::{MouseCamera,MouseLookState};
use crate::world::{WorldState,WorldSoundState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct SoundsStatePlugin;

pub enum SoundsEventAction {
    Pause,
    Resume,
    Toggle,
}

pub struct SoundsEvent {
    pub action: SoundsEventAction,
    pub name: String,
}

impl Plugin for SoundsStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SoundsEvent>()
        .add_system_set(
            SystemSet::on_enter(GameState::Running)
            .with_system(setup_sounds_interaction)
        ).add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_sounds_interaction)
            .with_system(update_sounds_states)
        );
    }
}

fn setup_sounds_interaction(
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
    mut world_state: ResMut<WorldState>,
) {
    for (sound_name, sounds_state) in world_state.animatable_sounds.iter() {
        let channel = audio
            .create_channel(sound_name);
        channel
            .play(asset_server.load(&format!("audio/{}.ogg", sounds_state.sound)))
            .looped();
        if sounds_state.paused {
            channel.pause();
        }
    }

    // footsteps
    let channel = audio
        .create_channel("footsteps");
    channel
        .play(asset_server.load(&format!("audio/steps_snow_dry.ogg")))
        .with_volume(0.2)
        .looped();
    channel.pause();
    world_state.animatable_sounds.insert("footsteps".into(), WorldSoundState {
        sound: "steps_snow_dry".into(),
        position: Vec3::ZERO,
        paused: true,
        volume: 0.2,
        panning: 0.5,
    });
    // train
    let channel = audio
        .create_channel("train");
    channel
        .play(asset_server.load(&format!("audio/train_rolling.ogg")))
        .with_volume(1.0)
        .looped();
    channel.pause();
    world_state.animatable_sounds.insert("train".into(), WorldSoundState {
        sound: "train_rolling".into(),
        position: Vec3::ZERO,
        paused: true,
        volume: 0.5,
        panning: 0.5,
    });
}

fn update_sounds_interaction(
    audio: Res<DynamicAudioChannels>,
    mut world_state: ResMut<WorldState>,
    mut sounds_events: EventReader<SoundsEvent>,
) {
    for sounds_event in sounds_events.iter() {
        if let Some(sounds_state) = world_state.animatable_sounds.get_mut(&sounds_event.name) {
            match sounds_event.action {
                SoundsEventAction::Toggle => {
                    if sounds_state.paused {
                        audio.channel(&sounds_event.name).resume();
                        sounds_state.paused = false;
                    } else {
                        audio.channel(&sounds_event.name).pause();
                        sounds_state.paused = true;
                    }
                }
                SoundsEventAction::Pause => {
                    audio.channel(&sounds_event.name).pause();
                    sounds_state.paused = true;
                }
                SoundsEventAction::Resume => {
                    audio.channel(&sounds_event.name).resume();
                    sounds_state.paused = false;
                }
            }
        }
    }
}

fn update_sounds_states(
    audio: Res<DynamicAudioChannels>,
    mut world_state: ResMut<WorldState>,
    query: Query<&GlobalTransform, With<MouseCamera>>,
    mouse_look: Res<MouseLookState>,
) {
    let camera_transform = query.single();

    // for each playing audio, update state and panning from (camera_pos - audio_pos)
    for (sound_name, mut sounds_state) in world_state.animatable_sounds.iter_mut() {
        if !sounds_state.paused {
            if sound_name == "train" || sound_name == "footsteps" {
                continue;
            }

            let diff_v = camera_transform.translation() - sounds_state.position;
            let panning = (mouse_look.right.dot(diff_v) * 0.5)
                .max(-1.0).min(1.0) * -0.5 + 0.5;
            let volume = 1.0 - 0.1 * diff_v.length().max(0.0).min(10.0);
            if (sounds_state.panning - panning).abs() > f32::EPSILON {
                audio.channel(sound_name).set_panning(panning as f64);
                sounds_state.panning = panning;
            }
            if (sounds_state.volume - volume).abs() > f32::EPSILON {
                audio.channel(sound_name).set_volume(volume as f64);
                sounds_state.volume = volume;
            }
        }
    }
}
