use crate::prelude::*;
use crate::player_ship::{PlayerShip};


pub fn periodic_bc(
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

pub fn tick_lifetime(
    time: Res<Time>,
    mut query: Query<&mut LifeTime>,
) {
    for mut lifetime in query.iter_mut() {
        lifetime.timer.tick(time.delta());
    }
}

#[derive(Debug)]
pub struct ReceivedDamageEvent {
    pub to: Entity,
    pub from: Entity,
    pub damage: i32,
}

pub fn collision(
    mut query: Query<&Health>,
    mut collision_events: EventReader<CollisionEvent>,
    mut dmg_event: EventWriter<ReceivedDamageEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            if let Ok(_health) = query.get_mut(*e1) {
                //health.value  -= 1; // TODO: I should really fire an event here...
                dmg_event.send(ReceivedDamageEvent { to: *e1, from: *e2, damage: 1 });
            }

            if let Ok(_health) = query.get_mut(*e2) {
                // health.value  -= 1; // TODO: I should really fire an event here...
                dmg_event.send(ReceivedDamageEvent { to: *e2, from: *e1, damage: 1 });
            }
        }
    }
}

pub fn propagate_damage(
    mut dmg_event: EventReader<ReceivedDamageEvent>,
    mut query: Query<&mut Health>,
) {
    for ev in dmg_event.iter() {
        if let Ok(mut damaged_health) = query.get_mut(ev.to) {
            if damaged_health.value > 0 {
                damaged_health.value -= ev.damage;
            }
        }
    }
}

pub fn shorten_bullet_lifetime(
    mut dmg_event: EventReader<ReceivedDamageEvent>,
    mut query: Query<&mut LifeTime, With<Bullet>>,
) {
    for ev in dmg_event.iter() {
        // The Bullet is only ever in the `from` field, since it does not have health.
        if let Ok(mut lifetime) = query.get_mut(ev.from) {
            println!("Shorten lifetime.");
            if lifetime.timer.remaining_secs() > 0.1 {
                lifetime.timer = Timer::new(Duration::from_millis(100), TimerMode::Once);
            }
        }
    }
}

pub fn dmg_event_debug(
    mut dmg_event: EventReader<ReceivedDamageEvent>,
) {
    for ev in dmg_event.iter() {
        eprintln!("Entity {:?} damaged by {:?}!", ev.to, ev.from);
    }
}


pub fn despawn_player(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<PlayerShip>>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0 {
            // TODO: Do more than despawn. Game over.
            commands.entity(entity).despawn();
        }
    }
}