//! Shows an issue with frustum culling in bevy 0.10.0

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

const FAR: f32 = 10.00;
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cam_transform = Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn(Camera3dBundle {
        projection: PerspectiveProjection {
            far: FAR,
            ..default()
        }
        .into(),
        transform: cam_transform.clone(),
        ..default()
    });

    // calculate the far plane
    let cutoff_z = cam_transform
        .compute_matrix()
        .transform_point3(Vec3::new(0.0, 0.0, -FAR))
        .z;

    // cubes
    for i in 0..20 {
        let z = i as f32 + 0.5;
        if z > cutoff_z {
            // these should be culled by the frustum culling?
            println!("cube {} is behind the far cutoff {} at {}", i, cutoff_z, z);
        }
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hsl(i as f32 / 20.0 * 360.0, 1.0, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, z),
            ..default()
        });
    }

    // light
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        ..Default::default()
    });
}
