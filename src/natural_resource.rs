use bevy::prelude::*;

use crate::{blinking_sprite::BlinkingSprite, spawn_occupable, NewOccupable, OccupableBundle, OccupableType, ResourceType};

#[derive(Component, PartialEq)]
pub struct NaturalResource {
    pub produced_resource: ResourceType,
    pub amount_remaining: u32,
}

pub struct NaturalResourcePlugin;

impl Plugin for NaturalResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_natural_resources);
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
