use std::process::Child;

use bevy::{prelude::*, render::view::visibility};
use bevy_mod_picking::prelude::*;

use crate::{
    button_value, occupable_counter, planet_sticker::PlanetSticker,
    planet_villager::PlanetVillager, OccupancyChange,
};

#[derive(PartialEq)]
pub enum OccupableType {
    Cutting,
    Interior,
}

#[derive(Component, PartialEq)]
pub struct Occupable {
    pub selected: bool,
    pub workers: Vec<Entity>,
    pub occupable_type: OccupableType,
}

pub struct OccupablePlugin;

impl Plugin for OccupablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_selected)
            .add_systems(Update, find_and_assign_villagers)
            .add_systems(PostStartup, spawn_ui);
    }
}

fn spawn_ui(
    q: Query<(Entity, &Occupable)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Startup");

    for (e, occupable) in q.iter() {
        println!("entity");

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
        let counter = spawn_counter(&mut commands, &asset_server, &mut texture_atlas_layouts);
        commands.entity(e).add_child(minus);
        commands.entity(e).add_child(plus);
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
            visibility: Visibility::Hidden,
            ..default()
        },))
        .id();
}

fn spawn_counter(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
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
        .insert(occupable_counter::OccupableCounter { count: 0 });
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
        y: 24.,
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
    occupable_query: Query<(&Occupable, &PlanetSticker, Entity)>,
) {
    println!("{}", villager_query.iter().count());
    let Ok((button, parent)) = button_query.get(event.target) else {
        return;
    };
    let Ok((occupable, sticker, entity)) = occupable_query.get(parent.get()) else {
        return;
    };
    ev_occupancy.send(OccupancyChange {
        occupable: entity,
        change: button.value,
    });
    /*let change = button.value;
    let new: i32 = occupable.workers.len() as i32 + change;
    if change == 1 {
        find_and_assign_villager(&entity, sticker, villager_query);
    } */
    //if new < 0 || new > 9 { return; }
    //occupable.number_of_workers = new;
}

fn find_and_assign_villagers(
    mut ev_occupancy: EventReader<OccupancyChange>,
    mut villager_query: Query<(&mut PlanetVillager, &PlanetSticker)>,
    occupable_query: Query<&PlanetSticker, With<Occupable>>,
) {
    for ev in ev_occupancy.read() {
        for (mut villager, sticker) in villager_query.iter_mut() {
            if let Ok(occupable_sticker) = occupable_query.get(ev.occupable) {
                if sticker.planet == occupable_sticker.planet && villager.current_occupable == None {
                    villager.current_occupable = Some(ev.occupable);
                }
            }
        }
    }
}

fn handle_selected(
    mut sprite_children: Query<(&mut Visibility, &Parent)>,
    occupables: Query<&Occupable>,
) {
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
