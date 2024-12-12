use std::{f32::INFINITY, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::{blinking_sprite::BlinkingSprite, planet::{Planet, PlanetWater}, planet_sticker::{PlanetSticker}, scaling_sprite::ScalingSprite, spawn_occupable, Occupable, OccupableParameters, OccupableType, ResourceType};

#[derive(Component, PartialEq)]
pub struct NaturalResource {
    pub produced_resource: ResourceType,
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
        .add_systems(Update,handle_spawning_resources.run_if(on_timer(Duration::from_secs_f32(500.))));
    }
}

fn determine_biome(planet: Entity, pos: f32, waters_query: &Query<&PlanetSticker, With<PlanetWater>>) -> Option<Biome> {
    /* let mut closest = INFINITY;
    for water in waters_query.iter() {
        if water.contains(pos) {
            return Some(Biome::Water);
        }
        let dist = water.edge_distance_to(pos);
        if dist < closest {
            closest = dist;
        }
    }
    if closest < 16. {
        return Some(Biome::Swamp);
    }
    return Some(Biome::Ground); */
    None
}

fn handle_spawning_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    planets_query: Query<Entity, With<Planet>>,
    waters_query: Query<&PlanetSticker, With<PlanetWater>>,
    stickers_query: Query<&PlanetSticker, Without<PlanetWater>>
) {
    /* for planet_entity in planets_query.iter() {
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
            Biome::Water => spawn_fish(&mut commands, &asset_server, planet_entity, pos),
            Biome::Swamp => spawn_bush(&mut commands, &asset_server, planet_entity, pos),
            Biome::Ground => spawn_tree(&mut commands, &asset_server, planet_entity, pos),
        }
    } */
}

fn handle_natural_resources (
    mut commands: Commands,
    mut natural_resource_query: Query<(Entity, &NaturalResource, &Transform, &mut ScalingSprite)>
) {
    /*for (natural_resource_entity, natural_resource, transform, mut scaling) in natural_resource_query.iter_mut() {
        if natural_resource.amount_remaining <= 0 {
            scaling.target_scale = Vec3::ZERO;
            commands.entity(natural_resource_entity).despawn_descendants();
            commands.entity(natural_resource_entity).remove::<Occupable>();
            if transform.scale == Vec3::ZERO {
                commands.entity(natural_resource_entity).despawn();
            }
        }
    }*/
}

fn spawn_natural_resource(commands: &mut Commands, occupable_parameters: OccupableParameters, produced: ResourceType, amount: u32) {
    /* let occupable = spawn_occupable(commands, occupable_parameters);
    commands.entity(occupable).insert((
        NaturalResource { produced_resource: produced, amount_remaining: amount },
        BlinkingSprite { enabled: false }
    )); */
}


pub fn spawn_tree(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
   /*  spawn_natural_resource(
        commands,
        OccupableParameters::new(
            asset_server.load("environment/tree.png"),
            planet,
            position_degrees,
            OccupableType::Cutting,
            1,
            8.,
            bevy::sprite::Anchor::BottomCenter
        ),
        ResourceType::Wood,
        2,
    ); */
}

pub fn spawn_bush(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    /* spawn_natural_resource(
        commands,
        OccupableParameters::new(
            asset_server.load("environment/bush.png"),
            planet,
            position_degrees,
            OccupableType::Foraging,
            1,
            8.,
            bevy::sprite::Anchor::BottomCenter
        ),
        ResourceType::Food,
        10,
    ); */
}

pub fn spawn_fish(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    position_degrees: f32,
) {
    /* spawn_natural_resource(
        commands,
        OccupableParameters::new(
            asset_server.load("environment/fish.png"),
            planet,
            position_degrees,
            OccupableType::Fishing,
            1,
            8.,
            bevy::sprite::Anchor::Custom(Vec2::new(0., 1.))
        ),
        ResourceType::Food,
        10,
    ); */
}