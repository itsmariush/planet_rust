use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
extern crate peroxide;

mod components;
mod systems;

use components::{
    camera::PanOrbitCamera,
    physics::*
};
use systems::{
    camera::pan_orbit_camera,
    physics::*
};

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Sun
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 6,
            })),
            material: materials.add(Color::rgb(0.990, 0.945, 0.455).into()),
            ..default()
        })
        .insert(Name::new("Sun"))
        .insert(Sun);

    let trajectory_length = 56_280;
    let earth_mass = 100f64;
    let r_mag = 20.0f64;
    let v_mag = (MU / r_mag).sqrt();
    let mut traj_earth = Trajectory::new(None, MU);
    let vec_r_e = vec![r_mag, 0.0, 0.0];
    let vec_v_e = vec![0.0, 0.0, v_mag];
    traj_earth.calculate(&TrajectoryPoint::new(0.0, vec_r_e.clone(), vec_v_e.clone()), None, trajectory_length);
    let earth = Planet::new(earth_mass);
    let moon_mass = 0.0149f64;
    let moon = Planet::new(moon_mass);
    let moon_mu = Planet::relative_mass(moon_mass, earth_mass);
    let moon_environment = DeriveEnv {
        points: traj_earth.points.clone(),
        relative_mass: moon_mu,
        current_step: 0
    };
    // Earth
    let earth_entity = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.4,
                subdivisions: 6,
            })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            ..default()
        })
        .insert(Transform::from_xyz(r_mag as f32, 0.0, 0.0))
        .insert(Name::new("Earth"))
        .insert(earth)
        .insert(traj_earth)
        .id();
    // Moon
    let moon_relative_mag = 1.5;
    let r_mag_moon = r_mag + moon_relative_mag;
    let v_mag_moon = (moon_mu / moon_relative_mag).sqrt();
    let mut traj_moon = Trajectory::new(Some(earth_entity), moon_mu);
    let vec_r_m = vec![r_mag_moon, 0.0, 0.0];
    let vec_v_m = vec![0.0, 0.0, v_mag_moon];
    traj_moon.calculate(&TrajectoryPoint::new(0.0, vec_r_m.clone(), vec_v_m.clone()), Some(moon_environment), 1000);
    calc_eccentricity(&vec_r_e, &vec_v_e, &vec_r_m, &vec_v_m, moon_mass, earth_mass);
    calc_eccentricity(&vec_r_e, &vec_v_e, &vec![0.0, 0.0, 0.0], &vec![0.0, 0.0, 0.0], 1.0, M1);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.2,
                subdivisions: 6,
            })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            ..default()
        })
        .insert(Transform::from_xyz(r_mag_moon as f32, 0.0, 0.0))
        .insert(Name::new("Moon"))
        .insert(moon)
        .insert(traj_moon);

    commands.spawn_bundle( PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            range: 1000.0,
            radius: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        })
        .insert(PanOrbitCamera::default());
}

// https://astronomy.stackexchange.com/questions/29005/calculation-of-eccentricity-of-orbit-from-velocity-and-radius
fn calc_eccentricity(r1: &Vec<f64>, r2: &Vec<f64>, v1: &Vec<f64>, v2: &Vec<f64>, m1: f64, m2: f64) {

    let r1 = Vec3::new(r1[0] as f32, r1[1] as f32, r1[2] as f32);
    let v1 = Vec3::new(v1[0] as f32, v1[1] as f32, v1[2] as f32);
    
    let r2 = Vec3::new(r2[0] as f32, r2[1] as f32, r2[2] as f32);
    let v2 = Vec3::new(v2[0] as f32, v2[1] as f32, v2[2] as f32);

    let r = r2 - r1;
    let v = v2 - v1;
    //let g = 6.674e-11;

    // e = ((v x (v x r)) / (m1 + m2)) - (r / |r|)
    let r_length = r.length();
    let rcrossv = r.cross(v);
    let rvcrossv = rcrossv.cross(v);
    let e = (rvcrossv / ((m1 + m2) as f32)) - (r / r_length);
    println!("{:?}", e.length());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .init_resource::<SimulationStep>()
        .register_inspectable::<Planet>()
        .add_startup_system(setup_scene)
        .add_system(simulation_system)
        .add_system(pan_orbit_camera)
        .add_system(trajectory_system)
        .add_system(transform_system)
        .add_system(debug_system)
        .run();
}
