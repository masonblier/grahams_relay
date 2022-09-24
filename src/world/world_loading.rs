use crate::loading::{LoadingUiEvent,LoadingUiEventAction,WorldAssets,
    WorldProps,World01Props,World03Props};
use crate::game_state::GameState;
use crate::movement::Mover;
use crate::settings::SettingsAsset;
use crate::world::{DoorState,InteractableState,WorldAsset,WorldState,
    WorldSoundState,WorldTrainState,AnimatableState};
use bevy::prelude::*;
use bevy::scene::InstanceId;
use bevy_rapier3d::prelude::*;
use std::collections::HashMap;

pub struct WorldLoadingPlugin;

#[derive(Default)]
pub struct WorldLoadingState {
    animatable_scenes: HashMap<String, InstanceId>,
    inited: bool,
    done: bool,
}

// component to tag unloadable world items
#[derive(Component,Default)]
pub struct WorldEntity;

impl Plugin for WorldLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldLoadingState>()
            .init_resource::<WorldState>()
            .add_system_set(SystemSet::on_enter(GameState::WorldInit).with_system(setup_world_init))
            .add_system_set(SystemSet::on_update(GameState::WorldInit).with_system(update_world_init))
            .add_system_set(SystemSet::on_enter(GameState::WorldLoading).with_system(setup_world_loading))
            .add_system_set(SystemSet::on_update(GameState::WorldLoading).with_system(update_world_loading));
    }
}

