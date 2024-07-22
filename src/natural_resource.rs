use std::{f32::INFINITY, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::{blinking_sprite::BlinkingSprite, planet::{Planet, PlanetWater}, planet_sticker::{Contains, EdgeDistanceTo, IsCollidingWith, PlanetSticker}, spawn_occupable, NewOccupable, OccupableBundle, OccupableType, ResourceType};

#[derive(Component, PartialEq)]
pub struct NaturalResource {
    pub produced_resource: ResourceType,
    pub amount_remaining: u32,
}

pub enum Biome {
    Water,
    Swamp,
    Ground
}

pub struct NaturalResourcePlugin;

impl Plugin for NaturalResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_natural_resources)
        .add_systems(Update,handle_spawning_resources.run_if(on_timer(Duration::from_secs_f32(0.))));
    }
}

fn determine_biome(planet: Entity, pos: f32, waters_query: &Query<&PlanetSticker, With<PlanetWater>>) -> Option<Biome> {
    let mut closest = INFINITY;
    for water in waters_query.iter() {
        if water.planet != Some(planet) { continue; };
        if water.contains(pos) {
            return Some(Biome::Water);
        }
        let dist = water.edge_distance_to(pos);
        if (dist < closest) {
            closest = dist;
        }
    }
    if (closest < 16.) {
        return Some(Biome::Swamp);
    }
    return Some(Biome::Ground);
}

fn handle_spawning_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    planets_query: Query<(Entity, &Planet)>,
    waters_query: Query<&PlanetSticker, With<PlanetWater>>,
    stickers_query: Query<&PlanetSticker, Without<PlanetWater>>
) {
    for (planet_entity, planet) in planets_query.iter() {
        let pos = rand::thread_rng().gen_range(0.0..360.0);
        let Some(biome) = determine_biome(planet_entity, pos, &waters_query) else { continue; };
        let mut found = false;
        for sticker in stickers_query.iter() {
            if sticker.is_colliding_with_pos(pos, 8.) {
                found = true;
                break;
            }
        }
        if found { continue; };
        match biome {
            Biome::Water => {},
            Biome::Swamp => spawn_bush(&mut commands, &asset_server, planet_entity, pos),
            Biome::Ground => spawn_tree(&mut commands, &asset_server, planet_entity, pos),
        }
    }
}

fn handle_natural_resources (
    mut commands: Commands,
    natural_resource_query: Query<(Entity, &NaturalResource)>
) {
    for (natural_resource_entity, natural_resource) in natural_resource_query.iter() {
        if (natural_resource.amount_remaining <= 0) {
            commands.entity(natural_resource_entity).despawn_recursive()
        }
    }
}

fn spawn_natural_resource(commands: &mut Commands, occupable_bundle: OccupableBundle, produced: ResourceType, amount: u32) {
    let occupable = spawn_occupable(commands, occupable_bundle);
    commands.entity(occupable).insert(
        NaturalResource { produced_resource: produced, amount_remaining: amount }
    );
    commands.entity(occupable).insert(
        BlinkingSprite { enabled: false }
    );
}


pub fn spawn_tree(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    spawn_natural_resource(
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
        ResourceType::Wood,
        20,
    );
}

pub fn spawn_bush(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    spawn_natural_resource(
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
        ResourceType::Food,
        20,
    );
}

pub fn spawn_fish(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    spawn_natural_resource(
        commands,
        OccupableBundle::new(
            asset_server.load("environment/bush.png"),
            planet,
            position_degrees,
            OccupableType::Fishing,
            ResourceType::Food,
            1,
            8.,
        ),
        ResourceType::Food,
        20,
    );
}
