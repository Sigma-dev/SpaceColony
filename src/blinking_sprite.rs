use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle}, window::PresentMode
};

#[derive(Component)]
pub struct BlinkingSprite {
    pub enabled: bool
}

pub struct BlinkingSpritePlugin;

impl Plugin for BlinkingSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_blinking);
    }
}

fn pos_sin(x: f32) -> f32 {
    return (x.sin() + 1.) / 2.0;
}

fn handle_blinking(
    mut blinking_query: Query<(&mut Sprite, &BlinkingSprite)>,
    time: Res<Time>,
) {
    for (mut sprite, blinking) in blinking_query.iter_mut() {
        if (blinking.enabled == true) { 
            sprite.color.set_alpha(pos_sin(time.elapsed_seconds() * 10.))
        } else {
            sprite.color.set_alpha(1.);
        }

    }
}