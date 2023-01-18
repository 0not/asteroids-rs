// use crate::player_ship;
use crate::prelude::*;
use rand::prelude::*;

pub fn setup_camera(
    mut commands: Commands,
    settings: Res<Settings>,
) {
    // Setup camera with black background
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(settings.back_color),
        },
        ..default()
    });
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    settings: Res<Settings>,
) {
    let player_pos: Vec2 = Vec2::new(0.0, 0.0);

    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(player_ship::mesh(settings.ship.size)).into(),
            material: materials.add(ColorMaterial::from(settings.ship.color)),
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
        player_ship::collider(settings.ship.size),
    )).with_children(|parent| {
        parent.spawn((
            Gun { 
                last_fired: time.startup() + time.elapsed(), 
                cooldown:   Duration::from_millis(100),
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec2::new(0.0, 1.05*settings.ship.size).extend(0.0)),
                ..default()
            },
        ));
    });
}

pub fn setup_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
) {
    // let pos: Vec2 = Vec2::new(100.0, 0.0);
    let mut rng = rand::thread_rng();
    
    for _ in 0..5 {
        let (collider, mesh) = asteroid::collider_and_mesh(settings.ship.size);
        let x  = rng.gen::<f32>() * 1000. - 500.;
        let y  = rng.gen::<f32>() * 1000. - 500.;
        let vx = rng.gen::<f32>() * 500. - 250.;
        let vy = rng.gen::<f32>() * 500. - 250.;

        let pos = Vec2::new(x, y);
        let vel = Vec2::new(vx, vy);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(ColorMaterial::from(settings.ship.color)),
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            Asteroid,
            RigidBody::Dynamic,
            Velocity { linvel: vel, angvel: 0.0},
            Damping { linear_damping: 0.0, angular_damping: 0. },
            ExternalForce {
                force: Vec2::ZERO,
                torque: 0.0,
            },
            collider,
            Restitution::coefficient(1.),
        ));
    }
}