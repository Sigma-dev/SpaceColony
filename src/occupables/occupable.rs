use std::process::Child;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    button_value,
    looping_float::LoopingFloat,
    occupable_counter::{self, OccupableCounter},
    planet_sticker::{self, PlanetSticker},
    planet_villager::*,
    resources,
};

#[derive(Resource, Default)]
pub struct SelectedOccupable {
    pub occupable: Option<Entity>,
}

#[derive(PartialEq)]
pub enum OccupableType {
    Cutting,
    Foraging,
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
    pub workers: Vec<Entity>,
    pub occupable_type: OccupableType,
    pub max_workers: u32,
    pub produced_resource: ResourceType,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ResourceType {
    Food,
    Wood,
}

#[derive(Bundle)]
pub struct OccupableBundle {
    sprite_bundle: SpriteBundle,
    planet_sticker: PlanetSticker,
    occupable: Occupable,
}

pub trait NewOccupable {
    fn new(
        texture: Handle<Image>,
        planet: Entity,
        position_degrees: f32,
        occupable_type: OccupableType,
        produced_resource: ResourceType,
        max_workers: u32,
        size_degrees: f32,
    ) -> Self;
}

impl NewOccupable for OccupableBundle {
    fn new(
        texture: Handle<Image>,
        planet: Entity,
        position_degrees: f32,
        occupable_type: OccupableType,
        produced_resource: ResourceType,
        max_workers: u32,
        size_degrees: f32,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..default()
                },
                texture,
                ..default()
            },
            planet_sticker: planet_sticker::PlanetSticker {
                planet: Some(planet),
                position_degrees: LoopingFloat::new(position_degrees),
                size_degrees: Some(size_degrees),
            },
            occupable: Occupable {
                selected: false,
                workers: Vec::new(),
                max_workers,
                occupable_type,
                produced_resource,
            },
        }
    }
}

pub struct OccupablePlugin;

impl Plugin for OccupablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_entity_system)
            .add_systems(Update, find_and_assign_villagers)
            .add_systems(Update, spawn_ui)
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
    return commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("ui/symbols.png"),
                transform: Transform {
                    translation: offset,
                    ..Default::default()
                },
                ..default()
            },
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
        ))
        .id();
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
        .insert(On::<Pointer<Click>>::run(change_value));
    return button;
}

fn change_value(
    mut ev_occupancy: EventWriter<OccupancyChange>,
    event: Listener<Pointer<Click>>,
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
    mut working_query: Query<Entity, With<VillagerWorking>>,
    mut occupable_query: Query<(&mut Occupable, &PlanetSticker)>,
    mut commands: Commands,
) {
    for ev in ev_occupancy.read() {
        if ev.change == 1 {
            for (villager_entity, sticker) in wandering_query.iter_mut() {
                if let Ok((mut occupable, occupable_sticker)) =
                    occupable_query.get_mut(ev.occupable)
                {
                    if sticker.planet == occupable_sticker.planet {
                        commands
                            .entity(villager_entity)
                            .remove::<VillagerWandering>()
                            .insert(VillagerWorking {
                                current_occupable: ev.occupable,
                                production_interval: 1.0,
                            });
                        occupable.workers.push(villager_entity);
                        return;
                    }
                }
            }
        } else if ev.change == -1 {
            if let Ok((mut occupable, _)) = occupable_query.get_mut(ev.occupable) {
                if let Some(worker) = occupable.workers.last() {
                    if let Ok(villager_entity) = working_query.get_mut(*worker) {
                        commands
                            .entity(villager_entity)
                            .remove::<VillagerWorking>()
                            .insert(VillagerWandering::default());
                        occupable.workers.pop();
                    }
                }
            }
        }
    }
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

fn spawn_occupable(commands: &mut Commands, occupable: OccupableBundle) {
    commands.spawn((
        occupable,
        On::<Pointer<Click>>::target_component_mut::<Occupable>(|_, occupable| {
            occupable.selected = true
        }),
    ));
}

pub fn spawn_tree(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    spawn_occupable(
        commands,
        OccupableBundle::new(
            asset_server.load("environment/tree.png"),
            planet,
            position_degrees,
            OccupableType::Cutting,
            ResourceType::Wood,
            1,
            8.,
        ),
    );
}

pub fn spawn_bush(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    spawn_occupable(
        commands,
        OccupableBundle::new(
            asset_server.load("environment/bush.png"),
            planet,
            position_degrees,
            OccupableType::Foraging,
            ResourceType::Food,
            1,
            8.,
        ),
    );
}

pub fn spawn_sawmill(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    spawn_occupable(
        commands,
        OccupableBundle::new(
            asset_server.load("buildings/sawmill.png"),
            planet,
            position_degrees,
            OccupableType::Interior,
            ResourceType::Wood,
            3,
            16.,
        ),
    );
}
