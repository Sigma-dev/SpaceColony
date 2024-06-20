use bevy::prelude::*;

pub struct AnimationProperties {
    pub frame_time: f32,
    pub length: u32,
}

#[derive(Component)]
pub struct SpritesheetAnimator {
    pub current_animation_index: u32,
    pub tile_size: UVec2,
    pub animation_properties: Vec<AnimationProperties>,
    pub current_frame_time: f32,
    last_animation_index: u32,
    current_frame_index: u32,
}

impl SpritesheetAnimator {
    pub fn new(tile_size: UVec2, animation_properties: Vec<AnimationProperties>) -> SpritesheetAnimator {
        SpritesheetAnimator {
            current_frame_index: 0,
            current_animation_index: 0,
            tile_size: tile_size,
            animation_properties: animation_properties,
            current_frame_time: 0.,
            last_animation_index: 0
        }
    }
}

pub struct SpritesheetAnimatorPlugin;

impl Plugin for SpritesheetAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_animators);
    }
}


fn handle_animators(mut animators: Query<(&mut SpritesheetAnimator, &mut Sprite, &Handle<Image>)>, images: Res<Assets<Image>>, time: Res<Time>) {
    for (mut animator, mut sprite, texture) in animators.iter_mut() {
        let image_option = images.get(texture.id());
        if let Some(image) = image_option {
            if animator.current_animation_index != animator.last_animation_index as u32 {
                animator.current_frame_index = 0;
                animator.current_frame_time = 0.;
            }
            if animator.current_frame_time > animator.animation_properties[animator.current_animation_index as usize].frame_time {
                animator.current_frame_time = 0.;
                animator.current_frame_index += 1;
            }
            if animator.current_frame_index >= animator.animation_properties[animator.current_animation_index as usize].length {
                animator.current_frame_index = 0
            }
            animator.current_frame_time += time.delta_seconds();
        }
        animator.last_animation_index = animator.current_animation_index;

        sprite.rect = Some(Rect { 
            min: Vec2 { x: ((animator.current_frame_index * animator.tile_size.x) as f32), y: ((animator.current_animation_index * animator.tile_size.y) as f32) }, 
            max: Vec2 { x: (((animator.current_frame_index + 1) * animator.tile_size.x) as f32), y: (((animator.current_animation_index + 1) * animator.tile_size.y) as f32) } 
        })
    }
} 

/*
fn handle_animators(mut animators: Query<(&mut SpritesheetAnimator, &mut TextureAtlas, &Handle<Image>)>, images: Res<Assets<Image>>, time: Res<Time>) {
    for (mut animator, mut atlas, texture) in animators.iter_mut() {
        let image_option = images.get(texture.id());
        if let Some(image) = image_option {
            let grid_size = UVec2 {x: image.size().x / animator.tile_size.x, y: image.size().y / animator.tile_size.y};
            let atlas_animation_index = atlas.index / (image.size().x as usize);
            if animator.current_animation_index != atlas_animation_index as u32 {
                atlas.index = (animator.current_animation_index * grid_size.x) as usize;
                animator.current_frame_time = 0.;
            }
            if animator.current_frame_time > animator.animation_properties[animator.current_animation_index as usize].frame_time {
                animator.current_frame_time = 0.;
                atlas.index += 1;
            }
            let atlas_animation_frame = atlas.index % image.size().x as usize;
            if atlas_animation_frame >= animator.animation_properties[animator.current_animation_index as usize].length as usize {
                atlas.index = (animator.current_animation_index * grid_size.x) as usize;
            }
           // println!("{}", atlas.index);
            animator.current_frame_time += time.delta_seconds();
            atlas.index = 2;
        }
       
    }
} */
