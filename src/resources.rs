use bevy::{app::{App, Plugin}, prelude::*, utils::HashMap};

use crate::{planet_queries::PlanetQueries, planet_sticker::PlanetSticker, storage::SpaceResource};

#[derive(Resource, Default)]
pub struct Resources {
    pub stored: HashMap<i32, i32>,
}

#[derive(Component)]
pub struct ResourceExtractor {
    pub range: f32,
    pub exploited_type: SpaceResource,
    pub exploited_resources: Vec<Entity>
}

impl ResourceExtractor {
    pub fn new(exploited_type: SpaceResource, range: f32) -> ResourceExtractor {
        ResourceExtractor { exploited_resources: Vec::new(), range, exploited_type }
    }
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Resources{ stored: HashMap::new()})
        .add_systems(Update, on_extractor);
    }
}

fn on_extractor(
    mut extractor_query: Query<(&PlanetSticker, &mut ResourceExtractor), Added<ResourceExtractor>>,
    planet_queries: PlanetQueries
) {
    for (sticker, mut extractor) in extractor_query.iter_mut() {
        extractor.exploited_resources = planet_queries.get_natural_resource_in_range(*sticker, extractor.range, extractor.exploited_type);
        println!("{:?}", extractor.exploited_resources);
    }
}