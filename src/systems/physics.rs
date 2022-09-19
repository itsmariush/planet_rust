use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::components::{
    physics::*
};

// Update the simulation step when enough time passed
pub fn simulation_system(
    time: Res<Time>,
    mut simulation: ResMut<SimulationStep>
) {
    // TODO: maybe rework this
    simulation.time_elapsed += time.delta_seconds_f64();
    if simulation.time_elapsed >= simulation.time_per_step {
        // passed one or more physics step
        simulation.step += simulation.step_size;
        simulation.time_elapsed -= simulation.time_per_step;
    }
}

pub fn debug_system(
    query: Query<&Trajectory>,
    simulation: Res<SimulationStep>,
    mut lines: ResMut<DebugLines>
) {
    let sim_step = simulation.step as usize;
    for trajectory in query.iter() {
        let draw_step = 120;
        let points = &trajectory.points;
        for p in (0..trajectory.points.len()).step_by(draw_step) {
            let pos_start = &points[&(p as u64)].position;
            let end_option = &trajectory.points.get(&((p + draw_step) as u64)); //unwrap_or(&points[&0]).position;
            if end_option.is_none() {
                break;
            }
            let pos_end = &end_option.unwrap().position;
            lines.line(Vec3::new(pos_start[0] as f32, pos_start[1] as f32, pos_start[2] as f32), Vec3::new(pos_end[0] as f32, pos_end[1] as f32, pos_end[2] as f32), 0.0);
        }
    }
}

// Update Planets based on simulation step
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
    mut query: Query<Entity, With<Planet>>,
    mut query_traj: Query<&mut Trajectory>,
    simulation: Res<SimulationStep>
) {
    // TODO: clean up / refactor
    for entity in query.iter_mut() {
        let traj_points = &query_traj.get(entity).unwrap().points;
        let traj_center = query_traj.get(entity).unwrap().center;
        let traj_rel_mass = query_traj.get(entity).unwrap().relative_mass;
        let next_step = simulation.step + simulation.step_size;
        if !traj_points.contains_key(&next_step) {
            // Calculate trajectory
            let current_point = traj_points.get(&simulation.step).expect("No current TrajectoryPoint found to calculate next").clone();
            let mut env = DeriveEnv::empty(traj_rel_mass);
            env.current_step = next_step;
            if let Some(center) = traj_center {
                env.points = query_traj.get(center).unwrap().points.clone();
            } 
            query_traj.get_mut(entity).unwrap().calculate(&current_point, Some(env), 10000);
        }
    }
}
