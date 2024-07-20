use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::occupable::{Occupable, OccupableType};
use crate::planet::{self, Planet, PlanetWater};
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
    water_query: Query<&PlanetSticker, (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
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
        let Some(planet_entity) = sticker.planet else {return;};
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
                &water_query,
                time.delta_seconds(),
                wandering.current_destination,
                7.,
            ) {
                wandering.wait_time = rand::thread_rng().gen_range(0.5..2.5);
            }
        }
    }
}

fn get_walk_dir(
    mut villager_sticker: &Mut<PlanetSticker>,
    water_query: &Query<&PlanetSticker, (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
    destination: LoopingFloat<360>
) -> Option<i32> {
    let shortest = villager_sticker.position_degrees.direction(destination.to_f32());
    let longest = -shortest;
    if is_path_free(water_query, villager_sticker.position_degrees, destination.to_f32(), shortest) {
        return Some(shortest);
    }
    if is_path_free(water_query, villager_sticker.position_degrees, destination.to_f32(), longest) {
        return Some(longest);
    }
    return None;
}

fn is_path_free(
    water_query: &Query<&PlanetSticker, (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
    start: LoopingFloat<360>,
    end: f32,
    dir: i32,
) -> bool {
    for water in water_query.iter() {
        if is_obstructing(water, start, end, dir) {
            return false;
        }
    }
    println!("free");
    return true;
}

fn is_obstructing(
    water: &PlanetSticker,
    start: LoopingFloat<360>,
    end: f32,
    dir: i32,
) -> bool {
    let dir_bool = dir == 1;
    let Some(water_size) = water.size_degrees else { return false; };
    let water_start = water.position_degrees - water_size / 2.;
    let water_end = water.position_degrees  + water_size / 2.;
    println!("{} {} {} {} {}", start, water_start.to_f32(), water_end.to_f32(), end, dir_bool);
    if (start.to_f32() > water_start.to_f32() && start.to_f32() < water_end.to_f32()) || (end > water_start.to_f32() && end < water_end.to_f32()) {
        return true;
    }
    let mut a;
    let mut b;
    if (dir_bool) {
        a = start.is_in_between(water_start.to_f32(), end, dir_bool);
        b = start.is_in_between(water_end.to_f32(), end, dir_bool);
    } else {
        a = LoopingFloat::<360>::new(end).is_in_between(water_start.to_f32(), start.to_f32(), true);
        b = LoopingFloat::<360>::new(end).is_in_between(water_end.to_f32(), start.to_f32(), true);
    }
    println!("a: {}, b: {}", a, b);
    if a || b {
        return true;
    }
    
    return false;
}


fn walk_towards(
    animator: &mut spritesheet_animator::SpritesheetAnimator,
    mut sticker: Mut<PlanetSticker>,
    mut sprite: Mut<Sprite>,
    water_query: &Query<&PlanetSticker, (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
    elapsed_seconds: f32,
    destination: LoopingFloat<360>,
    speed: f32,
) -> bool {
    let seperating = sticker.position_degrees.difference(destination.to_f32());
    if seperating.abs() < 0.1 {
        return true;
    }
    let dir_opt = get_walk_dir(&sticker, water_query, destination);
    let Some(dir) = dir_opt else {return false;};
    sticker.position_degrees += dir as f32 * speed * elapsed_seconds;
    sprite.flip_x = dir < 0;
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
    water_query: Query<&PlanetSticker, (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
    occupable_query: Query<(&Occupable, &PlanetSticker), Without<VillagerWorking>>,
    time: Res<Time>,
    mut resources: ResMut<Resources>
) {
    for (mut worker, sticker, mut visibility, sprite, mut animator) in
        villager_query.iter_mut()
    {
        let Some(planet_entity) = sticker.planet else {return;};
        *visibility = Visibility::Visible;
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
                &water_query,
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
                if occupable.occupable_type == OccupableType::Interior {
                    *visibility = Visibility::Hidden
                }
            }
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
            planet: Some(planet),
            position_degrees: LoopingFloat::new(position_degrees),
            size_degrees: None
        },
        PlanetVillager {
            _name: format!("{}", name),
        },
        VillagerWandering::default(),
    ));
}