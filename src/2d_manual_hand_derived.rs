use bevy::prelude::*;

const INPUT_FORCE: f32 = 400000.0;

const TAU: f32 = 0.01;

const CART_DENSITY: f32 = 0.1;
const POLE_DENSITY: f32 = 0.1;

const CART_SIZE: Vec3 = Vec3::new(100.0, 40.0, 0.0);
const POLE_SIZE: Vec3 = Vec3::new(10.0, 100.0, 0.0);

const CART_MASS: f32 = CART_SIZE.x * CART_SIZE.y * CART_DENSITY;
const POLE_MASS: f32 = POLE_SIZE.x * POLE_SIZE.y * POLE_DENSITY;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, control_cart)
        .add_systems(Update, update_pole)
        .run();
}

#[derive(Component)]
struct Cart;

#[derive(Component)]
struct Pole;

#[derive(Component)]
struct Velocity(f32);

impl Velocity {
    pub fn default() -> Self {
        Self(0.)
    }
}

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2d);

    // Spawn the cart
    commands.spawn(Sprite {
        color: Color::srgb(0.8, 0.7, 0.6),
        custom_size: Some(CART_SIZE.truncate()),
        ..Default::default()
    })
    .insert((
        Transform::from_xyz(0.0, -200.0, 0.0),
        Cart,
        Velocity::default()
    ));

    // Spawn the pole
    commands.spawn(Sprite {
        color: Color::srgb(0.4, 0.4, 0.4),
        custom_size: Some(POLE_SIZE.truncate()),
        ..Default::default()
    })
    .insert((
        Transform::from_xyz(0.0, -100.0, 0.0),
        Pole,
        Velocity::default()
    ));
}

fn control_cart(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Cart>>,
) {
    let (mut cart_transform, mut cart_velocity) = query.single_mut();

    let mut f: f32 = 0.;
    if input.pressed(KeyCode::ArrowLeft) {
        f = -INPUT_FORCE;
    }
    if input.pressed(KeyCode::ArrowRight) {
        f = INPUT_FORCE;
    }

    // Update the cart's velocity and position
    cart_velocity.0 += f / CART_MASS * TAU;
    cart_transform.translation.x += cart_velocity.0 * TAU;
}

fn update_pole(
    cart_query: Query<&Transform, (With<Cart>, Without<Pole>)>,
    mut pole_query: Query<&mut Transform, (With<Pole>, Without<Cart>)>,
) {
    let cart_transform = cart_query.single();
    let mut pole_transform = pole_query.single_mut();

    // Keep the pole directly above the cart
    pole_transform.translation.x = cart_transform.translation.x;

}
