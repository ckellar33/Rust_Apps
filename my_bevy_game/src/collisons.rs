use bevy::{input::keyboard, prelude::*, sprite::collide_aabb::{collide, Collision}};

use crate::player::*;
use crate::physics::*;

enum Collider {
    Solid,
    Scorable,
    Paddle,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_collisions.system())
            .add_system(player_collision_system.system());
    }
}

fn setup_collisions(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 1.0),
            // sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            sprite: Sprite::new(Vec2::new(200.0, 200.0)),
            ..Default::default()
        })
        .insert(Collider::Solid);
}

fn player_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&mut Velocity, &Transform, &TextureAtlasSprite), With<Player>>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    if let Ok((mut velocity, player_transform, sprite)) = player_query.single_mut() {
        let player_size = Vec2::new(24.0 * 6.0, 24.0 * 6.0);//sprite.size;
        // check collision with walls
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    // scoreboard.score += 1;
                    commands.entity(collider_entity).despawn();
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = velocity.0.x > 0.0,
                    Collision::Right => reflect_x = velocity.0.x < 0.0,
                    Collision::Top => reflect_y = velocity.0.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.0.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.0.x = -velocity.0.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.0.y = -velocity.0.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is
                // also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}