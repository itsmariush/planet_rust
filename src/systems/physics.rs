use bevy::prelude::*;

use crate::components::{
    physics::*
};

pub fn gravity_system(
    mut query: Query<(&mut Planet, &mut Transform)>,
) {
    for (mut planet, mut transform) in query.iter_mut() {
        match planet.trajectory.pop() {
            Some(t) => {
                transform.translation.x = t.position[0] as f32;
                transform.translation.y = t.position[1] as f32;
                transform.translation.z = t.position[2] as f32;
                if planet.trajectory.is_empty() {
                    planet.trajectory = Planet::calculate_trajectory(t.position, t.velocity, MU, 0.01, 10);
                }
            },
            _ => {}
        }
    }
}
