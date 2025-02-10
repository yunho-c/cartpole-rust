use bevy::{input::keyboard::KeyboardInput, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_startup_system(setup)
        .add_systems(Startup, setup)
        .add_systems(StateTransition, move_cube)
        .run();
}

#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid { half_size: Vec3::from_array([1.0, 1.0, 1.0]) })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    })
    .insert(Cube);

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_cube(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cube>>,
) {
    let mut cube_transform = query.single_mut();

    let mut movement = Vec3::ZERO;

    if input.pressed(KeyCode::ArrowLeft) {
        movement.x -= 0.1;
    }
    if input.pressed(KeyCode::ArrowRight) {
        movement.x += 0.1;
    }
    if input.pressed(KeyCode::ArrowUp) {
        movement.y += 0.1;
    }
    if input.pressed(KeyCode::ArrowDown) {
        movement.y -= 0.1;
    }
    if input.pressed(KeyCode::KeyE) {
        movement.z += 0.1;
    }
    if input.pressed(KeyCode::KeyQ) {
        movement.z -= 0.1;
    }

    cube_transform.translation += movement;
}