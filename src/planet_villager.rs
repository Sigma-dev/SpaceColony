use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::planet_sticker::PlanetSticker;
use crate::{AnimationIndices, AnimationTimer};

#[derive(PartialEq)]
pub enum PlanetVillagerState {
    Waiting,
    Running,
}

#[derive(Component)]
pub struct PlanetVillager {
    pub current_state: PlanetVillagerState,
    pub current_destination: LoopingFloat<360>,
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
    time: Res<Time>,
) {
    for (mut villager, mut sticker, mut transform) in villager_query.iter_mut() {
        if villager.current_state == PlanetVillagerState::Running {
            let seperating = sticker
                .position_degrees
                .difference(villager.current_destination.to_f32());
            if seperating.abs() < 0.1 {
                villager.current_state = PlanetVillagerState::Waiting;
                continue;
            }
            let dir = calculate_dir(sticker.position_degrees, villager.current_destination);
            sticker.position_degrees += dir * 15. * time.delta_seconds()
        }
    }
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
    )>,
) {
    for (indices, mut timer, mut atlas, mut sprite, villager, sticker) in &mut query {
        if calculate_dir(sticker.position_degrees, villager.current_destination) < 0. {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }

        if villager.current_state == PlanetVillagerState::Waiting {
            atlas.index = 0;
            return
        }
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
        //atlas.index += 1
    }
}
/* 
fn determine_frame(
    indices: &AnimationIndices,
    mut timer: &AnimationTimer,
    mut atlas: &TextureAtlas,
    mut time: Res<Time>,
) {
}
*/
