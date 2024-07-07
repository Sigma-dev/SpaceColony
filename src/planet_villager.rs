use bevy::prelude::*;
use bevy::render::view::visibility;

use crate::looping_float::LoopingFloat;
use crate::occupable::{self, Occupable, OccupableType};
use crate::planet_sticker::PlanetSticker;
use crate::{spritesheet_animator, AnimationIndices, AnimationTimer};
use rand::Rng;

#[derive(PartialEq)]
pub enum PlanetVillagerState {
    Wandering,
    Running,
    Working,
}

#[derive(Component)]
pub struct PlanetVillager {
    pub current_state: PlanetVillagerState,
    pub current_destination: Option<LoopingFloat<360>>,
    pub current_occupable: Option<Entity>,
    pub name: String
}

pub struct PlanetVillagerPlugin;

impl Plugin for PlanetVillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_villagers_behavior, animate_villagers));
    }
}

fn calculate_dir(p1: LoopingFloat<360>, p2: LoopingFloat<360>) -> f32 {
    let seperating = p1.difference(p2.to_f32());
    if seperating < 0. {
        return -1.;
    };
    return 1.;
}

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
}

fn handle_wandering(
    mut villager: Mut<PlanetVillager>,
    mut villager_sticker: Mut<PlanetSticker>,
    mut animator: Mut<spritesheet_animator::SpritesheetAnimator>,
    occupable_query: &Query<(&PlanetSticker, &Occupable), Without<PlanetVillager>>,
) {
    animator.current_animation_index = 0;
    if let Some(occupable_entity) = villager.current_occupable {
        if let Ok((sticker, occupable)) = occupable_query.get(occupable_entity) {
            villager.current_state = PlanetVillagerState::Running;
            if occupable.occupable_type == OccupableType::Cutting {
                villager.current_destination = Some(sticker.position_degrees + (-villager_sticker.position_degrees.direction(sticker.position_degrees.to_f32()) as f32 * 5.));
            } else {
                villager.current_destination = Some(sticker.position_degrees);
            }
        }
    }else if villager.current_destination.is_none() {
        villager.current_state = PlanetVillagerState::Running;
        villager.current_destination = Some(villager_sticker.position_degrees + rand::thread_rng().gen_range(-30.0..30.0))
    }
}

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
}
fn handle_working(
    mut villager: Mut<PlanetVillager>,
    mut animator: Mut<spritesheet_animator::SpritesheetAnimator>,
) {
    animator.current_animation_index = 2;
}

fn animate_villagers(
    time: Res<Time>,
    mut animators: Query<(
        &PlanetVillager,
        &PlanetSticker,
        &mut spritesheet_animator::SpritesheetAnimator,
        &mut Visibility,
        &mut Sprite
    )>,
    occupable_query: Query<&Occupable>
) {
    
    for (villager, sticker, mut animator, mut visibility, mut sprite) in animators.iter_mut() {
        *visibility = Visibility::Visible;
        if let Some(destination) = villager.current_destination {
            if calculate_dir(sticker.position_degrees, destination) < 0. {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
        if villager.current_state != PlanetVillagerState::Running {
            if villager.current_state == PlanetVillagerState::Working {
                if let Some(occupable_entity) = villager.current_occupable {
                    if let Ok(occupable) = occupable_query.get(occupable_entity) {
                        if occupable.occupable_type == OccupableType::Interior {
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
            continue;
        } 
    }
}