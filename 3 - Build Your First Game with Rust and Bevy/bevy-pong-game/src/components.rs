use bevy::math::Vec2;
use bevy::prelude::*; //this 'imports' the common bevy types and functions //a vector of x and y coordinates

#[derive(Component)] //this macro implements the traits for the struct to be used as a component in Bevy's ECS
                     //This struct has no fields and its primary purpose is to serve as a tag to identify entities representing player 1
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(Component)]
pub struct Player1Score;

#[derive(Component)]
pub struct Player2Score;

pub enum Scorer {
    Player1,
    Player2,
}

#[derive(Event)]
pub struct Scored(pub Scorer); //this is used to create events indicating that a player has scored

#[derive(Resource, Default)]
pub struct Score {
    pub player1: u32,
    pub player2: u32,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Boundary; //this is for the boundaries at the top and bottom of the screen that deflect the ball

#[derive(Component)]
pub struct Shape(pub Vec2); //to store the size of an entity in 2d space

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);
