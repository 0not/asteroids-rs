use crate::prelude::*;


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

pub struct ReceivedDamageEvent {
    pub to: Entity,
    pub from: Entity,
    pub damage: u32,
}

pub fn collision(
    mut query: Query<&Health>,
    mut collision_events: EventReader<CollisionEvent>,
    mut dmg_event: EventWriter<ReceivedDamageEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);

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
            damaged_health.value -= ev.damage;
        }
    }
}

pub fn live_or_die(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0 {
            commands.entity(entity).despawn();
        }
    }
}