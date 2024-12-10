use bevy::{app::*, prelude::*};

use crate::{planet::Planets, planet_sticker::PlanetSticker, planet_villager::{spawn_villager, PlanetVillager}, resources::Resources, ResourceType};

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
    villagers_query: Query<&PlanetSticker, With<PlanetVillager>>
) {
    let index = &(ResourceType::Food as i32);
    let current_value = resources.stored.get(index).copied().unwrap_or(0);
    let cap = 10;
    if current_value >= cap {
        resources.stored.insert(*index, current_value - cap as i32);
        if let Some(main_planet) = planets.main {
            let mut pos = 0.;
            for villager in villagers_query.iter() {
                if villager.planet != planets.main.unwrap() { continue; }
                pos = villager.position_degrees.to_f32();
            }
            spawn_villager(
                &mut commands,
                &asset_server,
                main_planet,
                pos,
                "spawned".to_owned(),
            );
            return;
        }
        
    }
}
