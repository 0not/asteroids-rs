// use crate::player_ship;
use crate::prelude::*;
use rand::prelude::*;

use crate::player_ship::{Gun, PlayerShipBundle};

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
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    settings: Res<Settings>,
) {
    let player_pos: Vec2 = Vec2::new(0.0, 0.0);

    let player_ship_bundle = PlayerShipBundle::new(
        &player_pos, 
        settings.ship.size,
        1000,
        ColorMaterial::from(settings.ship.color),
        meshes,
        materials,
    );
    commands
        .spawn(player_ship_bundle)
        .with_children(|parent| {  // TODO: Add children bundle
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
    let mut rng = rand::thread_rng();
    
    for _ in 0..10 {
        
        let x  = rng.gen::<f32>() * 1000. - 500.;
        let y  = rng.gen::<f32>() * 1000. - 500.;
        let vx = rng.gen::<f32>() * 300. - 150.;
        let vy = rng.gen::<f32>() * 300. - 150.;

        let pos = Vec2::new(x, y);
        let vel = Vec2::new(vx, vy);

        asteroid::spawn_asteroid(&mut commands, &mut meshes, &mut materials, &settings, pos, vel, asteroid::AsteroidSize::LARGE);
    }
}

