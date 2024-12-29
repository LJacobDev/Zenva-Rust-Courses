use crate::components::*;
use crate::constants::*;
use crate::PaddleBundle;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;

        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Player1,
            PaddleBundle::new(right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),   //uses .clone() because two copies will be needed, one for player1, and one for player2
                material: materials.add(ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0))),
                ..default()
            },
        ));

        commands.spawn((
            Player2,
            PaddleBundle::new(left_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),   //clone() isn't needed here because the remaining original mesh_handle instance can be moved into player2
                material: materials.add(ColorMaterial::from(Color::srgb(0.0, 0.0, 1.0))),
                ..default()
            },
        ));
    }
}
