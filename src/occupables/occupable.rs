use std::process::Child;

use bevy::{prelude::*, render::view::visibility};

#[derive(Component)]
pub struct Occupable {
    pub selected: bool,
    pub number_of_workers: i32
}

pub struct OccupablePlugin;

impl Plugin for OccupablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_selected);
    }
}

fn handle_selected(
    mut sprite_children: Query<(&mut Visibility, &Parent)>,
    occupables:  Query<&Occupable>
) {
    for (mut visibility, parent) in sprite_children.iter_mut() {
        let occupable = occupables.get(parent.get());
        if let Ok(valid) = occupable {
            if valid.selected {
                *visibility = Visibility::Visible
            }
            else {
                *visibility = Visibility::Hidden
            }
           
        }
    }
}
