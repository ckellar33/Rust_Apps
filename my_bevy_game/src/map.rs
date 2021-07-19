use std::path::PathBuf;

use bevy::{prelude::*, render::camera::Camera};
use bevy_ldtk::*;
use bevy_retrograde::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LdtkPlugin)
            .add_plugins(RetroPlugins)
            .add_startup_system(map_setup.system());
            // .add_system(spawn_player.system());
    }
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Enable hot reload
    asset_server.watch_for_changes().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // commands
    //     // Spawn the map
    //     .spawn()
    //     .insert_bundle(LdtkMapBundle {
    //         map: asset_server.load(PathBuf::from(
    //             &std::env::args().nth(1).unwrap_or("maps/Untitled.ldtk".into()),
    //         )),
    //         config: LdtkMapConfig {
    //             set_clear_color: true,
    //             scale: 1.0,
    //             level: std::env::args()
    //                 .nth(2)
    //                 .map(|x| x.parse().unwrap())
    //                 .unwrap_or(0),
    //             center_map: false,
    //         },
    //         ..Default::default()
    //     });

    // Spawn the map
    commands.spawn().insert_bundle(LdtkMapBundle {
        map: asset_server.load("maps/map1.ldtk"),
        // We offset the map a little to move it more to the center of the screen, because maps are
        // spawned with (0, 0) as the top-left corner of the map
        transform: Transform::from_xyz(-200., -100., 0.),
        ..Default::default()
    });
}

/// This system demonstrates how to get information out of the map, such as entity locations, and
/// spawn a sprite at the location of the entity
fn spawn_player(
    mut commands: Commands,
    printed_maps: Local<Vec<Entity>>,
    query: Query<(Entity, &Handle<LdtkMap>)>,
    map_assets: Res<Assets<LdtkMap>>,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (ent, handle) in query.iter() {
        // Skip any maps we have already printed the spawn location for
        if printed_maps.contains(&ent) {
            continue;
        }

        // If the map asset has finished loading
        if let Some(map) = map_assets.get(handle) {
            // This is the default level, but if you spawned a different level, put that ID here
            let level_idx = 0;

            // Get the level from the project
            let level = &map.project.levels[level_idx];

            // Find the entities layer
            let entities_layer = level
                .layer_instances
                .as_ref() // get a reference to the layer instances
                .unwrap() // Unwrap the option ( this could be None, if there are no layers )
                .iter() // Iterate over the layers
                .filter(|&x| x.__identifier == "Entities") // Filter on the name of the layer
                .next() // Get it
                .unwrap(); // Unwrap it ( would be None if it could not find a layer "MyEntities" )

            // Get the specific entity you want
            let player_start = entities_layer
                .entity_instances
                .iter() // Iterate over our entities in the layer
                .filter(|x| x.__identifier == "Player_Spawn") // Find the one we want
                .next() // Get it
                .unwrap(); // Unwrap it

            // Get the number of layers in the map and add one to it: this is how high we need to
            // spawn the player so that he is on top of all the maps
            let player_z = level.layer_instances.as_ref().unwrap().len() as f32 + 1.0;

            // Spawn the entity!
            commands.spawn().insert_bundle(SpriteBundle {
                // Set your sprite stuff
                transform: Transform::from_xyz(
                    // The player x position is the entity's x position from the map data
                    player_start.px[0] as f32,
                    // The player y position is the entity's y position from the map data, but
                    // multiplied by negative one because in the LDtk map +y means down and not up.
                    player_start.px[1] as f32 * -1.0,
                    // Spawn the player with the z value we determined earlier
                    player_z,
                ),
                material: color_materials.add(ColorMaterial {
                    texture: Some(asset_server.load("textures/character.png")),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
    }
}