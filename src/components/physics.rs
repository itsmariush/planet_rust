use std::collections::HashMap;
use std::time::Instant;

use bevy::prelude::*;
use peroxide::prelude::*;
use peroxide::numerical::ode;
use peroxide::c;

#[derive(Debug)]
pub struct SimulationStep {
    pub time_elapsed: f64,
    pub time_per_step: f64,
    pub step: u64,
    pub step_size: u64
}

impl Default for SimulationStep {
    fn default() -> Self {
        Self {
            time_elapsed: 0f64,
            time_per_step: 0.02f64,
            step: 0,
            step_size: 1
        }
    }
}

#[derive(Debug, Default)]
pub struct TrajectoryPoint {
    pub time: f64,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
}

#[derive(Component, Default)]
pub struct Trajectory {
    pub points: HashMap<u64, TrajectoryPoint>,
    pub center: Option<Entity>,
    pub relative_mass: f64,
}

#[derive(Component)]
pub struct Sun;
#[derive(Component)]
pub struct Planet {
    pub mass: f64,
}

pub const M1: f64 = 333.0;
pub const M2: f64 = 1.0;
pub const MU: f64 = (M1*M2)/(M1+M2);

impl Environment for Trajectory {}
impl Trajectory {
    pub fn new(center: Option<Entity>, mu: f64) -> Self{
        Self {
            points: HashMap::new(),
            center: center,
            relative_mass: mu,
        }
    }

    pub fn calculate(&mut self, translation: Vec<f64>, velocity: Vec<f64>, mu: f64, step_size: f64, times: usize) {
        fn f(st: &mut ode::State<f64>, env: &Vec<f64>) {
            println!("Param: {}", st.param);
            let mu = env[0];
            let value = &st.value;
            let derive = &mut st.deriv;
            // current position
            let r1 = &value[0..3].to_vec();
            let r2 = &value[3..6].to_vec();
            // distance between bodies
            let r12 = vec![r2[0] - r1[0], r2[1] - r1[1], r2[2] - r1[2]];
            let r_norm = r12.norm();
          
            // current velocity
            let v1 = &value[6..9];
            let v2 = &value[9..12];

            // acceleration
            let ax = -r12[0] * mu / r_norm.powi(3);
            let ay = -r12[1] * mu / r_norm.powi(3);
            let az = -r12[2] * mu / r_norm.powi(3);

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

        for n in (0..result.row).rev() {
            let row = result.row(n);
            self.points.insert(n as u64, 
                TrajectoryPoint { 
                    time: row[0],
                    position: row[4..7].to_vec(),
                    velocity: row[10..13].to_vec() 
                });
        }
    }
}

impl Planet {
    pub fn new(mass: f64) -> Self {
        Self {
            mass: mass,
        }
    }

    pub fn relative_mass(&self, other: &Planet) -> f64 {
        let m1 = self.mass;
        let m2 = other.mass;
        (m1 * m2) / (m1 + m2)
    }

}
