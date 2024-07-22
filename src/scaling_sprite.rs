use bevy::{
    math::VectorSpace, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle}, window::PresentMode
};

#[derive(Component)]
pub struct ScalingSprite {
    pub target_scale: Vec3
}

pub struct ScalingSpritePlugin;

impl Plugin for ScalingSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_scaling);
    }
}

fn pos_sin(x: f32) -> f32 {
    return (x.sin() + 1.) / 2.0;
}

fn handle_scaling(
    mut scaling_query: Query<(&mut Transform, &ScalingSprite)>,
    time: Res<Time>,
) {
    for (mut transform, scaling) in scaling_query.iter_mut() {
        transform.scale = transform.scale.lerp(scaling.target_scale, 10. * time.delta_seconds());
        if scaling.target_scale == Vec3::ZERO && transform.scale.length() < 0.05 {
            transform.scale = Vec3::ZERO;
        }
    }
}