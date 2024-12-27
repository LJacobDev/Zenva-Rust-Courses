use bevy::prelude::*;

//Commands is a Bevy resource that allows you to create, modify or delete entities and their components
pub fn spawn_camera(mut commands: Commands) {
    
    //Camera2dBundle is defined in bevy::prelude
    commands.spawn(Camera2dBundle::default());

    //bevy supports multiple cameras, each potentially rendering different parts of the game world or used for different purposes, like split screen / multiplayer
}
