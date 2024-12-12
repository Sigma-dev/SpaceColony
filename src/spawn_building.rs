use bevy::{prelude::*, sprite::*};

use crate::{planet_placing::{BuildingType, GetBuildingInfo}, planet_queries::PlanetQueries, planet_sticker::{PlanetCollider, PlanetSticker}, scaling_sprite::ScalingSprite};

#[derive(Event)]
pub struct SpawnBuilding {
    building: BuildingType,
    planet: Entity,
    position: f32,
}

impl SpawnBuilding {
    pub fn new(building: BuildingType, planet: Entity, position: f32) -> SpawnBuilding {
        SpawnBuilding { building, planet, position }
    }
}

pub struct SpawnBuildingPlugin;

impl Plugin for SpawnBuildingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SpawnBuilding>()
        .add_systems(Update, handle_placing);
    }
}

fn handle_placing(
    asset_server: Res<AssetServer>,
    mut planet_queries: PlanetQueries,
    mut events: EventReader<SpawnBuilding>,
) {
    for event in events.read() {
        let info = event.building.get_building_info();
        let sticker = PlanetSticker::new(event.planet, event.position);
        let collider = PlanetCollider::new(info.size);
        let bundle = (
            Sprite {
                image: asset_server.load(info.image_path),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform {
                scale: Vec3::ZERO,
                ..default()
            },
            ScalingSprite {
                target_scale: Vec3::ONE,
            },
            Name::new("Sawmill"),
            event.building.get_bundle(),
        );
        if planet_queries.try_place(bundle, sticker, collider).is_ok() {
            planet_queries.remove_resources(event.planet, info.cost)
        }
    }
}

