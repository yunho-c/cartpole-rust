use bevy::prelude::*;

const CART_SIZE: Vec3 = Vec3::new(100.0, 40.0, 0.0);
const POLE_SIZE: Vec3 = Vec3::new(10.0, 100.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(StateTransition, control_cart)
        .add_systems(StateTransition, update_pole)
        .run();
}

#[derive(Component)]
struct Cart;

#[derive(Component)]
struct Pole;

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::new_with_far(1.0));

    // Spawn the cart
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.8, 0.7, 0.6),
            custom_size: Some(CART_SIZE.truncate()),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -200.0, 0.0),
        ..Default::default()
    })
    .insert(Cart);

    // Spawn the pole
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.4, 0.4, 0.4),
            custom_size: Some(POLE_SIZE.truncate()),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -100.0, 0.0),
        ..Default::default()
    })
    .insert(Pole);
}

fn control_cart(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cart>>,
) {
    let mut cart_transform = query.single_mut();

    if input.pressed(KeyCode::ArrowLeft) {
        cart_transform.translation.x -= 5.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        cart_transform.translation.x += 5.0;
    }
}

fn update_pole(
    cart_query: Query<&Transform, With<Cart>>,
    mut pole_query: Query<&mut Transform, (With<Pole>, Without<Cart>)>,
) {
    let cart_transform = cart_query.single();
    let mut pole_transform = pole_query.single_mut();

    pole_transform.translation.x = cart_transform.translation.x;
}
