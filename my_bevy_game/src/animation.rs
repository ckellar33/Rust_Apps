use bevy::prelude::*;
use std::time::Duration;

pub struct AnimationFrame {
    pub index: i32,
    pub time: f32,
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame: i32,
}

pub struct Animations {
    pub animations: Vec<Animation>,
    pub current_animation: i32,
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_system.system());
    }
}

fn animate_system(time: Res<Time>, mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Animations)>) {
    for (mut timer, mut sprite, mut animations) in &mut query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let current_animation_index = animations.current_animation;
            match animations
                .animations
                .get_mut(current_animation_index as usize)
            {
                Some(animation) => {
                    animation.current_frame += 1;
                    if animation.current_frame as usize >= animation.frames.len() {
                        animation.current_frame = 0;
                    }
                    let frame_data = animation
                        .frames
                        .get(animation.current_frame as usize)
                        .unwrap();
                    timer.set_duration(Duration::from_secs_f32(frame_data.time));
                    if let Some(frame) = animation.frames.get(animation.current_frame as usize) {
                        sprite.index = frame.index as u32;
                    }
                }
                None => {}
            }
        }
    }
}