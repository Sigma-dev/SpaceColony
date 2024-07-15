use crate::{occupables::*, planet_sticker::{self, PlanetSticker}, planet_villager::{PlanetVillager, VillagerWandering, VillagerWorking}};
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
        app.add_systems(Update, (handle_events, handle_counters));
    }
}

fn handle_events(
    mut counters_query: Query<(&mut TextureAtlas, &Parent, &mut OccupableCounter)>,
    occupables_query: Query<&occupable::Occupable>,
    mut ev_occupancy: EventReader<OccupancyChange>,
) {
    for ev in ev_occupancy.read() {
        if let Ok(_) = occupables_query.get(ev.occupable) {
            for (mut atlas, parent, mut counter) in counters_query.iter_mut() {
                if parent.get() == ev.occupable {
                    counter.count += ev.change;
                    atlas.index = counter.count as usize;
                }
            }
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
    villager_query: Query<&PlanetSticker, With<VillagerWandering>>
) {
    for (parent, counter, visibility) in counters_query.iter_mut() {
        if let Ok((occupable_entity, occupable, occupable_sticker)) = occupables_query.get(parent.get()) {
            handle_selected(&selected_occupable, visibility, occupable_entity);
            if let Ok(mut minus_vis) = visibility_query.get_mut(counter.minus_button) {
                *minus_vis = Visibility::Inherited;
                if occupable.workers.len() == 0 {
                    *minus_vis = Visibility::Hidden;
                }
            }
            if let Ok(mut plus_vis) = visibility_query.get_mut(counter.plus_button) {
                *plus_vis = Visibility::Inherited;
                if occupable.workers.len() as u32 >= occupable.max_workers {
                    *plus_vis = Visibility::Hidden;
                }
                for villager_sticker in villager_query.iter() {
                    let mut found = false;
                    if villager_sticker.planet == occupable_sticker.planet {
                        found = true
                    }
                    if !found { 
                        *plus_vis = Visibility::Hidden; 
                    }
                }
                if villager_query.is_empty() {
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
