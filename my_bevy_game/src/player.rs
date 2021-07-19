use bevy::{input::keyboard, prelude::*};

use crate::physics::*;
use crate::animation::*;

pub struct Player;

struct Rotator {
    angle: f32
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system())
        .add_system(player_input.system());
    }
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, jump_velocity: Res<JumpVelocity>, mut q: Query<(&mut Animations, &mut Velocity, &mut CircularVelocity), With<Player>>) {
    for (mut animation, mut velocity, mut c_velocity) in q.iter_mut() {
        animation.current_animation = 0;
        c_velocity.y_angle = 0.0;
        velocity.0.x = 0.0;
        velocity.0.y = 0.0;

        if keyboard_input.pressed(KeyCode::Right) {
            animation.current_animation = 1;
            c_velocity.y_angle = 0.0;
            velocity.0.x = 10.0 * 40.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            animation.current_animation = 1;  
            c_velocity.y_angle = 180.0;
            velocity.0.x = -10.0 * 40.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            animation.current_animation = 3;  
            velocity.0.y = 10.0 * 40.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            animation.current_animation = 1;  
            velocity.0.y = -10.0 * 40.0;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            animation.current_animation = 2;
            velocity.0.z = jump_velocity.0;
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/female_advent_hd.png");
    // Gabe - Vec2::new(24.0, 24.0), 7, 1
    // female -
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 9, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            sprite: TextureAtlasSprite::new(0 as u32),
            // transform: Transform { translation: Vec3::new(0.0, 0.0, 1.0), rotation: Quat::default(), scale: Vec3::splat(6.0)},
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player)
        .insert(Rotator{angle: 0.0})
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(AffectedByGravity)
        .insert(CircularVelocity{ x_angle: 0.0, y_angle: 0.0, z_angle: 0.0, velocity: 0.0})
        .insert(Animations {
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 0,
                            time: 0.1,
                        }
                    ]
                },
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 0,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 36,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 37,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 38,
                            time: 0.1,
                        },
                        // AnimationFrame {
                        //     index: 4,
                        //     time: 0.1,
                        // },
                        // AnimationFrame {
                        //     index: 5,
                        //     time: 0.1,
                        // },
                        // AnimationFrame {
                        //     index: 6,
                        //     time: 0.1,
                        // }
                    ],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 1,
                            time: 0.1,
                        }
                    ]
                },
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 9,
                            time: 0.1,
                        }
                    ]
                },
            ],
            current_animation: 0,
        });
}