use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}, render::camera::Projection};
use bevy_inspector_egui::WorldInspectorPlugin;

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

fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
){
    let orbit_button = MouseButton::Right;

    let mut rotation_move = Vec2::ZERO;
    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if rotation_move.length_squared() > 0.0 {
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * PI * 2.0;
                delta
            };
            let delta_y = rotation_move.y / window.y * PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = pitch * transform.rotation;
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width(), window.height());
    window
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 1.0, subdivisions: 6})),
        material: materials.add(Color::rgb(0.7,0.3,0.0).into()),
        ..default()
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

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(PanOrbitCamera::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_scene)
        .add_system(pan_orbit_camera)
        .run();
}
