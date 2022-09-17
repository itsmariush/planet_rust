use bevy::prelude::*;
use peroxide::c;

use crate::components::{
    physics::*
};

pub fn gravity_system(
    mut query: Query<(&mut Planet, &mut Transform, &Name)>,
    query_planet: Query<&Planet>
) {
    for (mut planet, mut transform, name) in query.iter_mut() {
        match planet.trajectory.pop() {
            Some(t) => {
                transform.translation.x = t.position[0] as f32;
                transform.translation.y = t.position[1] as f32;
                transform.translation.z = t.position[2] as f32;
                if planet.trajectory.is_empty() {
                    println!("Trajectory of {name} is empty.");
                    let mut pos: Vec<f64> = vec![];
                    if let Some(ent) = planet.parent {
                        if let Ok(parent) = query_planet.get(ent) {
                            if !parent.trajectory.is_empty() {
                                pos.extend(parent.trajectory[parent.trajectory.len()-1].position.iter());
                            }
                        }
                    }
                    planet.trajectory = Planet::calculate_trajectory(c![pos; t.position], c![vec![0.0, 0.0, 0.0]; t.velocity], MU, 0.01, 1);
                }
            },
            _ => {}
        }
    }
}
