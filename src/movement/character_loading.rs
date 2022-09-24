use crate::game_state::GameState;
use crate::loading::{CharacterAssets};
use crate::movement::{CharacterState,Mover,MoverParent};
use crate::settings::SettingsAsset;
use crate::world::WorldState;
use bevy::prelude::*;
use bevy::scene::InstanceId;
use bevy_rapier3d::prelude::*;

pub struct CharacterLoadingPlugin;

pub struct CharacterAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Default)]
pub struct CharacterLoadingState {
    character_scene_instance: Option<InstanceId>,
    done: bool,
}

impl Plugin for CharacterLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CharacterLoadingState>()
            .init_resource::<CharacterState>()
            .add_system_set(SystemSet::on_enter(GameState::CharacterLoading)
                .with_system(setup_character_loading)
                .with_system(setup_character_animations))
            .add_system_set(SystemSet::on_update(GameState::CharacterLoading).with_system(update_character_loading));
    }
}

fn setup_character_loading(
    mut commands: Commands,
    character_handles: Res<CharacterAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    settings: Res<SettingsAsset>,
    mut character_loading: ResMut<CharacterLoadingState>,
    mut rapier_conf: ResMut<RapierConfiguration>,
) {
    let radius = 0.5;
    let half_height = 0.54;
    commands
        .spawn_bundle(SpatialBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0,0.5,0.0))))
        .insert(Mover::default())
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule(-half_height * Vec3::Y, half_height * Vec3::Y, radius))
        .insert(CollisionGroups::new(0b0001, 0b0001))

        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Ccd::enabled())
        .insert(ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO,
        })
        .insert(Damping { linear_damping: 10.0, angular_damping: 1.0 })
        .with_children(|parent| {
            if settings.graphics_settings.render_mode.as_str() == "colliders" {
                // spawn character container
                parent.spawn_bundle(SpatialBundle::from_transform(
                    Transform::from_translation(-Vec3::Y).with_rotation(Quat::default())
                ))
                .insert(MoverParent::default())
                .with_children(|parent| {
                    // render approx capsule as two spheres
                    parent.spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::UVSphere { radius, ..Default::default() })),
                        material: materials.add(Color::rgb(0.0, 0.7, 3.0).into()),
                        transform: Transform::from_xyz(0.0, 2.*half_height, 0.0),
                        ..default()
                    });
                    parent.spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::UVSphere { radius, ..Default::default() })),
                        material: materials.add(Color::rgb(0.0, 0.7, 3.0).into()),
                        transform: Transform::from_xyz(0.0, half_height, 0.0),
                        ..default()
                    });
                });
            } else {
                // spawn character container
                let character_parent = parent.spawn_bundle(SpatialBundle::from_transform(
                    Transform::from_translation(-Vec3::Y).with_rotation(Quat::default())
                ))
                .insert(MoverParent::default())
                .id();
                // spawn character gltf
                character_loading.character_scene_instance = Some(
                scene_spawner.spawn_as_child(character_handles.graham.clone(), character_parent));
            }
        });
    character_loading.done = false;
    // pause physics
    rapier_conf.physics_pipeline_active = false;
}

fn setup_character_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(CharacterAnimations(vec![
        asset_server.load("characters/graham_4action_v2.glb#Animation4"),
        asset_server.load("characters/graham_4action_v2.glb#Animation3"),
        asset_server.load("characters/graham_4action_v2.glb#Animation2"),
        asset_server.load("characters/graham_4action_v2.glb#Animation1"),
        asset_server.load("characters/graham_4action_v2.glb#Animation0"),
    ]));
}

fn update_character_loading(
    mut character_loading: ResMut<CharacterLoadingState>,
    mut character_state: ResMut<CharacterState>,
    mut state: ResMut<State<GameState>>,
    scene_spawner: Res<SceneSpawner>,
    settings: Res<SettingsAsset>,
    world_state: Res<WorldState>,
) {
    if character_loading.done {
        return;
    }
    if settings.graphics_settings.render_mode.as_str() == "colliders" {
        character_loading.done = true;

        if world_state.active_world == "credits" {
            state.set(GameState::Credits).unwrap();
        } else if world_state.active_world == "world03" {
            state.set(GameState::World03Loading).unwrap();
        } else {
            state.set(GameState::World01Loading).unwrap();
        }

        return;
    }

    // get player scene root entity (3rd lowest entity number?)
    // todo figure out a better way
    let mut lowest_ent_01: Option<Entity> = None;
    let mut lowest_ent_02: Option<Entity> = None;
    let mut lowest_ent_03: Option<Entity> = None;
    if let Some(inst_iter) = scene_spawner.iter_instance_entities(character_loading.character_scene_instance.unwrap()) {
        for inst in inst_iter {
            if !lowest_ent_01.is_some() || inst.id() < lowest_ent_01.unwrap().id() {
                lowest_ent_03 = lowest_ent_02;
                lowest_ent_02 = lowest_ent_01;
                lowest_ent_01 = Some(inst);
            } else if !lowest_ent_02.is_some() || inst.id() < lowest_ent_02.unwrap().id() {
                lowest_ent_03 = lowest_ent_02;
                lowest_ent_02 = Some(inst);
            } else if !lowest_ent_03.is_some() || inst.id() < lowest_ent_03.unwrap().id() {
                lowest_ent_03 = Some(inst);
            }
        }
    }

    if lowest_ent_03.is_some() {
        character_state.character_anim_entity = lowest_ent_03.clone();

        info!("Character loaded: {:?}", lowest_ent_03);
        character_loading.done = true;
        if world_state.active_world == "credits" {
            state.set(GameState::Credits).unwrap();
        } else if world_state.active_world == "world03" {
            state.set(GameState::World03Loading).unwrap();
        } else {
            state.set(GameState::World01Loading).unwrap();
        }
    }
}
