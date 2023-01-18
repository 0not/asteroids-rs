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