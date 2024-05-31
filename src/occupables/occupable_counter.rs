use crate::{occupables::*, OccupancyChange};
use bevy::{prelude::*, render::view::visibility};
use num_traits::ToPrimitive;

#[derive(Component)]
pub struct OccupableCounter {
    pub count: i32
}

pub struct OccupableCounterPlugin;

impl Plugin for OccupableCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_counters);
    }
}

fn handle_counters(
    mut counters_query: Query<(&mut TextureAtlas, &Parent, &mut OccupableCounter)>,
    occupables_query: Query<&occupable::Occupable>,
    mut ev_occupancy: EventReader<OccupancyChange>,
) {
    for ev in ev_occupancy.read() {
        if let Ok(occupable) = occupables_query.get(ev.occupable) {
            for (mut atlas, parent, mut counter) in counters_query.iter_mut() {
                if parent.get() == ev.occupable {
                    counter.count += ev.change;
                    println!("{}", occupable.workers.len());
                    atlas.index = counter.count as usize;
                }
            }
        }
    }
    /* 
    for (mut atlas, parent) in counters_query.iter_mut() {
        let occupable = occupables_query.get(parent.get());
        if let Ok(valid) = occupable {
            println!("damso: {}", valid.workers.len());
            atlas.index = valid.workers.len() as usize;
        }
    }
    */
}
