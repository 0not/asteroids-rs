mod components;

mod prelude {
    pub use bevy::{
        prelude::*, 
        sprite::MaterialMesh2dBundle,
        core_pipeline::clear_color::ClearColorConfig,
        render::mesh::Mesh,
        render::mesh::Indices,
        render::render_resource::PrimitiveTopology,
    };
    pub use std::time::{Instant, Duration};
    pub use bevy_rapier2d::prelude::*;

    pub use crate::components::*;
}

use prelude::*;

// Define game configuration constants
// const TIME_STEP: f32 = 1.0 / 60.0;
const SHIP_SIZE: f32 = 20.0;

const SHIP_COLOR:   Color = Color::WHITE;
const BULLET_COLOR: Color = Color::WHITE;
const BACK_COLOR:   Color = Color::BLACK;

// Define ship vertices
fn player_ship_vertices(ship_size: f32) -> Vec<[f32; 3]> {
    let s = (ship_size.powf(2.) - (ship_size/2.).powf(2.)).sqrt() / 2.;

    vec![[0., ship_size, 0.], [s, -ship_size/2., 0.], [-s, -ship_size/2., 0.]]
}

fn player_ship_collider(ship_size: f32) -> Collider {
    let verts = player_ship_vertices(ship_size);
    let points: Vec<_> = verts
        .iter()
        .map(|x| Vec2::new(x[0], x[1]))
        .collect();


    Collider::convex_hull(&points).unwrap_or(Collider::ball(ship_size))
}

fn create_player_ship_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, player_ship_vertices(SHIP_SIZE));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);
    mesh.set_indices(Some(Indices::U32(vec![0, 2, 1])));
    mesh
}

fn setup_camera(mut commands: Commands) {
    // Setup camera with black background
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(BACK_COLOR),
        },
        ..default()
    });
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let player_pos: Vec2 = Vec2::new(0.0, 0.0);

    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(create_player_ship_mesh()).into(),
            material: materials.add(ColorMaterial::from(SHIP_COLOR)),
            transform: Transform::from_translation(player_pos.extend(0.0)),
            ..default()
        },
        PlayerShip,
        RigidBody::Dynamic,
        Velocity { linvel: Vec2::ZERO, angvel: 0.0},
        Damping { linear_damping: 0.5, angular_damping: 10. },
        ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        },
        player_ship_collider(SHIP_SIZE),
    )).with_children(|parent| {
        parent.spawn((
            Gun { 
                last_fired: time.startup() + time.elapsed(), 
                cooldown:   Duration::from_millis(100),
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec2::new(0.0, SHIP_SIZE).extend(0.0)),
                ..default()
            },
        ));
    });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut ExternalForce), With<PlayerShip>>,
) {
    
    for (transform, mut external_force) in query.iter_mut() {
        let mut linear   = 0.0;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::W) {
            linear += 1.0;
        }

        // if keyboard_input.pressed(KeyCode::S) {
        //     linear -= 1.0;
        // }

        if keyboard_input.pressed(KeyCode::D) {
            rotation -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            rotation += 1.0;
        }

        external_force.force  = 5e6 * linear * transform.up().truncate() * time.delta_seconds();
        external_force.torque = 75e6 * rotation * time.delta_seconds();
    }

}

fn periodic_bc(
    windows: Res<Windows>,
    mut query: Query<&mut Transform>,
) {
    for mut transform in query.iter_mut() {
        let window = windows.get_primary().unwrap();
        let (w, h) = (window.width(), window.height());
        let (x, y) = (transform.translation.x, transform.translation.y);

        if x > w/2. {
            transform.translation.x -= w;
        }
        else if x < -w/2. {
            transform.translation.x += w;
        }
        else if y > h/2. {
            transform.translation.y -= h;
        }
        else if y < -h/2. {
            transform.translation.y += h;
        }
    }
}

fn spawn_bullet(
    commands:  &mut Commands,
    meshes:    &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    velocity: Vec2,
) {
    commands.spawn((
        Bullet,
        LifeTime { timer: Timer::new(Duration::from_millis(1500), TimerMode::Once) },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
            material: materials.add(ColorMaterial::from(BULLET_COLOR)),
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Velocity { linvel: velocity, angvel: 0.0},
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
    ));
}

fn fire_player_gun(
    mut commands:   Commands,
    mut meshes:     ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    ships: Query<(&Transform, &Velocity), With<PlayerShip>>,
    mut guns:  Query<(&GlobalTransform, &mut Gun)>,
) {
    for (ship_trans, ship_vel) in ships.iter() {
        // TODO: This only works with one gun, not extensible.
        let (gun_pos, mut gun) = guns.get_single_mut().unwrap();

        // Skip if not enough time since last firing.
        let now = time.startup() + time.elapsed();
        if now - gun.last_fired < gun.cooldown {
            continue
        }

        if keyboard_input.pressed(KeyCode::Space) {
            println!("Fire!");

            gun.last_fired = now;

            let ship_dir = ship_trans.up().truncate();
            let position = gun_pos.translation().truncate();
            let velocity = ship_dir * 500.0 + ship_vel.linvel;

            spawn_bullet(
                &mut commands,
                &mut meshes,
                &mut materials,
                position,
                velocity,
            );
        }
    }
}

fn tick_lifetime(
    time: Res<Time>,
    mut query: Query<&mut LifeTime>,
) {
    for mut lifetime in query.iter_mut() {
        lifetime.timer.tick(time.delta());
    }
}

fn despawn_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &LifeTime), With<Bullet>>,
) {
    for (entity, lifetime) in query.iter_mut() {
        if lifetime.timer.finished() {
            // Despawn
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Asteroids Clone".into(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player)
        .add_system(move_player)
        .add_system(fire_player_gun)
        .add_system(periodic_bc)
        .add_system(tick_lifetime)
        .add_system(despawn_bullet)
        .run();
}

