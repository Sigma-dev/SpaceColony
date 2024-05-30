use crate::occupables::*;
use bevy::{prelude::*, render::view::visibility};
use num_traits::ToPrimitive;

#[derive(Component)]
pub struct OccupableCounter;


pub struct OccupableCounterPlugin;

impl Plugin for OccupableCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_counters);
    }
}

fn handle_counters(
    mut counters_query: Query<(&mut TextureAtlas, &Parent), With<OccupableCounter>>,
    occupables_query:  Query<&occupable::Occupable>
) {
    for (mut atlas, parent) in counters_query.iter_mut() {
        let occupable = occupables_query.get(parent.get());
        if let Ok(valid) = occupable {
            atlas.index = valid.workers.len() as usize;
        }
    }
}