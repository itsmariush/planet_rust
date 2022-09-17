use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier3d::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
extern crate peroxide;

mod components;
mod systems;

use components::{
    camera::PanOrbitCamera,
    physics::*
};
use systems::camera::pan_orbit_camera;

// Calculate Center of Mass of two bodies
fn calc_com(m1: f32, m2: f32, r1: Vec3, r2: Vec3) -> Vec3 {
    // Rcom = (m1*R1 + m2*R2) / (m1 + m2)
    let rx = (m1 * r1.x + m2 * r2.x) / (m1 + m2);
    let ry = (m1 * r1.y + m2 * r2.y) / (m1 + m2);
    let rz = (m1 * r1.z + m2 * r2.z) / (m1 + m2);
    Vec3::new(rx, ry, rz)
}


fn gravity_system(
    mut query: Query<(&mut Planet, &mut Transform)>,
) {
    for (mut planet, mut transform) in query.iter_mut() {
        match planet.trajectory.points.pop() {
            Some(t) => {
                transform.translation.x = t.position[0] as f32;
                transform.translation.y = t.position[1] as f32;
                transform.translation.z = t.position[2] as f32;
                if planet.trajectory.points.is_empty() {
                    planet.trajectory.points = Trajectory::calculate_trajectory(t.position, t.velocity, 0.01, 10);
                }
            },
            _ => {}
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    // Plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..default()
        })
        .insert(Transform::from_xyz(0.0, -2.0, 0.0));

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
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(ColliderMassProperties::Mass(100.0))
        .insert(ReadMassProperties {
            ..Default::default()
        })
        .insert(Name::new("Sun"))
        .insert(Sun);

    let r_mag = 15f64;
    let v_mag = (MU / r_mag).sqrt();
    let traj = Trajectory::calculate_trajectory(vec![r_mag, 0.0, 0.0], vec![0.0, 0.0, v_mag], 0.01, 37000);
    for p in (0..traj.len()).step_by(100) {
        let pos = &traj[p];
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.1,
                    subdivisions: 1,
                })),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                ..default()
            })
            .insert(Transform::from_xyz(pos.position[0] as f32, pos.position[1] as f32, pos.position[2] as f32));
    }
    // Earth
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 6,
            })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Transform::from_xyz(r_mag as f32, 0.0, 0.0))
        .insert(ReadMassProperties::default())
        .insert(Velocity::default())
        .insert(Name::new("Earth"))
        .insert(Planet {
            trajectory: Trajectory::new(traj) 
        });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-4.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PanOrbitCamera::default());
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_scene)
        .add_system(pan_orbit_camera)
        .add_system(gravity_system)
        .run();
}
