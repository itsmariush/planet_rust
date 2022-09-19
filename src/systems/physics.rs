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
    query_planet: Query<&Planet>,
    simulation: Res<SimulationStep>
) {
    for (entity, mut transform, name) in query.iter_mut() {
        let planet = query_planet.get(entity).unwrap();
        let traj_points = &query_traj.get(entity).unwrap().points;
        let traj_center = query_traj.get(entity).unwrap().center;
        let traj_rel_mass = query_traj.get(entity).unwrap().relative_mass;
        let next_step = simulation.step + simulation.step_size;
        if !traj_points.contains_key(&next_step) {
            // Calculation trajectory
            println!("Calculate next trajectory");
            let current_point = traj_points.get(&simulation.step).expect("No current TrajectoryPoint found to calculate next").clone();
            let mut env = DeriveEnv::empty(traj_rel_mass);
            env.current_step = next_step;
            if let Some(center) = traj_center {
                env.points = query_traj.get(center).unwrap().points.clone();
            } 
            println!("Current Point: {current_point:?}, Mass: {}, Step: {}", env.relative_mass, env.current_step);
            query_traj.get_mut(entity).unwrap().calculate(&current_point, Some(env), 1600);
        }
    }
}
