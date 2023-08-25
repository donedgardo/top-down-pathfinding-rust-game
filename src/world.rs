use bevy::prelude::*;
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy_mod_picking::prelude::*;
use bevy_xpbd_3d::components::Collider;
use oxidized_navigation::NavMeshAffector;

pub fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, 0.0)
        ),
        ..default()
    });

    // Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane {
                size: 75.0,
                subdivisions: 0,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::IDENTITY,
            ..default()
        },
        Collider::cuboid(50.0, 0.2, 50.0),
        PickableBundle::default(),
        NavMeshAffector,
    ));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
            material: materials.add(Color::rgb(0.1, 0.1, 0.5).into()),
            transform: Transform::from_xyz(-5.0, 0.8, -5.0),
            ..default()
        },
        Collider::cuboid(2.5, 2.5, 2.5),
        PickableBundle::default(),
        NavMeshAffector,
    ));

    // Thin wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.1, 0.1, 0.5).into()),
            transform: Transform::from_xyz(-3.0, 0.8, 5.0).with_scale(Vec3::new(50.0, 15.0, 1.0)),
            ..default()
        },
        // At the time of writing, xpbd (v0.2) colliders don't support scaling, so you have to create the collider with the post-scaled size.
        Collider::cuboid(5.0, 1.5, 0.1),
        NavMeshAffector,
    ));
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        PickableBundle::default(), // Adds selection, highlighting, and the `Pickable` override.
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        PickableBundle::default(), // Adds selection, highlighting, and the `Pickable` override.
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -4.0),
        ..default()
    });
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}