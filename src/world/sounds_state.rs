use crate::game_state::GameState;
use crate::inputs::{MouseCamera,MouseLookState};
use crate::world::WorldState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
pub struct SoundsStatePlugin;

pub enum SoundsEventAction {
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
    world_state: Res<WorldState>,
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
            let panning = (mouse_look.right.dot(camera_transform.translation() - sounds_state.position) * 0.5)
                .max(-1.0).min(1.0) * -0.5 + 0.5;
            if (sounds_state.panning - panning).abs() > f32::EPSILON {
                audio.channel(sound_name).set_panning(panning as f64);
                sounds_state.panning = panning;
            }
        }
    }
}
