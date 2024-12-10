use bevy::prelude::*;
use crate::looping_float::LoopingFloat;
use crate::occupable::{Occupable, OccupableType};
use crate::planet::PlanetWater;
use crate::planet_sticker::{self, PlanetCollider, PlanetSticker};
use crate::resources::Resources;
use crate::{spritesheet_animator, natural_resource::NaturalResource};
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
    pub current_work: Entity,
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
    water_query: Query<(&PlanetSticker, &PlanetCollider), (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
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
            wandering.wait_time -= time.delta_secs();
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
                time.delta_secs(),
                wandering.current_destination,
                7.,
            ) {
                wandering.wait_time = rand::thread_rng().gen_range(0.5..2.5);
            }
        }
    }
}

pub fn get_walk_dir(
    villager_sticker: &PlanetSticker,
    water_query: &Query<(&PlanetSticker, &PlanetCollider), (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
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
    water_query: &Query<(&PlanetSticker, &PlanetCollider), (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
    start: LoopingFloat<360>,
    end: f32,
    dir: i32,
) -> bool {
    for (water, water_collider) in water_query.iter() {
        if is_obstructing(water, water_collider, start, end, dir) {
            return false;
        }
    }
    return true;
}

fn is_obstructing(
    water: &PlanetSticker,
    water_collider: &PlanetCollider,
    start: LoopingFloat<360>,
    end: f32,
    dir: i32,
) -> bool {
    let dir_bool = dir == 1;
    let water_size = water_collider.size_degrees;
    let water_start = water.position_degrees - water_size / 2.;
    let water_end = water.position_degrees  + water_size / 2.;
    if (start.to_f32() > water_start.to_f32() && start.to_f32() < water_end.to_f32()) || (end > water_start.to_f32() && end < water_end.to_f32()) {
        return true;
    }
    let a;
    let b;
    if dir_bool {
        a = start.is_in_between(water_start.to_f32(), end, dir_bool);
        b = start.is_in_between(water_end.to_f32(), end, dir_bool);
    } else {
        a = LoopingFloat::<360>::new(end).is_in_between(water_start.to_f32(), start.to_f32(), true);
        b = LoopingFloat::<360>::new(end).is_in_between(water_end.to_f32(), start.to_f32(), true);
    }
    if a || b {
        return true;
    }
    
    return false;
}


fn walk_towards(
    animator: &mut spritesheet_animator::SpritesheetAnimator,
    mut sticker: Mut<PlanetSticker>,
    mut sprite: Mut<Sprite>,
    water_query: &Query<(&PlanetSticker, &PlanetCollider), (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
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
    mut commands: Commands,
    mut villager_query: Query<(
        Entity,
        &mut VillagerWorking,
        &mut PlanetSticker,
        &mut Visibility,
        &mut Sprite,
        &mut spritesheet_animator::SpritesheetAnimator,
    )>,
    water_query: Query<(&PlanetSticker, &PlanetCollider), (With<PlanetWater>, Without<Occupable>, Without<VillagerWorking>, Without<VillagerWandering>)>,
    occupable_query: Query<(&Occupable, &PlanetSticker), Without<VillagerWorking>>,
    mut natural_resource_query: Query<&mut NaturalResource>,
    time: Res<Time>,
    mut resources: ResMut<Resources>
) {
    for (worker_entity, mut worker, sticker, mut visibility, sprite, mut animator) in
        villager_query.iter_mut()
    {
        *visibility = Visibility::Visible;
        if let Ok((occupable, occupable_sticker)) = occupable_query.get(worker.current_work) {
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
                time.delta_secs(),
                target,
                15.,
            ) {
                let anim = match occupable.occupable_type {
                    OccupableType::Cutting => PlanetVillagerAnimationState::Cut,
                    OccupableType::Foraging => PlanetVillagerAnimationState::Forage,
                    OccupableType::Interior => PlanetVillagerAnimationState::Idle,
                    OccupableType::Fishing => PlanetVillagerAnimationState::Forage,
                };
                animator.current_animation_index = anim as u32;
                if let Ok(mut natural_resource) = natural_resource_query.get_mut(worker.current_work) {
                    worker.production_interval -= time.delta_secs();
                    if worker.production_interval <= 0.0 {
                        let index = natural_resource.produced_resource as i32;
                        let current_value = resources.stored.get(&index).copied().unwrap_or(0);
                        natural_resource.amount_remaining -= 1;
                        resources.stored.insert(index, current_value + 1 as i32);
                        worker.production_interval = 1.0;
                    }
                }
                if occupable.occupable_type == OccupableType::Interior {
                    *visibility = Visibility::Hidden
                }
            }
        } else {
            if worker.current_work == worker.current_occupable {
                commands
                            .entity(worker_entity)
                            .remove::<VillagerWorking>()
                            .insert(VillagerWandering::default());
            } else {
                worker.current_work = worker.current_occupable;
            }
        }
    }
}

pub fn spawn_villager(commands: &mut Commands, asset_server: &Res<AssetServer>, planet: Entity, position_degrees: f32, name: String) {
    commands.spawn((
        Sprite {
            image: asset_server.load("player/player.png"),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform {
            translation: Vec3{ x: 0., y: 0., z: 10.},
            ..default()
        },
        spritesheet_animator::SpritesheetAnimator::new(
            UVec2 { x: 16, y: 16 },
            vec![vec![0.6; 2], vec![0.2; 2], vec![0.2; 4], vec![0.2; 2]],
        ),
        planet_sticker::PlanetSticker {
            planet,
            position_degrees: LoopingFloat::new(position_degrees),
        },
        PlanetVillager {
            _name: format!("{}", name),
        },
        VillagerWandering::default(),
        PickingBehavior::IGNORE,
        Name::new("Villager")
    ));
}

pub fn count_workers(worker_query: &Query<&VillagerWorking>, occupable_entity: Entity) -> u32 {
    let mut count = 0;
    for worker in worker_query {
        if worker.current_work == occupable_entity {
            count += 1;
        } else if worker.current_occupable == occupable_entity {
            count += 1;
        }
    }
    return count;
}

pub fn count_occupiers(worker_query: &Query<&VillagerWorking>, occupable_entity: Entity) -> u32 {
    let mut count = 0;
    for worker in worker_query {
        if worker.current_occupable == occupable_entity {
            count += 1;
        }
    }
    return count;
}