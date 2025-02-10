#[allow(unused_variables)]

use bevy::prelude::*;

const CART_HEIGHT: f32 = 1.0;
const CART_WIDTH:  f32 = 3.0;
const CART_LENGTH: f32 = 2.0;

const POLE_DIAMETER: f32 = 0.2;
const POLE_LENGTH:   f32 = 4.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(CART_LENGTH, CART_HEIGHT, CART_WIDTH)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, CART_HEIGHT / 2., 0.0),
        ..default()
    });
    // pole
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(POLE_DIAMETER / 2., POLE_LENGTH)),
        material: materials.add(Color::rgb_u8(20, 20, 20)),
        transform: Transform::from_xyz(0.0, CART_HEIGHT, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
