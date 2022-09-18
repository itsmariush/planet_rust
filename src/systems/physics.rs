use bevy::prelude::*;
use peroxide::c;

use crate::components::{
    physics::*
};

pub fn simulation_system(
    time: Res<Time>,
    mut simulation: ResMut<SimulationStep>
) {
    simulation.time_elapsed += time.delta_seconds_f64();
    if simulation.time_elapsed >= simulation.time_per_step {
        // passed one or more physics step
        simulation.step += simulation.step_size;
        simulation.time_elapsed -= simulation.time_per_step;
    }
}

pub fn transform_system(
    mut query: Query<(&Trajectory, &mut Transform, Option<&Name>), With<Planet>>,
    simulation: Res<SimulationStep>
) {
    for (trajectory, mut transform, name) in query.iter_mut() {
        if let Some(t) = trajectory.points.get(&simulation.step) {
            transform.translation.x = t.position[0] as f32;
            transform.translation.y = t.position[1] as f32;
            transform.translation.z = t.position[2] as f32;
        }
    }
}

// Calculate Trajectory
pub fn trajectory_system(
    mut query: Query<(Entity, &mut Transform, Option<&Name>), With<Planet>>,
    mut query_traj: Query<&mut Trajectory>,
    query_planet: Query<&Planet>
) {
    // traj_moon.calculate(TrajectoryPoint::new(0.0, vec![r_mag_moon, 0.0, 0.0], vec![0.0, 0.0, v_mag_moon]), Some(environment), 10000);
    /*
    for (entity, mut transform, name) in query.iter_mut() {
        let planet = query_planet.get(entity).unwrap();
        let traj_points = &mut query_traj.get_mut(entity).unwrap().points;
        if let Some(t) = traj_points.pop() {            

            if traj_points.is_empty() {
                // Calculate new trajectory
                if let Some(center) = query_traj.get(entity).unwrap().center{
                    // Orbit Arround other entity
                    let center_planet = query_planet.get(center).unwrap();
                    let center_traj = &query_traj.get(center).unwrap().points;
                    let center_position = center_traj[center_traj.len()-1].position.clone();
                    let center_velocity = center_traj[center_traj.len()-1].velocity.clone();
                    let mu = center_planet.relative_mass(planet);
                    query_traj.get_mut(entity).unwrap().calculate(c![center_position; t.position], c![center_velocity; t.velocity], mu, 0.01, 1);
                } else {
                    // Orbit arround [0.0, 0.0, 0.0]
                }
            }
        } else {
            // TODO: calculate new trajectory from scratch
        }
        
    }
    */
}
