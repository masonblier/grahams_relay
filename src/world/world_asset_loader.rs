use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
// use bevy_rapier3d::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a008b74b4962"]
pub struct WorldAsset {
    pub colliders: Vec<WorldCollider>,
    pub doors: Vec<WorldDoor>,
    pub interactables: Vec<WorldInteractable>,
    pub lights: Vec<WorldLight>,
    pub props: Vec<WorldProp>,
    pub sounds: Vec<WorldSound>,
}

// represents data for convex colliders defined for a world
#[derive(Debug, Deserialize)]
pub struct WorldCollider {
    pub shape: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

// represents door
#[derive(Debug, Deserialize)]
pub struct WorldDoor {
    pub prop: String,
    pub name: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

// represents interactable target collider
#[derive(Debug, Deserialize)]
pub struct WorldInteractable {
    pub shape: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub interaction: Option<WorldInteraction>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct WorldInteraction {
    pub interaction: String,
    pub interaction_text: String,
    pub actions: Vec<(String, String)>,
    pub blockers: Vec<(String, String)>,
}

// represents gltf prop
#[derive(Debug, Deserialize)]
pub struct WorldProp {
    pub prop: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub animatable: Option<String>,
}

// represents 3d positioned sound
#[derive(Debug, Deserialize)]
pub struct WorldSound {
    pub sound: String,
    pub translation: Vec3,
    pub paused: bool,
    pub animatable: Option<String>,
}

// represents light
#[derive(Debug, Deserialize)]
pub struct WorldLight {
    pub light_type: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub watts: f32,
    pub animatable: Option<String>,
}

#[derive(Default)]
pub struct WorldAssetLoader;

impl AssetLoader for WorldAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let world_asset = ron::de::from_bytes::<WorldAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(world_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["world"]
    }
}

pub struct WorldAssetLoaderPlugin;

impl Plugin for WorldAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<WorldAsset>()
            .init_asset_loader::<WorldAssetLoader>();
    }
}
