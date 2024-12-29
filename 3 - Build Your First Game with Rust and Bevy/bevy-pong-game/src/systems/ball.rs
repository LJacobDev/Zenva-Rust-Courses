use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::BallBundle;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0));
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        BallBundle::new(1.0, 0.0), 
        MaterialMesh2dBundle{
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        }));
}


pub fn move_ball(
    mut ball: Query<(&mut Position, &Velocity), With<Ball>>,

){
    if let Ok((mut position, velocity)) = ball.get_single_mut(){
        position.0 += velocity.0 * BALL_SPEED;
    }
}

// they seem to put something that draws the paddles positions here 
// in a catch all function that influences both the ball and the paddles, 
// rather than giving each system their own, or using something that is
// decoupled from both of them to have it defined in one place there
pub fn update_entity_positions(
    mut entity: Query<(&mut Transform, &Position)>
){
    for(mut transform, position) in entity.iter_mut(){
        transform.translation = position.0.extend(0.);
    }
}


pub fn detect_scoring(
    mut ball_query: Query<&mut Position, With<Ball>>,
    window_query: Query<&Window>,
    mut score_event_write: EventWriter<Scored>,
){
    let window = match window_query.get_single(){
        Ok(w) => w,
        Err(_) => return,
    };

    let half_window_width = window.resolution.width() / 2.0;
    let ball_position = match ball_query.get_single_mut(){
        Ok(bp) => bp,
        Err(_) => return,
    };

    if ball_position.0.x > half_window_width{
        score_event_write.send(Scored(Scorer::Player2));
    } else if ball_position.0.x < -half_window_width{
        score_event_write.send(Scored(Scorer::Player1));
    }
}
