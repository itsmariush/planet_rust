use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Component)]
struct Movable {
    speed: f32,
}

fn update_movables(
    mut query: Query<&mut Transform, With<Movable>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::W) {
        for mut transform in query.iter_mut() {
            transform.translation.x += 1.0;
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 1.0, subdivisions: 6})),
        material: materials.add(Color::rgb(1.0,0.0,0.0).into()),
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
    }).insert(Movable {
        speed: 1.0,
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_scene)
        .add_system(update_movables)
        .run();
}
