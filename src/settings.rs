use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
// use bevy_rapier3d::prelude::*;
use serde::Deserialize;
use crate::game_state::GameState;
use crate::loading::SettingsAssets;

#[derive(Debug, Default, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a008b74b4844"]
pub struct SettingsAsset {
    pub graphics_settings: GraphicsSettings,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GraphicsSettings {
    pub render_mode: String,
}

#[derive(Default)]
pub struct SettingsLoader;

impl AssetLoader for SettingsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let settings_asset = ron::de::from_bytes::<SettingsAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(settings_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["settings"]
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SettingsAsset>()
            .init_resource::<SettingsAsset>()
            .init_asset_loader::<SettingsLoader>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_settings));
    }
}

fn setup_settings(
    settings_assets: Res<Assets<SettingsAsset>>,
    settings_handles: Res<SettingsAssets>,
    mut settings: ResMut<SettingsAsset>,
) {
    let settings_asset = settings_assets.get(&settings_handles.default_settings).unwrap();
    settings.graphics_settings = settings_asset.graphics_settings.clone();
}
