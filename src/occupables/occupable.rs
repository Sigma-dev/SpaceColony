use std::process::Child;

use bevy::{prelude::*, render::view::visibility};
use bevy_mod_picking::prelude::*;

use crate::{
    button_value,
    occupable_counter::{self, OccupableCounter},
    planet_sticker::PlanetSticker,
    planet_villager::{PlanetVillager, VillagerWandering, VillagerWorking},
    Resources,
};

#[derive(Resource, Default)]
pub struct SelectedOccupable {
    pub occupable: Option<Entity>,
}

#[derive(PartialEq)]
pub enum OccupableType {
    Cutting,
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
    pub max_workers: i32,
    pub produced_resource: ResourceType,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ResourceType {
    Food,
    Wood,
}

pub struct OccupablePlugin;

impl Plugin for OccupablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_entity_system, handle_selected))
            .add_systems(Update, (find_and_assign_villagers))
            .add_systems(FixedUpdate, produce_resources)
            .add_systems(PostStartup, spawn_ui);
    }
}

fn spawn_ui(
    q: Query<(Entity, &Occupable)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.init_resource::<SelectedOccupable>();
    for (e, occupable) in q.iter() {
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
    mut commands: &mut Commands,
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    index: i32,
    offset: Vec3,
) -> Entity {
    return commands
        .spawn((SpriteSheetBundle {
            texture: asset_server.load("ui/symbols.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(8.0, 8.0),
                    10,
                    2,
                    None,
                    None,
                )),
                index: index as usize,
            },
            transform: Transform {
                translation: offset,
                ..Default::default()
            },
            ..default()
        },))
        .id();
}

fn spawn_counter(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
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
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
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
    villager_query: Query<(&mut PlanetVillager, &PlanetSticker)>,
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
    mut wandering_query: Query<(Entity, &mut VillagerWandering, &PlanetSticker)>,
    mut working_query: Query<(Entity, &mut VillagerWorking, &PlanetSticker)>,
    mut occupable_query: Query<(&mut Occupable, &PlanetSticker)>,
    mut commands: Commands,
) {
    for ev in ev_occupancy.read() {
        if ev.change == 1 {
            for (villager_entity, mut villager, sticker) in wandering_query.iter_mut() {
                if let Ok((mut occupable, occupable_sticker)) =
                    occupable_query.get_mut(ev.occupable)
                {
                    if sticker.planet == occupable_sticker.planet {
                        commands
                            .entity(villager_entity)
                            .remove::<VillagerWandering>()
                            .insert(VillagerWorking {
                                current_occupable: ev.occupable,
                            });
                        occupable.workers.push(villager_entity);
                        return;
                    }
                }
            }
        } else if ev.change == -1 {
            if let Ok((mut occupable, occupable_sticker)) = occupable_query.get_mut(ev.occupable) {
                if let Some(worker) = occupable.workers.last() {
                    if let Ok((villager_entity, mut villager, _)) = working_query.get_mut(*worker) {
                        commands
                            .entity(villager_entity)
                            .remove::<VillagerWorking>()
                            .insert(VillagerWandering::default());
                        occupable.workers.pop();
                        println!("Here")
                    }
                }
            }
        }
    }
}

fn handle_selected(
    mut sprite_children: Query<(&mut Visibility, &Parent)>,
    occupables: Query<&Occupable>,
    mut selected_occupable: Res<SelectedOccupable>,
) {
    //selected_occupable
    return;
    for (mut visibility, parent) in sprite_children.iter_mut() {
        let occupable = occupables.get(parent.get());
        if let Ok(valid) = occupable {
            if valid.selected {
                *visibility = Visibility::Visible
            } else {
                *visibility = Visibility::Hidden
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

fn produce_resources(
    mut working_query: Query<(Entity, &mut VillagerWorking, &PlanetSticker)>,
    mut occupable_query: Query<(&mut Occupable)>,
    mut resources: ResMut<Resources>,
) {
    for (occupable) in occupable_query.iter_mut() {
        let index = occupable.produced_resource as i32;
        let current_value = resources.stored.get(&index).copied().unwrap_or(0);
        let updated_value = current_value + occupable.workers.len() as i32;
        resources.stored.insert(index, updated_value);
    }
}
