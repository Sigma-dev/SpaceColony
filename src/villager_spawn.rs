use bevy::{app::*, prelude::*, utils::*};

use crate::{planet::Planets, planet_villager::spawn_villager, resources::Resources, ResourceType};

pub struct VillagerSpawnPlugin;

impl Plugin for VillagerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_spawn);
    }
}

fn handle_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut resources: ResMut<Resources>,
    planets: Res<Planets>,
) {
    let index = &(ResourceType::Food as i32);
    let current_value = resources.stored.get(index).copied().unwrap_or(0);
    if current_value > 10 {
        resources.stored.insert(*index, current_value - 10 as i32);
        if let Some(main_planet) = planets.main {
            spawn_villager(
                &mut commands,
                &asset_server,
                main_planet,
                0.,
                "spawned".to_owned(),
            )
        }
    }
}
