use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct LeftBall;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 1.5, 5.0)));

    commands.spawn((
        RigidBody::Dynamic,
        LockedAxes::ALL_LOCKED.unlock_rotation_z(),
        Collider::cuboid(6.0, 0.25, 1.0),
        Mesh3d(meshes.add(Cuboid::new(6.0, 0.25, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.9, 0.5),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.125, 0.0),
    ));

    commands.spawn((
        LeftBall,
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.5, 0.5),
            ..default()
        })),
        // Separate issue: if the GraviyScale is added here, the difference is ColliderDensity
        // has no effect
        //GravityScale(1.0),
        ColliderDensity(1.0),
        Transform::from_xyz(-2.5, 1.75, 0.0),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.5, 0.5),
            ..default()
        })),
        ColliderDensity(1.0),
        Transform::from_xyz(2.5, 1.75, 0.0),
    ));

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

fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    single: Single<(&mut ColliderDensity, &MeshMaterial3d<StandardMaterial>), With<LeftBall>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (mut collider_density, material_3d) = single.into_inner();
    if keyboard.just_pressed(KeyCode::Space) {
        // If we change the ColliderDensity here, the seasaw should tip the the left
        collider_density.0 = 5.0;
        if let Some(mut m) = materials.get_mut(material_3d) {
            m.base_color = Color::srgb(0.5, 0.5, 0.9);
        }
    }
}
