use bevy::prelude::*;
mod systems;
use systems::*;

mod constants;

mod components;
use components::*;

mod bundles;
use bundles::*;

fn main() {
    /*
       This pong game is going to have a program structure with the intention to modularize it so that it has improved organization, maintainability, and scalability

       it will be following the ECS Entity-component-system paradigm

       so first a components.rs file is made that defines ECS components in the game, like player, ball, and paddle.

       These components will be centralized there, making a single place to manage all the data types attached to entities, making it easier to track and modify


       Next, a systems directory is made

       This contains all the individual modules for different systems in the game

       each module handles a specific aspect of the game logic, ensuring separation of concerns

       mod.rs file is created in it as it will export the modules for main.rs to use, just by referencing the system module


       Next we create ball.rs which contains systems related to the ball's behaviour, like spawning, movement, and scoring detection

       And paddle.rs contains systems for input from player one and player two

       scoreboard.rs handles the systems related to the scoreboard, such as updating and displaying scores

       collision.rs contains systems that detect collisions and interaction between the ball and paddles

       Next, in the src directory, a bundles.rs is made.  This defines the entity bundles, which are components that are frequently used together.  For example, a ball bundle might include ball, position

       using these bundles helps streamline entity creation, ensuring that all the required components are included

       lastly, there is constants.rs which contains the constant values used throughout the game, like speed, dimension, other config settings we might need.  Having them centralized here helps to change the values and making sure it propagates throughout the code base from one place.


       This program structure allows us to break down the modules according to their functionality

       By separating components, system and bundles, it means that each file has a clear responsibility and it helps manage complexity, and reduces likelihood of errors

       This will be really useful as the game grows in complexity,

       and new features and systems can be added without disrupting the existing code that much

    */

    //the plugins being added cover functionalities like window creation, event handling, input management, rendering

    App::new()
        .add_plugins(DefaultPlugins.set(create_window()))
        .add_systems(Startup, (spawn_dotted_line, spawn_ball, spawn_paddles, spawn_camera))
        .run();
}
