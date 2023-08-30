use bevy::prelude::*;
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy_mod_picking::prelude::*;
use bevy_xpbd_3d::components::{Collider, Position};
use bevy_xpbd_3d::prelude::{CoefficientCombine, Friction, GravityScale, LockedAxes, Restitution, RigidBody};
use oxidized_navigation::NavMeshAffector;
use crate::movement::MovementPath;
use crate::pathfinding::MoveEvent;

#[derive(Component)]
pub struct Selected;

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
        RigidBody::Static,
        Collider::cuboid(75.0, 0.5, 75.0),
        PickableBundle::default(),
        NavMeshAffector,
        On::<Pointer<Down>>::send_event::<MoveEvent>(),
    ));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 1.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.1, 0.1, 0.5).into()),
            transform: Transform::from_xyz(-5.0, 0.8, -5.0),
            ..default()
        },
        Collider::capsule(1., 0.5),
        RigidBody::Dynamic,
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        Friction::new(0.),
        GravityScale(2.0),
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        PickableBundle::default(),
        Selected,
        MovementPath::default()
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
        RigidBody::Static,
        Collider::cuboid(5.0, 1.5, 0.1),
        NavMeshAffector,
    ));
}
