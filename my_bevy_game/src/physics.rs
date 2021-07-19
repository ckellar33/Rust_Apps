use bevy::{input::keyboard, prelude::*};

use crate::player::*;

pub struct Velocity(pub Vec3);
pub struct CircularVelocity {
    pub x_angle: f32,
    pub y_angle: f32,
    pub z_angle: f32,
    pub velocity: f32
}

pub struct Gravity(pub f32);
pub struct JumpVelocity(pub f32);
pub struct AffectedByGravity;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(velocity_system.system())
        .add_system(circular_velocity_system.system())
        .add_system(gravity_system.system());
    }
}

fn gravity_system(time: Res<Time>, jump_velocity: Res<JumpVelocity>, gravity: Res<Gravity>, mut q: Query<&mut Velocity, With<AffectedByGravity>>) {
    for mut velocity in q.iter_mut() {
        if velocity.0.z != 0.0 {
            velocity.0.z -= gravity.0 * time.delta_seconds();
        }
        if velocity.0.z <= -(jump_velocity.0) {
            velocity.0.z = 0.0;
        }
    }
}

fn velocity_system(time: Res<Time>, mut q: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in q.iter_mut() {
        let translation = &mut transform.translation;
        let x = translation.x;
        let y = translation.y;
        let delta = time.delta_seconds();
        transform.translation = Vec3::new(x + velocity.0.x * delta, y + velocity.0.y * delta + velocity.0.z * delta, 0.0);
    }
}

fn circular_velocity_system(mut q: Query<(&mut CircularVelocity, &mut Transform)>) {
    for (c_velocity, mut transform) in q.iter_mut() {
        transform.rotation = Quat::from_rotation_ypr(
            c_velocity.y_angle * std::f32::consts::PI / 180.0,
            c_velocity.x_angle * std::f32::consts::PI / 180.0,
            c_velocity.z_angle * std::f32::consts::PI / 180.0
        );
    }   
}