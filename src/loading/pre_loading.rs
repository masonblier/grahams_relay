use crate::game_state::GameState;
use crate::inputs::MouseCamera;
use crate::loading::{LoadingUiState};
use bevy::prelude::*;

pub struct PreLoadingPlugin;

#[derive(Default)]
pub struct PreLoadingState {
    pub pre_loaded: bool,
    pub ui_entity: Option<Entity>,
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for PreLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PreLoadingState>()
            .add_system_set(SystemSet::on_enter(GameState::PreLoading)
                .with_system(setup_camera))
            .add_system_set(SystemSet::on_update(GameState::PreLoading).with_system(update_pre_loading));
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 1.7, 0.0),
        ..Default::default()
    })
    .insert(UiCameraConfig {
        show_ui: true,
        ..default()
    })
    .insert(MouseCamera::default());
}

fn update_pre_loading(
    font_assets: Res<Assets<Font>>,
    mut pre_loading: ResMut<PreLoadingState>,
    mut state: ResMut<State<GameState>>,
    loading_ui_state: Res<LoadingUiState>,
) {
    let font_asset = font_assets.get(&loading_ui_state.font_handle);
    if pre_loading.pre_loaded || font_asset.is_none() {
        return;
    }

    info!("Pre loaded: {:?}", font_asset.unwrap());
    pre_loading.pre_loaded = true;
    state.set(GameState::AssetLoading).unwrap();
}
