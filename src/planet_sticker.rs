use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::planet::Planet;

#[derive(Component, Default)]
pub struct PlanetSticker {
    pub planet: Option<Entity>,
    pub position_degrees: LoopingFloat<360>,
}

pub struct PlanetStickerPlugin;

impl Plugin for PlanetStickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stick_to_planet);
    }
}

fn stick_to_planet(
    mut sticker_query: Query<(&mut Transform, &PlanetSticker)>,
    targets: Query<(&GlobalTransform, &Planet)>,
) {
    for (mut transform, sticker) in sticker_query.iter_mut() {
        if let Some(planet) = sticker.planet {
            if let Ok((planet_transform, planet)) = targets.get(planet) {
                let center = planet_transform.translation();
                let pos_rad = sticker.position_degrees.to_f32().to_radians();
                let sink = 0.75;
                transform.translation.x = center.x + pos_rad.sin() * (planet.radius - sink);
                transform.translation.y = center.y + pos_rad.cos() * (planet.radius - sink);

                let direction = Vec2::new(pos_rad.sin(), pos_rad.cos());
                let rotation = Quat::from_rotation_arc(
                    Vec3::Y,
                    Vec3::new(direction.x, direction.y, 0.0).normalize(),
                );
                transform.rotation = rotation;
            }
        }
    }
}
