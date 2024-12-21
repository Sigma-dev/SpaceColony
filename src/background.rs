use bevy::{app::*, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::Material2d};

use crate::planet_placing::UpdateSelection;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarsMaterial {
}

impl Material2d for StarsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/stars.wgsl".into()
    }
} 

#[derive(Component)]
pub struct Background;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
        .add_systems(Update, handle_background);
    }
}

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<StarsMaterial>>,
) {
    commands.spawn((
        Name::new("Background"),
        Mesh2d(meshes.add(Rectangle { half_size: Vec2 { x: 100., y: 100. } })),
        MeshMaterial2d(custom_materials.add(StarsMaterial { })),
        Transform {
            translation: Vec3 { x: 0., y: 0., z: -100. },
            ..default()
        },
        Background
    )).observe(|trigger: Trigger<Pointer<Click>>, mut select_event: EventWriter<UpdateSelection>|{
        println!("damso");
        select_event.send(UpdateSelection::new(None));
    });
}

fn handle_background(
    mut background_query: Query<&mut Transform, With<Background>>,
    window: Query<&Window>
) {
    let window = window.single();
    for mut background_transform in background_query.iter_mut() {
        background_transform.scale = Vec3{ x: window.width() / 100., y: window.width() / 100., z: 1.};
    }
}