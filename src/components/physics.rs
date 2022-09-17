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

pub const G: f64 = 6.67259e-20;
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
            let r1 = &value[0..3].to_vec(); 
            let r2 = &value[3..6].to_vec();
            let r_norm = vec![r2[0] - r1[0], r2[1] - r1[1], r2[2] - r1[2]].norm();
          
            // current velocity
            let v1 = &value[6..9];
            let v2 = &value[9..12];

            let ax1 = r1[0] * MU / r_norm.powi(3);
            let ay1 = r1[1] * MU / r_norm.powi(3);
            let az1 = r1[2] * MU / r_norm.powi(3);

            // current acceleration
            let ax2 = -r2[0] * MU / r_norm.powi(3);
            let ay2 = -r2[1] * MU / r_norm.powi(3);
            let az2 = -r2[2] * MU / r_norm.powi(3);
            
            derive[0] = v1[0];
            derive[1] = v1[1];
            derive[2] = v1[2];
            derive[3] = v2[0];
            derive[4] = v2[1];
            derive[5] = v2[2];
            derive[6] = ax1;
            derive[7] = ay1;
            derive[8] = az1;
            derive[9] = ax2;
            derive[10] = ay2;
            derive[11] = az2;
        }

        let mut ode_test = ExplicitODE::new(f);
        let init_state: ode::State<f64> = ode::State::new(
            0.0,
            c![translation; velocity],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
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


        let mut points1: Vec<TrajectoryPoint> = vec![];
        for n in (0..result.row).rev() {
            let row = result.row(n);
            points1.push(
                TrajectoryPoint { 
                    position: row[1..4].to_vec(),
                    velocity: row[7..10].to_vec() 
                });
        }
        return points1;
    }
}
