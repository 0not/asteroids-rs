use crate::prelude::*;

use bevy::{
    render::mesh::Mesh,
    render::mesh::Indices,
    render::render_resource::PrimitiveTopology,
};

const BULLET_COLOR: Color = Color::WHITE;

// Define ship vertices
pub fn vertices(ship_size: f32) -> Vec<[f32; 3]> {
    let s = (ship_size.powf(2.) - (ship_size/2.).powf(2.)).sqrt() / 2.;

    vec![[0., ship_size, 0.], [s, -ship_size/2., 0.], [-s, -ship_size/2., 0.]]
}

pub fn collider(ship_size: f32) -> Collider {
    let verts = vertices(ship_size);
    let points: Vec<_> = verts
        .iter()
        .map(|x| Vec2::new(x[0], x[1]))
        .collect();


    Collider::convex_hull(&points).unwrap_or(Collider::ball(ship_size))
}

pub fn mesh(ship_size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices(ship_size));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);
    mesh.set_indices(Some(Indices::U32(vec![0, 2, 1])));
    mesh
}

// Fire player gun
pub fn fire_gun(
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

// TODO:  Move these bullet functions elsewhere.
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
        Collider::ball(2.0),
        // Bullets are group 2, and can only interact with GROUP_3 (asteroids)
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
    ));
}

pub fn despawn_bullet(
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

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    settings: Res<Settings>,
    mut query: Query<(&Transform, &mut ExternalForce), With<PlayerShip>>,
) {
    
    for (transform, mut external_force) in query.iter_mut() {
        let mut linear   = 0.0;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::W) {
            linear += 1.0;
        }

        // TODO: Have power up that enables reverse thrusters
        // if keyboard_input.pressed(KeyCode::S) {
        //     linear -= 1.0;
        // }

        if keyboard_input.pressed(KeyCode::D) {
            rotation -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            rotation += 1.0;
        }

        external_force.force  = settings.ship.force * linear * transform.up().truncate() * time.delta_seconds();
        external_force.torque = settings.ship.torque * rotation * time.delta_seconds();
    }

}

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(fire_gun)
            .add_system(despawn_bullet)
            .add_system(move_player);
    }
}