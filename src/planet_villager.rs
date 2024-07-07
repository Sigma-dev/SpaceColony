use bevy::prelude::*;
use bevy::render::view::visibility;

use crate::looping_float::LoopingFloat;
use crate::occupable::{self, Occupable, OccupableType};
use crate::planet_sticker::PlanetSticker;
use crate::{spritesheet_animator};
use rand::Rng;

pub enum PlanetVillagerAnimationState {
    Idle = 0,
    Run = 1,
    Cut = 2,
}

#[derive(Component)]
pub struct PlanetVillager {
    pub name: String,
}

#[derive(Component)]
pub struct VillagerWorking {
    pub current_occupable: Entity,
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
        mut sticker,
        mut sprite,
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
    animator.current_animation_index = PlanetVillagerAnimationState::Run as u32;;
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
) {
    for (mut worker, mut sticker, mut visibility, mut sprite, mut animator) in
        villager_query.iter_mut()
    {
        if let Ok((occupable, occupable_sticker)) = occupable_query.get(worker.current_occupable) {
            let mut target = occupable_sticker.position_degrees;
            if occupable.occupable_type == OccupableType::Cutting {
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
                animator.current_animation_index = PlanetVillagerAnimationState::Cut as u32;;
            }

            *visibility = if occupable.occupable_type == OccupableType::Interior {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        }
    }
}
