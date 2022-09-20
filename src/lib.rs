mod diag;
mod game_state;
mod inputs;
mod loading;
mod movement;
mod menu;
mod settings;
mod world;

use crate::diag::DiagOverlayPlugin;
use crate::game_state::GameState;
use crate::inputs::{KeyInputPlugin, MouseInputPlugin};
use crate::loading::{AssetLoadingPlugin,LoadingUiStatePlugin,PreLoadingPlugin};
use crate::menu::MenuPlugin;
use crate::movement::{CharacterLoadingPlugin,MovementStatePlugin};
use crate::settings::SettingsPlugin;
use crate::world::{WorldAssetLoaderPlugin,WorldLoadingPlugin,WorldStatePlugin};

use bevy::app::App;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::PreLoading)
            .add_plugin(LoadingUiStatePlugin)
            .add_plugin(PreLoadingPlugin)
            .add_plugin(AssetLoadingPlugin)
            .add_plugin(CharacterLoadingPlugin)
            .add_plugin(SettingsPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(DiagOverlayPlugin)
            .add_plugin(WorldAssetLoaderPlugin)
            .add_plugin(WorldLoadingPlugin)
            .add_plugin(KeyInputPlugin)
            .add_plugin(MouseInputPlugin)
            .add_plugin(MovementStatePlugin)
            .add_plugin(WorldStatePlugin)
            ;
    }
}