fn setup_world_init(
    mut commands: Commands,
    mut world_loading: ResMut<WorldLoadingState>,
    mut loading_ui_events: EventWriter<LoadingUiEvent>,
    world_ents: Query<(Entity,With<WorldEntity>,Without<Mover>)>,
    mover_ents: Query<(Entity,With<Mover>,Without<WorldEntity>)>,
) {
    world_loading.inited = false;

    // update loading ui text
    loading_ui_events.send(LoadingUiEvent {
        action: LoadingUiEventAction::SetText,
        payload: Some("Spawning".into()),
    });

    // clear any previous entities
    for (ent, _, _) in world_ents.iter() {
        commands.entity(ent).despawn_recursive();
    }
    for (ent, _, _) in mover_ents.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn update_world_init(
    mut state: ResMut<State<GameState>>,
    mut world_loading: ResMut<WorldLoadingState>,
    world_state: Res<WorldState>,
) {
    if !world_loading.inited {
        world_loading.inited = true;
        if world_state.active_world == "credits" {
            state.set(GameState::Credits).unwrap();
        } else {
            state.set(GameState::CharacterLoading).unwrap();
        }
    }
}

fn setup_world_loading(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    world_assets: Res<Assets<WorldAsset>>,
    world_props: Res<WorldProps>,
    world01_props: Res<World01Props>,
    world03_props: Res<World03Props>,
    world_handles: Res<WorldAssets>,
    mut world_loading: ResMut<WorldLoadingState>,
    mut world_state: ResMut<WorldState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<SettingsAsset>,
) {
    world_loading.done = false;

    // reset state
    world_loading.animatable_scenes = HashMap::new();
    world_state.interactable_states = HashMap::new();
    world_state.animatables = HashMap::new();
    world_state.animatable_lights = HashMap::new();
    world_state.animatable_sounds = HashMap::new();
    world_state.animatable_trains = HashMap::new();
    world_state.doors = HashMap::new();
    world_state.active_train = None;

    let world_asset = if world_state.active_world == "world03" {
        world_assets.get(&world_handles.world03).unwrap()
    } else {
        world_assets.get(&world_handles.world01).unwrap()
    };

    // load props
    for data in world_asset.props.iter() {
        let prop_handle: Option<Handle<Scene>> = match data.prop.as_str() {
            "big_switch" => Some(world_props.big_switch.clone()),
            "bottle_lightfuel" => Some(world_props.bottle_lightfuel.clone()),
            "cardboard_closed" => Some(world01_props.cardboard_closed.clone()),
            "cardboard_opened" => Some(world01_props.cardboard_opened.clone()),
            "cardboard_tube" => Some(world01_props.cardboard_tube.clone()),
            "city_fence" => Some(world_props.city_fence.clone()),
            "diesel_generator" => Some(world_props.diesel_generator.clone()),
            "door_blue" => Some(world_props.door_blue.clone()),
            "fountain_round" => Some(world01_props.fountain_round.clone()),
            "fuse_console" => Some(world_props.fuse_console.clone()),
            "fuse_small" => Some(world_props.fuse_small.clone()),
            "house_roof01" => Some(world01_props.house_roof01.clone()),
            "house_woodside" => Some(world01_props.house_woodside.clone()),
            "office_table" => Some(world01_props.office_desk01.clone()),
            "office_desk" => Some(world01_props.office_desk02.clone()),
            "office_chair" => Some(world01_props.office_chair.clone()),
            "pallet" => Some(world01_props.pallet.clone()),
            "rail_track" => Some(world_props.rail_track.clone()),
            "train_wire" => Some(world_props.train_wire.clone()),
            "tunnel_entrance" => Some(world_props.tunnel_entrance.clone()),
            "world01_building01" => Some(world01_props.world01_building01.clone()),
            "world01_generator_wire" => Some(world_props.world01_generator_wire.clone()),
            "world01_ground01" => Some(world01_props.world01_ground01.clone()),
            "world01_ground02" => Some(world_props.world01_ground02.clone()),
            "world01_ground03" => Some(world_props.world01_ground03.clone()),
            "refinery_column01" => Some(world03_props.refinery_column01.clone()),
            "refinery_desalter" => Some(world03_props.refinery_desalter.clone()),
            "refinery_scaffolding" => Some(world03_props.refinery_scaffolding.clone()),
            "refinery_sphere" => Some(world03_props.refinery_sphere.clone()),
            "refinery_tank01" => Some(world03_props.refinery_tank01.clone()),
            "refinery_warmer" => Some(world03_props.refinery_warmer.clone()),
            "world03_ground" => Some(world03_props.world03_ground.clone()),
            "world03_pipes" => Some(world03_props.world03_pipes.clone()),
            "world03_walkways" => Some(world03_props.world03_walkways.clone()),
            _ => {
                println!("Unknown prop! {:?}", data);
                None
            }
        };
        let mut prop_instance: Option<InstanceId> = None;
        if prop_handle.is_some() {
            commands.spawn_bundle(SpatialBundle::from_transform(
                Transform::from_translation(data.translation)
            ))
            .insert(WorldEntity)
            .with_children(|parent2| {
                let parent = parent2.spawn_bundle(SpatialBundle::from_transform(
                    Transform::from_rotation(data.rotation)
                )).id();
                if settings.graphics_settings.render_mode.as_str() != "colliders" {
                    prop_instance = Some(scene_spawner.spawn_as_child(prop_handle.unwrap(), parent));
                }
            });
        }
        if data.animatable.is_some() {
            if settings.graphics_settings.render_mode.as_str() != "colliders" {
                // store animation scene spawner reference
                world_loading.animatable_scenes.insert(data.animatable.clone().unwrap(), prop_instance.unwrap());
            }
        }
    }

    // load colliders
    for data in world_asset.colliders.iter() {
        let shape_handle: Option<Collider> = match data.shape.as_str() {
            "cuboid" => Some(Collider::cuboid(data.scale[0],data.scale[1],data.scale[2])),
            _ => None
        };
        if shape_handle.is_some() {
            commands
                    .spawn_bundle(SpatialBundle::from_transform(
                        Transform::from_translation(data.translation)))
                    .insert(shape_handle.unwrap())
                    .insert(CollisionGroups::new(0b0001, 0b0001))
                    .insert(WorldEntity)
                    .with_children(|parent| {
                        if settings.graphics_settings.render_mode.as_str() == "colliders" {
                            parent.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                transform: Transform::from_scale(data.scale*2.0),
                                ..default()
                            });
                        }
                    })
                    ;

        }
    }

    // doors
    for data in world_asset.doors.iter() {
        let prop_handle: Option<Handle<Scene>> = match data.prop.as_str() {
            "door_blue" => Some(world_props.door_blue.clone()),
            _ => None
        };
        let mut prop_instance: Option<InstanceId> = None;
        if prop_handle.is_some() {
            let parent_entity = commands.spawn_bundle(SpatialBundle::from_transform(
                Transform::from_translation(data.translation)
            ))
            .insert(WorldEntity)
            .with_children(|parent2| {
                let parent = parent2.spawn_bundle(SpatialBundle::from_transform(
                    Transform::from_rotation(data.rotation)
                )).id();
                if settings.graphics_settings.render_mode.as_str() != "colliders" {
                    prop_instance = Some(scene_spawner.spawn_as_child(prop_handle.unwrap(), parent));
                }

                let door_scale = data.scale * Vec3::new(0.8, 1.5, 0.05);
                let shape_handle: Option<Collider> = match data.prop.as_str() {
                    "door_blue" => Some(Collider::cuboid(door_scale[0],door_scale[1],door_scale[2])),
                    _ => None
                };
                if shape_handle.is_some() {
                    parent2
                            .spawn_bundle(SpatialBundle::from_transform(
                                Transform::from_translation(door_scale[1] * Vec3::Y)))
                            .insert(shape_handle.unwrap())
                            .insert(CollisionGroups::new(0b0001, 0b0001))

                            .with_children(|parent| {
                                if settings.graphics_settings.render_mode.as_str() == "colliders" {
                                    parent.spawn_bundle(PbrBundle {
                                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                                        material: materials.add(Color::rgb(0.5, 0.1, 0.1).into()),
                                        transform: Transform::from_scale(door_scale*2.0),
                                        ..default()
                                    });
                                }
                            })
                            ;
                }
            }).id();
            world_state.doors.insert(data.name.to_string(), DoorState { parent_entity: Some(parent_entity), open: false });
        }
    }

    // interactables
    for data in world_asset.interactables.iter() {
        if data.interaction.is_some() {
            let collider = Collider::ball(data.scale[0]);
            let collider_ent_id = commands
                    .spawn_bundle(SpatialBundle::from_transform(
                        Transform::from_translation(data.translation)))
                    .insert(collider)
                    .insert(CollisionGroups::new(0b0100, 0b0100))
                    .insert(Sensor {})
                    .insert(WorldEntity)
                    .with_children(|parent| {
                        if settings.graphics_settings.render_mode.as_str() == "colliders" {
                            parent.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: data.scale[0], ..Default::default() })),
                                material: materials.add(Color::rgb(0.0, 0.7, 0.6).into()),
                                ..default()
                            });
                        }
                    })
                    .id();
            // todo store interaction type, collider_ent_id, etc
            world_state.interactable_states.insert(collider_ent_id, InteractableState { interaction: data.interaction.clone().unwrap() });
        } else {
            println!("unknown interactable :: {:?}", data);
        }
    }

    // load lights
    for data in world_asset.lights.iter() {
        let light_entity = if data.light_type == "spot" {
            commands.spawn_bundle(DirectionalLightBundle {
                transform: Transform::from_translation(data.translation),
                directional_light: DirectionalLight {
                    illuminance: data.watts,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            })
            .insert(WorldEntity)
            .id()
        } else {
            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(data.translation),
                point_light: PointLight {
                    intensity: data.watts,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            })
            .insert(WorldEntity)
            .id()
        };
        if data.animatable.is_some() {
            world_state.animatable_lights.insert(data.animatable.clone().unwrap(), light_entity);
        }
    }


    // load sounds
    for data in world_asset.sounds.iter() {
        if data.animatable.is_some() {
            world_state.animatable_sounds.insert(data.animatable.clone().unwrap(), WorldSoundState {
                sound: data.sound.clone(),
                position: data.translation,
                panning: 0.5,
                volume: 0.5,
                paused: data.paused,
            });
        }
    }

    // trains
    // "denki_train" => Some(world_props.denki_train.clone()),
    for data in world_asset.trains.iter() {
        let train_handle = Some(commands
            .spawn_bundle(SpatialBundle::from_transform(
                Transform::from_translation(data.translation)))
                .insert(WorldEntity)
                .with_children(|parent| {
                    let world_handle: Option<Handle<WorldAsset>> = match data.prop.as_str() {
                        "denki_train" => Some(world_handles.denki_train.clone()),
                        _ => None
                    };
                    let train_asset = world_assets.get(&world_handle.unwrap()).unwrap();
                    // colliders
                    for data in train_asset.colliders.iter() {
                        let shape_handle: Option<Collider> = match data.shape.as_str() {
                            "cuboid" => Some(Collider::cuboid(data.scale[0],data.scale[1],data.scale[2])),
                            _ => None
                        };
                        if shape_handle.is_some() {
                            parent
                                    .spawn_bundle(SpatialBundle::from_transform(
                                        Transform::from_translation(data.translation)))
                                    .insert(shape_handle.unwrap())
                                    .insert(CollisionGroups::new(0b0001, 0b0001))

                                    .with_children(|parent| {
                                        if settings.graphics_settings.render_mode.as_str() == "colliders" {
                                            parent.spawn_bundle(PbrBundle {
                                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                                                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                                transform: Transform::from_scale(data.scale*2.0),
                                                ..default()
                                            });
                                        }
                                    })
                                    ;

                        }
                    }
                    // gltf props
                    for data in train_asset.props.iter() {
                        let mut prop_instance: Option<InstanceId> = None;
                        let prop_handle: Option<Handle<Scene>> = match data.prop.as_str() {
                            "denki_train" => Some(world_props.denki_train.clone()),
                            _ => None
                        };
                        if prop_handle.is_some() {
                            parent.spawn_bundle(SpatialBundle::from_transform(
                                Transform::from_translation(data.translation)
                            )).with_children(|parent2| {
                                let parent = parent2.spawn_bundle(SpatialBundle::from_transform(
                                    Transform::from_rotation(data.rotation)
                                )).id();
                                if settings.graphics_settings.render_mode.as_str() != "colliders" {
                                    prop_instance = Some(scene_spawner.spawn_as_child(prop_handle.unwrap(), parent));
                                }
                            });
                        }
                    }
                    // lights
                    for data in train_asset.lights.iter() {
                        let light_entity = if data.light_type == "spot" {
                            parent.spawn_bundle(DirectionalLightBundle {
                                transform: Transform::from_translation(data.translation),
                                directional_light: DirectionalLight {
                                    illuminance: data.watts,
                                    shadows_enabled: true,
                                    ..default()
                                },
                                ..default()
                            }).id()
                        } else {
                            parent.spawn_bundle(PointLightBundle {
                                transform: Transform::from_translation(data.translation),
                                point_light: PointLight {
                                    intensity: data.watts,
                                    shadows_enabled: true,
                                    ..default()
                                },
                                ..default()
                            }).id()
                        };
                        if data.animatable.is_some() {
                            world_state.animatable_lights.insert(data.animatable.clone().unwrap(), light_entity);
                        }
                    }
                    // interactables
                    for data in train_asset.interactables.iter() {
                        if data.interaction.is_some() {
                            let collider = Collider::ball(data.scale[0]);
                            let cgroup = if data.interaction.as_ref().unwrap().interaction == "click" {
                                CollisionGroups::new(0b0100, 0b0100)
                            } else {
                                CollisionGroups::new(0b0010, 0b0010)
                            };
                            let collider_ent_id = parent
                                    .spawn_bundle(SpatialBundle::from_transform(
                                        Transform::from_translation(data.translation)))
                                    .insert(collider)
                                    .insert(cgroup)
                                    .insert(Sensor {})
                                    .with_children(|parent| {
                                        if settings.graphics_settings.render_mode.as_str() == "colliders" {
                                            parent.spawn_bundle(PbrBundle {
                                                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: data.scale[0], ..Default::default() })),
                                                material: materials.add(Color::rgb(0.0, 0.7, 0.6).into()),
                                                ..default()
                                            });
                                        }
                                    })
                                    .id();
                            // todo store interaction type, collider_ent_id, etc
                            world_state.interactable_states.insert(collider_ent_id, InteractableState { interaction: data.interaction.clone().unwrap() });
                        } else {
                            println!("unknown interactable :: {:?}", data);
                        }
                    }
                })
                .id());
            if data.animatable.is_some() {
                // store animation scene spawner reference
                world_state.animatable_trains.insert(data.animatable.clone().unwrap(), WorldTrainState {
                    parent_entity: train_handle,
                    running: false,
                });
            }
    }
}

