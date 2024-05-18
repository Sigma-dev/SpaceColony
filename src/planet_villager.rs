use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::planet_sticker::PlanetSticker;

#[derive(PartialEq)]
pub enum PlanetVillagerState {
    Waiting,
    Running,
}

#[derive(Component)]
pub struct PlanetVillager {
    pub current_state: PlanetVillagerState,
    pub current_destination: f32,
}

pub struct PlanetVillagerPlugin;

impl Plugin for PlanetVillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_villagers_behavior, animate_villagers));
    }
}

fn handle_villagers_behavior(
    mut villager_query: Query<(&mut PlanetVillager, &mut PlanetSticker, &mut Transform)>,
    time: Res<Time>
) {
    for (mut villager, mut sticker, mut transform) in villager_query.iter_mut() {
        if villager.current_state == PlanetVillagerState::Running {

            let seperating = sticker
                .position_degrees
                .difference(villager.current_destination);
            if seperating.abs() < 0.1 {
                villager.current_state = PlanetVillagerState::Waiting;
                continue;
            }
            println!("{}", sticker.position_degrees);

            let dir = seperating.clamp(-1., 1.);
            sticker.position_degrees += dir * 10. * time.delta_seconds()
        }
    }
}

fn animate_villagers() {}
