use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.02;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_move)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 1.5, 5.0)));

    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(25.0, 1.0),
        Mesh3d(meshes.add(Cylinder::new(25.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::sphere(0.5),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(6.0, 0.25, 1.0),
        Mesh3d(meshes.add(Cuboid::new(6.0, 0.25, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.9, 0.5),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.125, 0.0),
    ));

    for x_offset in [-2.5, 2.5] {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::sphere(0.5),
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.5, 0.5),
                ..default()
            })),
            // Separate issue: if the GraviyScale is added here, the difference is ColliderDensity
            // has no effect
            //GravityScale(0.1),
            ColliderDensity(1.0 + x_offset * 0.05),
            Transform::from_xyz(x_offset, 1.75, 0.0),
        ));
    }

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            contact_shadows_enabled: true,
            intensity: 333_000.0,
            ..default()
        },
        Transform::from_xyz(-3.0, 6.0, -3.0),
    ));
}

fn mouse_move(
    motion: Res<AccumulatedMouseMotion>,
    mut transform: Single<&mut Transform, With<Camera3d>>,
) {
    let delta_yaw = -motion.delta.x * 0.003;
    let delta_pitch = -motion.delta.y * 0.002;
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}