fn update_world_loading(
    mut world_loading: ResMut<WorldLoadingState>,
    mut world_state: ResMut<WorldState>,
    mut state: ResMut<State<GameState>>,
    scene_spawner: Res<SceneSpawner>,
    asset_server: Res<AssetServer>,
    mut loading_ui_events: EventWriter<LoadingUiEvent>,
    mut rapier_conf: ResMut<RapierConfiguration>,
) {
    if world_loading.done {
        return;
    }

    // get animation keys not yet loaded
    let waiting_keys = world_loading.animatable_scenes.keys().into_iter().filter(|&anim_key| {
        !world_state.animatables.contains_key(anim_key)
    }).collect::<Vec<&String>>();

    // if no waiting keys, all done
    if waiting_keys.len() == 0 {
        info!("World loaded: {:?}", 1);

        // hide loading ui
        loading_ui_events.send(LoadingUiEvent {
            action: LoadingUiEventAction::Hide,
            payload: None,
        });

        world_loading.done = true;
        state.set(GameState::Running).unwrap();

        // resume physics
        rapier_conf.physics_pipeline_active = true;
    } else {
        // check for waiting loaded scenes
        for waiting_key in waiting_keys {
            let mut lowest_ent: Option<Entity> = None;
            if let Some(inst_iter) = scene_spawner.iter_instance_entities(*world_loading.animatable_scenes.get(waiting_key).unwrap()) {
                for inst in inst_iter {
                    if !lowest_ent.is_some() || inst.id() < lowest_ent.unwrap().id() {
                        lowest_ent = Some(inst);
                    }
                }
            }
            if lowest_ent.is_some() {
                let clips = if waiting_key.starts_with("switch") {
                    vec![
                        asset_server.load("props/big_switch.glb#Animation3"),
                    ]
                } else { Vec::new() };
                world_state.animatables.insert(waiting_key.to_string(), AnimatableState { scene_entity: lowest_ent.clone(), clips });
            }
        }
    }
}
