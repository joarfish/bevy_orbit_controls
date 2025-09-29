use bevy::prelude::*;
use bevy_orbit_controls::{OrbitCameraController, OrbitCameraControllerBundle, OrbitControlsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OrbitControlsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            ..PerspectiveProjection::default()
        }),
        OrbitCameraControllerBundle::from( OrbitCameraController::new(vec3(5.0, 5.0, 5.0), Vec3::ZERO))
    ));

    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));
}
