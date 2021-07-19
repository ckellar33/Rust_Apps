use bevy::{prelude::*};

mod player;
mod physics;
mod animation;
mod map;
mod collisons;

use player::*;
use animation::*;
use physics::*;
use collisons::*;
use map::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CollisionPlugin)
        .insert_resource(Gravity(45.0 * 40.0))
        .insert_resource(JumpVelocity(23.0 * 40.0))
        // .add_startup_system(setup_world.system().label("world"))
        // .add_startup_system(setup_player.system().after("world"))
        // .add_system(animate_sprite_system.system())
        // .add_system(player_movement.system())
        // .add_system(world_movement.system())
        // .add_system(world_translation.system())
        // .add_system(position_translation.system())
        .run();
}