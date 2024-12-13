use bevy::{app::{App, Plugin}, prelude::*, utils::HashMap};

use crate::{planet_queries::PlanetQueries, planet_sticker::PlanetSticker, storage::{SpaceResource, Storage}};

#[derive(Resource, Default)]
pub struct Resources {
    pub stored: HashMap<i32, i32>,
}

#[derive(Component)]
pub struct ResourceExtractor {
    pub range: f32,
    pub extraction_time: f32,
    pub exploitation_bonus: u32,
    pub exploited_type: SpaceResource,
    pub exploited_resources: Vec<Entity>,
    last_extraction_time: Option<f32>,
}

#[derive(Component)]
#[require(Text2d)]
pub struct FadingText {
    fade_rate: f32,
}

impl FadingText {
    pub fn new(fade_rate: f32) -> FadingText {
        FadingText { fade_rate }
    }
}

#[derive(Event)]
pub struct ResourceProduced {
    pub produced_by: Entity,
    pub resource: SpaceResource,
    pub amount: u32,
}

impl ResourceProduced {
    pub fn new(produced_by: Entity, resource: SpaceResource, amount: u32) -> ResourceProduced {
        ResourceProduced { produced_by, resource, amount }
    }
}

impl ResourceExtractor {
    pub fn new(exploited_type: SpaceResource, extraction_time: f32, exploitation_bonus: u32, range: f32) -> ResourceExtractor {
        ResourceExtractor { 
            range,
            exploited_type, 
            extraction_time,
            exploitation_bonus,
            exploited_resources: Vec::new(), 
            last_extraction_time: None,
        }
    }
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ResourceProduced>()
        .insert_resource(Resources{ stored: HashMap::new()})
        .add_systems(Update, (on_extractor, handle_extraction, handle_produced_events, handle_fading_texts));
    }
}

fn on_extractor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut extractor_query: Query<(Entity, &PlanetSticker, &mut ResourceExtractor), Added<ResourceExtractor>>,
    planet_queries: PlanetQueries
) {
    for (entity, sticker, mut extractor) in extractor_query.iter_mut() {
        extractor.exploited_resources = planet_queries.get_natural_resource_in_range(*sticker, extractor.range, extractor.exploited_type);
        let child = commands.spawn((
            FadingText::new(1.),
            TextFont {
                font: asset_server.load("fonts/pixel.ttf"),
                font_size: 5.,
                ..default()
            }
        )).id();
        commands.entity(entity).add_child(child);
    }
}

fn handle_extraction(
    mut produced_event: EventWriter<ResourceProduced>,
    mut extractor_query: Query<(Entity, &mut ResourceExtractor, &mut Storage)>,
    time: Res<Time>
) {
    for (entity, mut extractor, mut extractor_storage) in extractor_query.iter_mut() {
        if extractor.last_extraction_time.map_or(true, |t| time.elapsed_secs() > t + extractor.extraction_time) {
            let amount = extractor.exploited_resources.len() as u32 * extractor.exploitation_bonus;
            if amount == 0 {
                continue;
            }
            extractor_storage.add(extractor.exploited_type, amount);
            extractor.last_extraction_time = Some(time.elapsed_secs());
            produced_event.send(ResourceProduced::new(entity, extractor.exploited_type, amount));
        }
    }
}

fn handle_fading_texts(
    mut text_query: Query<(&mut Transform, &mut TextColor, &FadingText)>,
    time: Res<Time>,
) {
    for (mut transform, mut text_color, fading_text) in text_query.iter_mut() {
        let alpha = text_color.alpha();
        if alpha > 0. {
            let delta = fading_text.fade_rate * time.delta_secs();
            transform.translation.y += delta * 1.0;
            text_color.set_alpha(alpha - delta);
        }
    }
    
}

fn handle_produced_events(
    mut produced_event: EventReader<ResourceProduced>,
    mut text_query: Query<(&Parent, &mut Transform, &mut Text2d, &mut TextColor)>,
) {
    for event in produced_event.read() {
        for (p, mut transform, mut text, mut text_color) in text_query.iter_mut() {
            if p.get() != event.produced_by { continue; }
            *transform = Transform::from_xyz(0., 30., 0.);
            text.0 = format!("+{} {}", event.amount, event.resource.to_string());
            text_color.set_alpha(1.);
        }
    } 
}