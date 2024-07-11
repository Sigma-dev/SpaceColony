use bevy::{app::{App, Plugin}, prelude::Resource, utils::HashMap};

#[derive(Resource, Default)]
pub struct Resources {
    pub stored: HashMap<i32, i32>,
}
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Resources{ stored: HashMap::new()});
    }
}