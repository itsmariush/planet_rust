use std::time::Instant;

use bevy::prelude::*;
use peroxide::prelude::*;
use peroxide::numerical::ode;
use peroxide::c;

#[derive(Debug)]
pub struct TrajectoryPoint {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
}

#[derive(Debug, Default)]
pub struct Trajectory {
    pub points: Vec<TrajectoryPoint>,
}

#[derive(Component)]
pub struct Sun;
#[derive(Component)]
pub struct Planet {
    pub trajectory: Trajectory,
}

pub const M1: f64 = 333.0;
pub const M2: f64 = 1.0;
pub const MU: f64 = (M1*M2)/(M1+M2);
impl Trajectory {
    pub fn new(points: Vec<TrajectoryPoint>) -> Self {
        Self {
            points: points
        }
    }
    pub fn calculate_trajectory(translation: Vec<f64>, velocity: Vec<f64>, step_size: f64, times: usize) -> Vec<TrajectoryPoint> {
        fn f(st: &mut ode::State<f64>, _: &NoEnv) {
            let value = &st.value;
            let derive = &mut st.deriv;
            
            // current position
            let r = &value[0..3].to_vec();
            let r_norm = r.norm();
          
            // current velocity
            let velocity = &value[3..6];

            // acceleration
            let ax = -r[0] * MU / r_norm.powi(3);
            let ay = -r[1] * MU / r_norm.powi(3);
            let az = -r[2] * MU / r_norm.powi(3);

            derive[0] = velocity[0];
            derive[1] = velocity[1];
            derive[2] = velocity[2];
            derive[3] = ax;
            derive[4] = ay;
            derive[5] = az;
        }

        let mut ode_test = ExplicitODE::new(f);
        let init_state: ode::State<f64> = ode::State::new(
            0.0,
            c![translation; velocity],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            );
        let start = Instant::now();
        let result = ode_test
            .set_initial_condition(init_state)
            .set_method(ExMethod::RK4)
            .set_step_size(step_size)
            .set_times(times)
            .integrate();
        let duration = start.elapsed();
        println!("{result}");
        println!("Time elapsed integrating: {duration:?}");

        let mut points: Vec<TrajectoryPoint> = vec![];
        for n in (0..result.row).rev() {
            let row = result.row(n);
            points.push(
                TrajectoryPoint { 
                    position: row[1..4].to_vec(),
                    velocity: row[4..7].to_vec() 
                });
        }
        return points;
    }
}
