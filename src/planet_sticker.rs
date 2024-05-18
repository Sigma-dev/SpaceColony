use bevy::prelude::*;

use crate::planet::Planet;
use crate::looping_float::LoopingFloat;

#[derive(Component)]
pub struct PlanetSticker {
    pub planet: Entity,
    pub position_degrees: LoopingFloat<360>
}

pub struct PlanetStickerPlugin;

impl Plugin for PlanetStickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stick_to_planet);
    }
}

fn stick_to_planet(mut commands: Commands, mut sticker_query: Query<(&mut Transform, &PlanetSticker)>, targets: Query<(&GlobalTransform, &Planet)>) {
    for (mut transform, sticker) in sticker_query.iter_mut() {
        if let Ok((planet_transform, planet)) = targets.get(sticker.planet) {
            let center = planet_transform.translation();
            let pos_rad = sticker.position_degrees.to_f32().to_radians();
            transform.translation.x = center.x + pos_rad.sin() * planet.radius;
            transform.translation.y = center.y + pos_rad.cos() * planet.radius;

            let direction = Vec2::new(pos_rad.sin(), pos_rad.cos());
            let rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::new(direction.x, direction.y, 0.0).normalize());
            transform.rotation = rotation;
        }
    }
}
/* 
fn rotate(mut sticker_query: Query<&mut PlanetSticker>) {
    for mut sticker in sticker_query.iter_mut() {
        sticker.position_degrees += 0.5;
        println!("{}", sticker.position_degrees);
    }
}
*/