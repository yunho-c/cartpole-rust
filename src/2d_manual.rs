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
        .add_systems(StateTransition, step_physics)
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
    .insert(Cart)
    .insert(Velocity::default());

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
    .insert(Pole)
    .insert(Velocity::default());
}

fn step_physics(
    input: Res<ButtonInput<KeyCode>>,
    mut pole_query: Query<(&mut Transform, &mut Velocity), (With<Pole>, Without<Cart>)>,
    mut cart_query: Query<(&mut Transform, &mut Velocity), (With<Cart>, Without<Pole>)>,
) {
    let (mut cart_transform, mut cart_velocity) = cart_query.single_mut();
    let (mut pole_transform, mut pole_velocity) = pole_query.single_mut();

    // ====
    // Cart
    // ====
    let mut f: f32 = 0.;
    if input.pressed(KeyCode::ArrowLeft) {
        // cart_transform.translation.x -= 5.0;
        f = -INPUT_FORCE;
    }
    if input.pressed(KeyCode::ArrowRight) {
        // cart_transform.translation.x += 5.0;
        f = INPUT_FORCE;
    }
    let a_c = f / CART_MASS;
    cart_velocity.0 += a_c * TAU; // v_c(t) 
    cart_transform.translation.x += cart_velocity.0 * TAU; // x_c(t)

    // ====
    // Pole
    // ====
    pole_transform.translation.x = cart_transform.translation.x; // x_j(t): kinematic constraint
    
    // let pivot = Vec3::new(cart_transform.translation.x, cart_transform.translation.y, cart_transform.translation.z)
    // pole_transform.rotate_around(pivot, )
    pole_transform.rotation.z += pole_velocity.0 * TAU; // theta(t) [in radians]
    if pole_transform.rotation.z > 3.14 { pole_transform.rotation.z -= 3.14; }
    else if pole_transform.rotation.z < -3.14 { pole_transform.rotation.z -= 3.14; }
    let g = 9.81; 
    // let theta_double_dot = (g * pole_transform.rotation.z.sin() - a_c * pole_transform.rotation.z.cos()) / 1.;
    let theta_double_dot = (g * pole_transform.rotation.z.sin() - a_c * pole_transform.rotation.z.cos()) / 1. / 180. * 3.14;
    pole_velocity.0 += theta_double_dot * TAU; // omega(t) [in radians]
}
