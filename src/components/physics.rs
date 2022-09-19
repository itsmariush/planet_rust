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

const TIME_PER_STEP: f64 = 0.01;
impl Default for SimulationStep {
    fn default() -> Self {
        Self {
            time_elapsed: 0f64,
            time_per_step: TIME_PER_STEP,
            step: 0,
            step_size: 1
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TrajectoryPoint {
    pub time: f64,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
}

impl TrajectoryPoint {
    pub fn new(time: f64, position: Vec<f64>, velocity: Vec<f64>) -> Self {
        Self {
            time: time,
            position: position,
            velocity: velocity,
        }
    }
}

#[derive(Debug)]
pub struct DeriveEnv {
    pub relative_mass: f64,
    pub points: HashMap<u64, TrajectoryPoint>,
}
impl Environment for DeriveEnv{}
impl Default for DeriveEnv {
    fn default() -> Self {
        let mut p: HashMap<u64, TrajectoryPoint> = HashMap::new();
        p.insert(0, TrajectoryPoint { time: 0f64, position: vec![0.0, 0.0, 0.0], velocity: vec![0.0, 0.0, 0.0]});
        Self {
            points: p,
            relative_mass: MU,
        }
    }
}

#[derive(Component, Clone, Debug)]
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

impl Default for Trajectory {
    fn default() -> Self {
        let mut p: HashMap<u64, TrajectoryPoint> = HashMap::new();
        p.insert(0, TrajectoryPoint { time: 0f64, position: vec![0.0, 0.0, 0.0], velocity: vec![0.0, 0.0, 0.0]});
        Self {
            points: p,
            center: None,
            relative_mass: 0f64,
        }
    }
}
impl Environment for Trajectory {}
impl Trajectory {
    pub fn new(center: Option<Entity>, mu: f64) -> Self{
        Self {
            points: HashMap::new(),
            center: center,
            relative_mass: mu,
        }
    }

    pub fn calculate(&mut self, start_point: TrajectoryPoint, environment: Option<DeriveEnv>, times: usize) {
        let mut ode_test = ExplicitODE::new(f);
        let o_traj = environment.unwrap_or_default();
        println!("Start Point: {start_point:?}");
        let init_state: ode::State<f64> = ode::State::new(
            0.0,
            c![start_point.position; start_point.velocity],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            );
        let start = Instant::now();
        let result = ode_test
            .set_initial_condition(init_state)
            .set_method(ExMethod::RK4)
            .set_step_size(TIME_PER_STEP)
            .set_times(times)
            .set_env(o_traj)
            .integrate();
        let duration = start.elapsed();
        println!("{result}");
        println!("Time elapsed integrating: {duration:?}");

        for n in (0..result.row).rev() {
            let row = result.row(n);
            self.points.insert(n as u64, 
                TrajectoryPoint { 
                    time: row[0],
                    position: row[1..4].to_vec(),
                    velocity: row[4..7].to_vec() 
                });
        }

        fn f(st: &mut ode::State<f64>, env: &DeriveEnv) {
            let mu = env.relative_mass;
            let parent_traj = &env.points;
            let value = &st.value;
            let derive = &mut st.deriv;

            let def = &TrajectoryPoint { time: 0.0, position: vec![0.0, 0.0, 0.0], velocity: vec![0.0, 0.0, 0.0] };
            let r1 = &parent_traj.get(&((st.param*100.0).ceil() as u64)).unwrap_or(def).position;
            // current position
            let r2 = &value[0..3].to_vec();

            // vector from body1 to body2
            let r12 = vec![r2[0] - r1[0], r2[1] - r1[1], r2[2] - r1[2]];
            let r_norm = r12.norm();
          
            // current velocity
            let v2 = &value[3..6];

            // acceleration
            let ax2 = -r12[0] * mu / r_norm.powi(3);
            let ay2 = -r12[1] * mu / r_norm.powi(3);
            let az2 = -r12[2] * mu / r_norm.powi(3);

            // keep position of first body constant for now
            derive[0] = v2[0];
            derive[1] = v2[1];
            derive[2] = v2[2];
            derive[3] = ax2;
            derive[4] = ay2;
            derive[5] = az2;
        }
    }
}

