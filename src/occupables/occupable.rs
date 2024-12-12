use bevy::{prelude::*, sprite::Anchor};

use crate::{
    button_value,
    looping_float::LoopingFloat,
    natural_resource::NaturalResource,
    occupable_counter::{self, OccupableCounter},
    planet::Planet,
    planet_placing::{BuildingType, GetBuildingInfo},
    planet_sticker::{self, PlanetSticker},
    planet_villager::*,
    scaling_sprite::ScalingSprite,
};

#[derive(Resource, Default)]
pub struct SelectedOccupable {
    pub occupable: Option<Entity>,
}

#[derive(PartialEq)]
pub enum OccupableType {
    Cutting,
    Foraging,
    Fishing,
    Interior,
}

#[derive(Event)]
pub struct OccupancyChange {
    pub occupable: Entity,
    pub change: i32,
}

#[derive(Component, PartialEq)]
pub struct Occupable {
    pub selected: bool,
    pub max_workers: u32,
    pub occupable_type: OccupableType,
}

#[derive(Component, PartialEq)]
pub struct Automator {
    pub exploited_resource: ResourceType,
    pub range: f32,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ResourceType {
    Food,
    Wood,
}

pub struct OccupableParameters {
    texture: Handle<Image>,
    planet: Entity,
    position_degrees: f32,
    occupable_type: OccupableType,
    max_workers: u32,
    size_degrees: f32,
    anchor: Anchor,
}

impl OccupableParameters {
    pub fn new(texture: Handle<Image>, planet: Entity, position_degrees: f32, occupable_type: OccupableType, max_workers: u32, size_degrees: f32, anchor: Anchor) -> OccupableParameters {
        OccupableParameters {
            texture,
            planet,
            position_degrees,
            occupable_type,
            max_workers,
            size_degrees,
            anchor
        }
    }
}

pub struct OccupablePlugin;

impl Plugin for OccupablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_entity_system)
            .add_systems(Update, find_and_assign_villagers)
            .add_systems(Update, spawn_ui)
            .add_systems(Update, handle_automators)
            .insert_resource(SelectedOccupable::default())
            .add_event::<OccupancyChange>();
    }
}

fn spawn_ui(
    q: Query<Entity, (With<Occupable>, Without<Children>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for e in q.iter() {
        let minus = spawn_button(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            true,
        );
        let plus = spawn_button(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            false,
        );
        let counter = spawn_counter(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            minus,
            plus,
        );
        commands.entity(counter).add_child(minus);
        commands.entity(counter).add_child(plus);
        commands.entity(e).add_child(counter);
    }
}

fn spawn_symbol(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    index: i32,
    offset: Vec3,
) -> Entity {
    return commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("ui/symbols.png"),
            TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(8, 8),
                    10,
                    2,
                    None,
                    None,
                )),
                index: index as usize,
            },
        ),
        Transform::from_translation(offset),
        Name::new("Symbol")
    )).id();
}

fn spawn_counter(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    minus: Entity,
    plus: Entity,
) -> Entity {
    let counter = spawn_symbol(
        commands,
        texture_atlas_layouts,
        asset_server,
        0,
        Vec3 {
            x: 0.,
            y: 24.,
            z: 0.,
        },
    );
    commands
        .entity(counter)
        .insert(occupable_counter::OccupableCounter {
            count: 0,
            minus_button: minus,
            plus_button: plus,
        });
    return counter;
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    minus: bool,
) -> Entity {
    let offset = Vec3 {
        x: 16. * if minus { -1. } else { 1. },
        y: 0.,
        z: 0.,
    };
    let index = if minus { 11 } else { 10 };
    let button = spawn_symbol(commands, texture_atlas_layouts, asset_server, index, offset);
    commands.entity(button).insert(button_value::Buttonvalue {
        value: if minus { -1 } else { 1 },
    });
    commands
        .entity(button)
        .observe(change_value);
    return button;
}

fn change_value(
    event: Trigger<Pointer<Click>>,
    mut ev_occupancy: EventWriter<OccupancyChange>,
    button_query: Query<(&button_value::Buttonvalue, &Parent)>,
    counter_query: Query<&Parent, With<OccupableCounter>>,
    occupable_query: Query<Entity, With<Occupable>>,
) {
    let Ok((button, parent)) = button_query.get(event.target) else {
        return;
    };
    let Ok(counter_entity) = counter_query.get(parent.get()) else {
        return;
    };
    let Ok(entity) = occupable_query.get(counter_entity.get()) else {
        return;
    };

    ev_occupancy.send(OccupancyChange {
        occupable: entity,
        change: button.value,
    });
}

