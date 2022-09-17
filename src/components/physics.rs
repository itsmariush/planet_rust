use std::time::Instant;

use bevy::prelude::*;
use peroxide::prelude::*;
use peroxide::numerical::ode;
use peroxide::c;

#[derive(Debug, Default)]
pub struct TrajectoryPoint {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
}

#[derive(Debug, Default)]
pub struct PlanetInfo {
}

#[derive(Component)]
pub struct Sun;
#[derive(Component)]
pub struct Planet {
    pub mass: f64,
    pub trajectory: Vec<TrajectoryPoint>,
    pub parent: Option<Entity>,
}

pub const M1: f64 = 333.0;
pub const M2: f64 = 1.0;
pub const MU: f64 = (M1*M2)/(M1+M2);

impl Planet {
    pub fn new(mass: f64, trajectory: Vec<TrajectoryPoint>, parent: Option<Entity>) -> Self {
        Self {
            mass: mass,
            trajectory: trajectory,
            parent: parent 
        }
    }

    pub fn relative_mass(m1 : f64, m2: f64) -> f64 {
        (m1 * m2) / (m1 + m2)
    }

    pub fn calculate_trajectory(translation: Vec<f64>, velocity: Vec<f64>, mu: f64, step_size: f64, times: usize) -> Vec<TrajectoryPoint> {
        fn f(st: &mut ode::State<f64>, env: &Vec<f64>) {
            let mu = env[0];
            let value = &st.value;
            let derive = &mut st.deriv;
            
            // current position
            let r1 = &value[0..3].to_vec();
            let r2 = &value[3..6].to_vec();
            // distance between bodies
            let r_norm = vec![r2[0] - r1[0], r2[1] - r1[1], r2[2] - r1[2]].norm();
          
            // current velocity
            let v1 = &value[6..9];
            let v2 = &value[9..12];

            // acceleration
            let ax = -r2[0] * mu / r_norm.powi(3);
            let ay = -r2[1] * mu / r_norm.powi(3);
            let az = -r2[2] * mu / r_norm.powi(3);

            // keep position of first body constant for now
            derive[0] = r1[0];
            derive[1] = r1[1];
            derive[2] = r1[2];
            derive[3] = v2[0];
            derive[4] = v2[1];
            derive[5] = v2[2];
            derive[6] = ax; 
            derive[7] = ay;
            derive[8] = az;
            derive[9] = ax;
            derive[10] = ay;
            derive[11] = az;
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
            .set_env(vec![mu])
            .integrate();
        let duration = start.elapsed();
        println!("{result}");
        println!("Time elapsed integrating: {duration:?}");

        let mut points: Vec<TrajectoryPoint> = vec![];
        for n in (0..result.row).rev() {
            let row = result.row(n);
            points.push(
                TrajectoryPoint { 
                    position: row[4..7].to_vec(),
                    velocity: row[10..13].to_vec() 
                });
        }
        return points;
    }
}
