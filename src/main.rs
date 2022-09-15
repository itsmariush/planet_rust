use std::convert::TryInto;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::input::mouse::*;
use bevy::render::camera::Projection;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier3d::prelude::*;
extern crate peroxide;
use peroxide::fuga::*;
use peroxide::numerical::ode;

const _G: f32 = 1f32;//6.67259e-20;

#[derive(Component)]
struct PanOrbitCamera {
    focus: Vec3,
    radius: f32,
    upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera { 
            focus: Vec3::ZERO,
            radius: 5.0, 
            upside_down: false, 
        }
    }
}

#[derive(Component)]
struct Sun;
#[derive(Component)]
struct Planet;

fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
){
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut rotation_move = Vec2::ZERO;
    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;
    
    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    if input_mouse.just_pressed(orbit_button) || input_mouse.just_released(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {

        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = pitch * transform.rotation;
        } else if pan.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            let translation = (right+up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // zoom cant be ZERO
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width(), window.height());
    window
}

// https://towardsdatascience.com/use-python-to-create-two-body-orbits-a68aed78099c
fn gravity_system(mut query: Query<(&Name, &mut Velocity, &Transform, &ReadMassProperties), With<Planet>>) {
    let center = Vec3::ZERO;
    let sun_mass = 1000.0;
    let sun_grav_param = {
        let pow: i64 = 10_i64.pow(11);
        1.3271244 * pow as f64 // km^3/s^2
    };
    for (name, mut velocity, transform, read_mass) in query.iter_mut() {
        //println!("{}: {}kg", name, read_mass.0.mass);
        let distance = center.distance_squared(transform.translation);
    }
}

// Calculate Center of Mass of two bodies
fn calc_com(m1: f32, m2: f32, r1: Vec3, r2: Vec3) -> Vec3 {
    // Rcom = (m1*R1 + m2*R2) / (m1 + m2)
    let rx = (m1 * r1.x + m2 * r2.x) / (m1 + m2);
    let ry = (m1 * r1.y + m2 * r2.y) / (m1 + m2);
    let rz = (m1 * r1.z + m2 * r2.z) / (m1 + m2);
    Vec3::new(rx, ry, rz)
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>
) {
    rapier_config.gravity = Vec3::ZERO;

    // Plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {size: 100.0})),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..default()
        })
    .insert(Transform::from_xyz(0.0, -2.0, 0.0));

    // Sun
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 1.0, subdivisions: 6})),
        material: materials.add(Color::rgb(0.990, 0.945, 0.455).into()),
        ..default()})
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(ColliderMassProperties::Mass(100.0))
        .insert(ReadMassProperties {
            ..Default::default()
        })
        .insert(Name::new("Sun"))
        .insert(Sun);
    
    // Earth
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 1.0, subdivisions: 6})),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        ..default()})
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Transform::from_xyz(15.0, 0.0, 0.0))
        .insert(ReadMassProperties::default())
        .insert(Velocity::default())
        .insert(Name::new("Earth"))
        .insert(Planet);

    let com : Vec3 = calc_com(1000f32, 200f32, Vec3::ZERO, Vec3::new(4.0, 0.0, 4.0));
    println!("Center of Mass: {com}");

    // Center of Mass
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.5, subdivisions: 6})),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..default()})
        .insert(Transform::from_translation(com))
        .insert(ReadMassProperties::default())
        .insert(Velocity::default())
        .insert(Name::new("COM"))
        .insert(Planet);

    let mut ode_test = ExplicitODE::new(f);
    let init_state : ode::State<f64> = ode::State::new(
            0.0,
            vec![0.0, 0.0, 0.0, 15.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0, 0.0, 10.0, 0.0]
        );
    let result = ode_test
        .set_initial_condition(init_state)
        .set_method(ExMethod::RK4)
        .set_step_size(0.0001f64)
        .set_times(100)
        .integrate();

    println!("{result}");

    for n in 0..result.row {
        let row = result.row(n);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.2, subdivisions: 6})),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()})
            .insert(Transform::from_xyz(row[1] as f32, row[2] as f32, row[3] as f32))
            .insert(ReadMassProperties::default())
            .insert(Velocity::default())
            .insert(Name::new("Trajectory1"));
            
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.2, subdivisions: 6})),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            ..default()})
            .insert(Transform::from_xyz(row[4] as f32, row[5] as f32, row[6] as f32))
            .insert(ReadMassProperties::default())
            .insert(Velocity::default())
            .insert(Name::new("Trajectory2"));
    }

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(PanOrbitCamera::default());
}

fn f(st: &mut ode::State<f64>, _: &NoEnv) {
    let value = &mut st.value;
    let derive = &mut st.deriv; 
    
    let r1 = Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32);
    let r2 = Vec3::new(value[3] as f32, value[4] as f32, value[5] as f32);

    let r_mag = (r2 - r1).normalize();

    // X1 = G * m2 * ((X2-X1) / r^3)
    let m2 = 0.1f32;
    let m1 = 33.3f32;
    let r_mag3 = r_mag.powf(3.0);
    let nx1 = _G * m2 * ((r2.x - r1.x) / r_mag3.x);
    let ny1 = _G * m2 * ((r2.y - r1.y) / r_mag3.y);
    let nz1 = _G * m2 * ((r2.z - r1.z) / r_mag3.z);
 
    let nx2 = _G * m1 * ((r1.x - r2.x) / r_mag3.x);
    let ny2 = _G * m1 * ((r1.y - r2.y) / r_mag3.y);
    let nz2 = _G * m1 * ((r1.z - r2.z) / r_mag3.z);

    derive[0] = nx1 as f64;
    derive[1] = ny1 as f64;
    derive[2] = nz1 as f64;
    derive[3] = nx2 as f64;
    derive[4] = ny2 as f64;
    derive[5] = nz2 as f64;
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_scene)
        .add_system(pan_orbit_camera)
        .add_system(gravity_system)
        .run();
}