fn find_and_assign_villagers(
    mut ev_occupancy: EventReader<OccupancyChange>,
    mut wandering_query: Query<(Entity, &PlanetSticker), With<VillagerWandering>>,
    mut working_query: Query<(Entity, &VillagerWorking)>,
    mut occupable_query: Query<(Entity, &PlanetSticker)>,
    mut commands: Commands,
) {
    for ev in ev_occupancy.read() {
        if ev.change == 1 {
            for (villager_entity, sticker) in wandering_query.iter_mut() {
                if let Ok((_, occupable_sticker)) = occupable_query.get(ev.occupable) {
                    if sticker.planet == occupable_sticker.planet {
                        commands
                            .entity(villager_entity)
                            .remove::<VillagerWandering>()
                            .insert(VillagerWorking {
                                current_occupable: ev.occupable,
                                current_work: ev.occupable,
                                production_interval: 1.0,
                            });
                        return;
                    }
                }
            }
        } else if ev.change == -1 {
            if let Ok((occupable_entity, _)) = occupable_query.get_mut(ev.occupable) {
                for (worker_entity, worker) in working_query.iter_mut() {
                    if worker.current_occupable != occupable_entity {
                        continue;
                    };
                    commands
                        .entity(worker_entity)
                        .remove::<VillagerWorking>()
                        .insert(VillagerWandering::default());
                    return;
                }
            }
        }
    }
}

fn handle_automators(
    planets_query: Query<&Planet>,
    automator_query: Query<(Entity, &Automator, &Occupable, &PlanetSticker)>,
    mut natural_resource_query: Query<
        (Entity, &NaturalResource, &mut Occupable, &PlanetSticker),
        Without<Automator>,
    >,
    mut villager_query: Query<(Entity, &mut VillagerWorking)>,
) {
    /* for (automator_entity, automator, _automator_occupable, automator_sticker) in
        automator_query.iter()
    {
        let mut free: Vec<Entity> = vec![];
        for (villager_entity, villager) in villager_query.iter() {
            if villager.current_work == automator_entity {
                free.push(villager_entity);
            }
        }
        for (occupable_entity, natural_resource, occupable, occupable_sticker) in
            natural_resource_query.iter_mut()
        {
            if free.is_empty() {
                continue;
            }
            if automator_entity == occupable_entity {
                continue;
            }
            let mut count = 0;

            for (_, villager) in villager_query.iter() {
                if villager.current_work == occupable_entity {
                    count += 1;
                }
            }
            if count == occupable.max_workers {
                continue;
            };
            let Ok(planet) = planets_query.get(automator_sticker.planet) else {
                continue;
            };
            let dist: f32 = automator_sticker
                .position_degrees
                .arc_distance(occupable_sticker.position_degrees.to_f32(), planet.radius);
            if dist > automator.range {
                continue;
            }
            if automator.exploited_resource != natural_resource.produced_resource {
                continue;
            };
            let Some(villager_entity) = free.last() else {
                continue;
            };
            let Ok((_, mut villager)) = villager_query.get_mut(*villager_entity) else {
                continue;
            };
            villager.current_work = occupable_entity;
            free.pop();
        }
    } */
}

fn select_entity_system(
    mut events: EventReader<Pointer<Click>>,
    mut selected_occuppable: ResMut<SelectedOccupable>,
    query: Query<Entity, With<Occupable>>,
) {
    for event in events.read() {
        if query.get(event.target).is_ok() {
            selected_occuppable.occupable = Some(event.target);
        }
    }
}

pub fn spawn_occupable(commands: &mut Commands, occupable: OccupableParameters) -> Entity {
    let created = commands.spawn((
        Sprite {
            image: occupable.texture,
            anchor: occupable.anchor,
            ..default()
        },
        Transform {
            scale: Vec3::ZERO,
            ..default()
        },
        planet_sticker::PlanetSticker {
            planet: occupable.planet,
            position_degrees: LoopingFloat::new(occupable.position_degrees),
        },
        planet_sticker::PlanetCollider {
            size_degrees: occupable.size_degrees,
        },
        Occupable {
            occupable_type: occupable.occupable_type,
            selected: false,
            max_workers: occupable.max_workers,
        },
        ScalingSprite {
            target_scale: Vec3::ONE,
        },
        Name::new("Occupable"),
    )).id();
    commands.entity(created).observe(|trigger: Trigger<Pointer<Click>>, mut occupables: Query<&mut Occupable> | 
        occupables.get_mut(trigger.entity()).unwrap().selected = true
    );
    created
}

fn spawn_automator(
    commands: &mut Commands,
    occupable_parameters: OccupableParameters,
    range: f32,
    exploited_resource: ResourceType,
) {
    let occupable = spawn_occupable(commands, occupable_parameters);
    commands.entity(occupable).insert(Automator {
        exploited_resource,
        range,
    });
}

pub fn spawn_building(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
    building_type: BuildingType,
) {
    /* let texture_path = match building_type {
        BuildingType::Sawmill => "buildings/sawmill.png",
    };
    let info = building_type.get_building_info();
    spawn_automator(
        commands,
        OccupableParameters::new(
            asset_server.load(texture_path),
            planet,
            position_degrees,
            OccupableType::Interior,
            3,
            16.,
            Anchor::BottomCenter,
        ),
        info.range,
        info.exploited_resource,
    ); */
}
