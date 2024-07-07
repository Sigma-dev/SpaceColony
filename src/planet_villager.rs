use bevy::prelude::*;
use bevy::render::view::visibility;

use crate::looping_float::LoopingFloat;
use crate::occupable::{self, Occupable, OccupableType};
use crate::planet_sticker::PlanetSticker;
use crate::{spritesheet_animator, AnimationIndices, AnimationTimer};
use rand::Rng;

pub enum PlanetVillagerAnimationState {
    Idle = 0,
    Run = 1,
    Work = 2,
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
            wait_time: 0.01
        }
    }
}

pub struct PlanetVillagerPlugin;

impl Plugin for PlanetVillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_working_villagers,
                handle_wandering_villagers,
            ),
        );
    }
}

fn calculate_dir(p1: LoopingFloat<360>, p2: LoopingFloat<360>) -> f32 {
    let seperating = p1.difference(p2.to_f32());
    if seperating < 0. {
        return -1.;
    };
    return 1.;
}
/*
fn handle_villagers_behavior(
    mut villager_query: Query<(&mut PlanetVillager, &mut PlanetSticker, &mut Transform, &mut spritesheet_animator::SpritesheetAnimator)>,
    occupable_query: Query<(&PlanetSticker, &Occupable), Without<PlanetVillager>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut villager, mut sticker, mut transform, mut animator) in villager_query.iter_mut() {
        match villager.current_state {
            PlanetVillagerState::Wandering => handle_wandering(villager, sticker, animator, &occupable_query),
            PlanetVillagerState::Running => handle_running(villager, animator, sticker, &time),
            PlanetVillagerState::Working => handle_working(villager, animator),
        }
    }
}*/

fn handle_wandering_villagers(
    mut villager_query: Query<(
        &mut PlanetVillager,
        &mut VillagerWandering,
        &mut PlanetSticker,
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
        &mut spritesheet_animator::SpritesheetAnimator,
    )>,
    occupable_query: Query<(&PlanetSticker, &Occupable), Without<PlanetVillager>>,
    time: Res<Time>,
) {
    for (mut villager, mut wandering, mut sticker, mut transform, mut sprite, mut visibility, mut animator) in
        villager_query.iter_mut()
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
            if (walk_towards(&mut animator, sticker, sprite, time.delta_seconds(), wandering.current_destination, 7.)) {
                wandering.wait_time = rand::thread_rng().gen_range(0.5..2.5);
            }
            /* 
            animator.current_animation_index = 1;
            let seperating = sticker
                .position_degrees
                .difference(wandering.current_destination.to_f32());
            if seperating.abs() < 0.1 {
                wandering.wait_time = rand::thread_rng().gen_range(0.5..2.5);
                return;
            }
            let dir = calculate_dir(sticker.position_degrees, wandering.current_destination);
            sticker.position_degrees += dir * 7. * time.delta_seconds();
            sprite.flip_x = seperating < 0.;
            */
        }
    }
}

fn walk_towards(animator: &mut spritesheet_animator::SpritesheetAnimator, mut sticker: Mut<PlanetSticker>,  mut sprite: Mut<Sprite>, elapsed_seconds: f32, destination: LoopingFloat<360>, speed: f32) -> bool {
    let seperating = sticker
                .position_degrees
                .difference(destination.to_f32());
    if seperating.abs() < 0.1 {
        return true;
    }
    let dir = calculate_dir(sticker.position_degrees, destination);
    sticker.position_degrees += dir * speed * elapsed_seconds;
    sprite.flip_x = seperating < 0.;
    animator.current_animation_index = 1;
    return false
}
/*
fn handle_running(
    mut villager: Mut<PlanetVillager>,
    mut animator: Mut<spritesheet_animator::SpritesheetAnimator>,
    mut sticker: Mut<PlanetSticker>,
    time: &Res<Time>,
) {
    animator.current_animation_index = 1;
    if let Some(destination) = villager.current_destination {
        let seperating = sticker.position_degrees.difference(destination.to_f32());
        if seperating.abs() < 0.1 {
            if villager.current_occupable != None {
                villager.current_state = PlanetVillagerState::Working;
            } else {
                villager.current_state = PlanetVillagerState::Wandering;
                villager.current_destination = None;
            }
        }
        let dir = calculate_dir(sticker.position_degrees, destination);
        sticker.position_degrees += dir * 15. * time.delta_seconds()
    }
}*/

fn handle_working_villagers(
    mut villager_query: Query<(
        &mut VillagerWorking,
        &mut PlanetSticker,
        &mut Visibility,
        &mut Sprite,
        &mut spritesheet_animator::SpritesheetAnimator,
    )>,
    occupable_query: Query<(&Occupable, &PlanetSticker), Without<PlanetVillager>>,
    time: Res<Time>
) {
    for (mut worker, mut sticker, mut visibility, mut sprite, mut animator) in
        villager_query.iter_mut()
    {
        *visibility = Visibility::Visible;
        

        if let Ok((occupable,occupable_sticker)) = occupable_query.get(worker.current_occupable) {
            let mut target = occupable_sticker.position_degrees;
            if occupable.occupable_type == OccupableType::Cutting {
                target += sticker.position_degrees.direction(occupable_sticker.position_degrees.to_f32()) as f32 * -5.;
            }
            if walk_towards(&mut animator, sticker, sprite, time.delta_seconds(), target, 15.) {
                animator.current_animation_index = 2;
            }
            if occupable.occupable_type == OccupableType::Interior {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
