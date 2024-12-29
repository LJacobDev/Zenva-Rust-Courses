use crate::components::*;
use crate::constants::*;
use crate::BallBundle;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0));
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        BallBundle::new(1.0, 0.0),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

// they seem to put something that draws the paddles positions here
// in a catch all function that influences both the ball and the paddles,
// rather than giving each system their own, or using something that is
// decoupled from both of them to have it defined in one place there
pub fn update_entity_positions(mut entity: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in entity.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}

pub fn detect_scoring(
    mut ball_query: Query<&mut Position, With<Ball>>,
    window_query: Query<&Window>,
    mut score_event_write: EventWriter<Scored>,
) {
    let window = match window_query.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let half_window_width = window.resolution.width() / 2.0;
    let ball_position = match ball_query.get_single_mut() {
        Ok(bp) => bp,
        Err(_) => return,
    };

    if ball_position.0.x > half_window_width {
        score_event_write.send(Scored(Scorer::Player2));
    } else if ball_position.0.x < -half_window_width {
        score_event_write.send(Scored(Scorer::Player1));
    }
}

/// The instructor seems to have it set so that this function is called on every update and reads the event list for a scored event, rather than being something that is only invoked at some point where a score event is detected first by something else watching for it
pub fn respawn_ball(
    mut ball_query: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    if let Ok((mut position, mut velocity)) = ball_query.get_single_mut() {
        for event in events.read(){
            //a scoring event has just happened, so reposition the ball
            position.0 = Vec2::new(0., 0.);

            //now give it a random velocity
            let mut rng = rand::thread_rng();

            let pi = std::f32::consts::PI;
            
            //first establish a random angle for the ball's velociry vector

            let angle: f32 = match event.0{
                Scorer::Player1 => rng.gen_range(-pi/4.0 .. pi/4.0),
                Scorer::Player2 => rng.gen_range(3.0 * pi/4.0 .. 5.0 * pi/4.0)
            }; 

            //give the ball a new velocity vector based on the randon angle
            velocity.0 = Vec2::new(angle.cos(), angle.sin());
        }
    }
}
