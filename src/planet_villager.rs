use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::occupable::{Occupable, OccupableType};
use crate::planet_sticker::{self, PlanetSticker};
use crate::resources::Resources;
use crate::spritesheet_animator;
use rand::Rng;

pub enum PlanetVillagerAnimationState {
    Idle = 0,
    Run = 1,
    Cut = 2,
    Forage = 3,
}

#[derive(Component)]
pub struct PlanetVillager {
    pub _name: String,
}

#[derive(Component)]
pub struct VillagerWorking {
    pub current_occupable: Entity,
    pub production_interval: f32,
}

#[derive(Component)]
pub struct VillagerWandering {
    pub current_destination: LoopingFloat<360>,
    pub wait_time: f32,
}

impl Default for VillagerWandering {
    fn default() -> VillagerWandering {
        VillagerWandering {
            current_destination: LoopingFloat::new(0.),
            wait_time: 0.01,
        }
    }
}

pub struct PlanetVillagerPlugin;

impl Plugin for PlanetVillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_working_villagers, handle_wandering_villagers),
        );
    }
}

fn handle_wandering_villagers(
    mut villager_query: Query<(
        &mut VillagerWandering,
        &mut PlanetSticker,
        &mut Sprite,
        &mut Visibility,
        &mut spritesheet_animator::SpritesheetAnimator,
    )>,
    time: Res<Time>,
) {
    for (
        mut wandering,
        sticker,
        sprite,
        mut visibility,
        mut animator,
    ) in villager_query.iter_mut()
    {
        *visibility = Visibility::Visible;
        animator.current_animation_index = PlanetVillagerAnimationState::Idle as u32;
        if wandering.wait_time > 0. {
            wandering.wait_time -= time.delta_seconds();
            if wandering.wait_time <= 0. {
                wandering.current_destination =
                    sticker.position_degrees + rand::thread_rng().gen_range(-20.0..20.0)
            }
        } else {
            if walk_towards(
                &mut animator,
                sticker,
                sprite,
                time.delta_seconds(),
                wandering.current_destination,
                7.,
            ) {
                wandering.wait_time = rand::thread_rng().gen_range(0.5..2.5);
            }
        }
    }
}

fn walk_towards(
    animator: &mut spritesheet_animator::SpritesheetAnimator,
    mut sticker: Mut<PlanetSticker>,
    mut sprite: Mut<Sprite>,
    elapsed_seconds: f32,
    destination: LoopingFloat<360>,
    speed: f32,
) -> bool {
    let seperating = sticker.position_degrees.difference(destination.to_f32());
    if seperating.abs() < 0.1 {
        return true;
    }
    let dir = sticker.position_degrees.direction(destination.to_f32()) as f32;
    sticker.position_degrees += dir * speed * elapsed_seconds;
    sprite.flip_x = seperating < 0.;
    animator.current_animation_index = PlanetVillagerAnimationState::Run as u32;
    return false;
}

fn handle_working_villagers(
    mut villager_query: Query<(
        &mut VillagerWorking,
        &mut PlanetSticker,
        &mut Visibility,
        &mut Sprite,
        &mut spritesheet_animator::SpritesheetAnimator,
    )>,
    occupable_query: Query<(&Occupable, &PlanetSticker), Without<VillagerWorking>>,
    time: Res<Time>,
    mut resources: ResMut<Resources>
) {
    for (mut worker, sticker, mut visibility, sprite, mut animator) in
        villager_query.iter_mut()
    {
        if let Ok((occupable, occupable_sticker)) = occupable_query.get(worker.current_occupable) {
            let mut target = occupable_sticker.position_degrees;
            if occupable.occupable_type != OccupableType::Interior {
                target += sticker
                    .position_degrees
                    .direction(occupable_sticker.position_degrees.to_f32())
                    as f32
                    * -5.;
            }
            if walk_towards(
                &mut animator,
                sticker,
                sprite,
                time.delta_seconds(),
                target,
                15.,
            ) {
                let anim = match occupable.occupable_type {
                    OccupableType::Cutting => PlanetVillagerAnimationState::Cut,
                    OccupableType::Foraging => PlanetVillagerAnimationState::Forage,
                    OccupableType::Interior => PlanetVillagerAnimationState::Idle
                };
                animator.current_animation_index = anim as u32;
                worker.production_interval -= time.delta_seconds();
                if worker.production_interval <= 0.0 {
                    let index = occupable.produced_resource as i32;
                    let current_value = resources.stored.get(&index).copied().unwrap_or(0);
                    resources.stored.insert(index, current_value + 1 as i32);
                    worker.production_interval = 1.0;
                }
            }

            *visibility = if occupable.occupable_type == OccupableType::Interior {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        }
    }
}

pub fn spawn_villager(commands: &mut Commands, asset_server: &Res<AssetServer>, planet: Entity, position_degrees: f32, name: String) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player/player.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
        spritesheet_animator::SpritesheetAnimator::new(
            UVec2 { x: 16, y: 16 },
            vec![vec![0.6; 2], vec![0.2; 2], vec![0.2; 4], vec![0.2; 2]],
        ),
        planet_sticker::PlanetSticker {
            planet: planet,
            position_degrees: LoopingFloat::new(position_degrees),
        },
        PlanetVillager {
            _name: format!("{}", name),
        },
        VillagerWandering::default(),
    ));
}