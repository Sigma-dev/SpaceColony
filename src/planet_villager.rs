use bevy::prelude::*;
use bevy::render::view::visibility;

use crate::looping_float::LoopingFloat;
use crate::occupable::{self, Occupable, OccupableType};
use crate::planet_sticker::PlanetSticker;
use crate::{AnimationIndices, AnimationTimer};

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
    mut villager_query: Query<(&mut PlanetVillager, &mut PlanetSticker, &mut Transform)>,
    occupable_query: Query<&PlanetSticker, (With<Occupable>, Without<PlanetVillager>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut villager, mut sticker, mut transform) in villager_query.iter_mut() {
        match villager.current_state {
            PlanetVillagerState::Wandering => handle_wandering(villager, &occupable_query),
            PlanetVillagerState::Running => handle_running(villager, sticker, &time),
            PlanetVillagerState::Working => handle_working(villager),
        }
    }
}

fn handle_wandering(
    mut villager: Mut<PlanetVillager>,
    occupable_query: &Query<&PlanetSticker, (With<Occupable>, Without<PlanetVillager>)>,
) {
    if let Some(occupable) = villager.current_occupable {
        if let Ok(found) = occupable_query.get(occupable) {
            villager.current_destination = Some(found.position_degrees);
            villager.current_state = PlanetVillagerState::Running;
        }
    }
}

fn handle_running(
    mut villager: Mut<PlanetVillager>,
    mut sticker: Mut<PlanetSticker>,
    time: &Res<Time>,
) {
    if let Some(destination) = villager.current_destination {
        let seperating = sticker.position_degrees.difference(destination.to_f32());
        if seperating.abs() < 0.1 {
            if villager.current_occupable != None {
                villager.current_state = PlanetVillagerState::Working;
            } else {
                villager.current_state = PlanetVillagerState::Wandering;
            }
        }
        let dir = calculate_dir(sticker.position_degrees, destination);
        sticker.position_degrees += dir * 15. * time.delta_seconds()
    }
}
fn handle_working(mut villager: Mut<PlanetVillager>) {
}

fn animate_villagers(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut Sprite,
        &PlanetVillager,
        &PlanetSticker,
        &mut Visibility
    )>,
    occupable_query: Query<&Occupable>
) {
    for (indices, mut timer, mut atlas, mut sprite, villager, sticker, mut visibility) in &mut query {
        *visibility = Visibility::Visible;
        if let Some(destination) = villager.current_destination {
            if calculate_dir(sticker.position_degrees, destination) < 0. {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
        if villager.current_state != PlanetVillagerState::Running {
            atlas.index = 0;
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
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
        if atlas.index == 1 {
            atlas.index = 2
        }
    }
}