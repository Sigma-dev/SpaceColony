use crate::{occupables::*, planet::PlanetWater, planet_sticker::{self, PlanetCollider, PlanetSticker}, planet_villager::{self, count_occupiers, count_workers, VillagerWandering, VillagerWorking}, Occupable};
use bevy::prelude::*;
use occupable::OccupancyChange;

#[derive(Component)]
pub struct OccupableCounter {
    pub count: i32,
    pub minus_button: Entity,
    pub plus_button: Entity,
}

pub struct OccupableCounterPlugin;

impl Plugin for OccupableCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_count, handle_counters));
    }
}

fn _handle_events(
    mut counters_query: Query<(&mut Sprite, &Parent, &mut OccupableCounter)>,
    occupables_query: Query<&occupable::Occupable>,
    worker_query: Query<&VillagerWorking>,
    mut ev_occupancy: EventReader<OccupancyChange>,
) {
    for ev in ev_occupancy.read() {
        if let Ok(_) = occupables_query.get(ev.occupable) {
            for (mut sprite, parent, mut counter) in counters_query.iter_mut() {
                if parent.get() == ev.occupable {
                    //counter.count += ev.change;
                    let count = count_workers(&worker_query, ev.occupable);
                    counter.count = count as i32;
                    sprite.texture_atlas.as_mut().unwrap().index = counter.count as usize;
                }
            }
        }
    }
}

fn handle_count(
    mut counters_query: Query<(&mut Sprite, &Parent, &mut OccupableCounter)>,
    occupables_query: Query<Entity, With<occupable::Occupable>>,
    worker_query: Query<&VillagerWorking>,
) {
    for (mut sprite, parent, mut counter) in counters_query.iter_mut() {
        if let Ok(occupable_entity) = occupables_query.get(parent.get()) {
            let count = count_workers(&worker_query, occupable_entity);
            counter.count = count as i32;
            sprite.texture_atlas.as_mut().unwrap().index = counter.count as usize;
        }
    }
}

fn handle_counters(
    mut counters_query: Query<(
        &Parent,
        &mut OccupableCounter,
        &mut Visibility,
    )>,
    occupables_query: Query<(Entity, &occupable::Occupable, &planet_sticker::PlanetSticker)>,
    mut visibility_query: Query<&mut Visibility, Without<OccupableCounter>>,
    selected_occupable: Res<occupable::SelectedOccupable>,
    wandering_query: Query<&PlanetSticker, With<VillagerWandering>>,
    working_query: Query<&VillagerWorking>,
    water_query: Query<(&PlanetSticker, &PlanetCollider), (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
) {
    for (parent, counter, visibility) in counters_query.iter_mut() {
        if let Ok((occupable_entity, occupable, occupable_sticker)) = occupables_query.get(parent.get()) {
            handle_selected(&selected_occupable, visibility, occupable_entity);
            if let Ok(mut minus_vis) = visibility_query.get_mut(counter.minus_button) {
                *minus_vis = Visibility::Inherited;
                if count_occupiers(&working_query, occupable_entity) == 0 {
                    *minus_vis = Visibility::Hidden;
                }
            }
            if let Ok(mut plus_vis) = visibility_query.get_mut(counter.plus_button) {
                *plus_vis = Visibility::Inherited;
                if count_workers(&working_query, occupable_entity) >= occupable.max_workers {
                    *plus_vis = Visibility::Hidden;
                }
                let mut found = false;
                for villager_sticker in wandering_query.iter() {
                    if villager_sticker.planet == occupable_sticker.planet {
                        if planet_villager::get_walk_dir(&villager_sticker, &water_query, occupable_sticker.position_degrees).is_some() {
                            found = true;
                        }
                    }   
                }
                if !found { 
                    *plus_vis = Visibility::Hidden; 
                }
            }
        }
    }
}

fn handle_selected(
    selected_occupable: &Res<occupable::SelectedOccupable>,
    mut counter_visibility: Mut<Visibility>,
    occupable_entity: Entity,
) {
    *counter_visibility = Visibility::Hidden;
    if let Some(selected) = selected_occupable.occupable {
        if selected == occupable_entity {
            *counter_visibility = Visibility::Visible;
        }
    }
}
