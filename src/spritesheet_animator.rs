use bevy::prelude::*;

#[derive(Component)]
pub struct SpritesheetAnimator {
    pub current_animation_index: u32,
    tile_size: UVec2,
    animation_frames: Vec<Vec<f32>>,
    current_frame_time: f32,
    last_animation_index: u32,
    current_frame_index: u32,
}

impl SpritesheetAnimator {
    pub fn new(tile_size: UVec2, animation_frames: Vec<Vec<f32>>) -> SpritesheetAnimator {
        SpritesheetAnimator {
            current_frame_index: 0,
            current_animation_index: 0,
            tile_size: tile_size,
            animation_frames: animation_frames,
            current_frame_time: 0.,
            last_animation_index: 0
        }
    }
}

pub struct SpritesheetAnimatorPlugin;

impl Plugin for SpritesheetAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_animators, frame_sprite,));
    }
}


fn handle_animators(mut animators: Query<(&mut SpritesheetAnimator, &mut Sprite)>, time: Res<Time>) {
    for (mut animator, mut sprite) in animators.iter_mut() {
        if animator.current_animation_index != animator.last_animation_index as u32 {
            animator.current_frame_index = 0;
            animator.current_frame_time = 0.;
        }
        if animator.current_frame_index >= animator.animation_frames[animator.current_animation_index as usize].len() as u32 {
            animator.current_frame_index = 0;
            animator.current_frame_time = 0.;
        }
        if animator.current_frame_time > animator.animation_frames[animator.current_animation_index as usize][animator.current_frame_index as usize] {
            animator.current_frame_index += 1;
            animator.current_frame_time = 0.;
        }
        animator.current_frame_time += time.delta_seconds();
        animator.last_animation_index = animator.current_animation_index;
    }
}

fn frame_sprite(mut animators: Query<(&mut Sprite, &SpritesheetAnimator)>) {
    for (mut sprite, animator) in animators.iter_mut() {
        sprite.rect = Some(Rect { 
            min: Vec2 { x: ((animator.current_frame_index * animator.tile_size.x) as f32), y: ((animator.current_animation_index * animator.tile_size.y) as f32) }, 
            max: Vec2 { x: (((animator.current_frame_index + 1) * animator.tile_size.x) as f32), y: (((animator.current_animation_index + 1) * animator.tile_size.y) as f32) } 
        })
    }
}