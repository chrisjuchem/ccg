mod actions;
mod card;
mod grab;
mod state;

use crate::game::actions::resolve_action;
use crate::game::state::Stack;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_mod_picking::prelude::{Drop, OnPointer, RaycastPickTarget};
use bevy_text_mesh::TextMeshPlugin;
use card::CardPlugin;
use grab::{battlefield_drop_handler, GrabPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.05, 0.0, 0.0)));
        app.add_startup_system(init_objects)
            .add_startup_system(init_camera);

        app.add_plugin(GrabPlugin);
        app.add_plugin(CardPlugin);

        app.add_plugin(TextMeshPlugin);

        app.insert_resource(Stack::default());
        app.add_system(resolve_action);
    }
}

fn init_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // table
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(19.0))),
        material: materials.add(StandardMaterial {
            perceptual_roughness: 0.9,
            ..Color::rgb(0.3, 0.5, 0.3).into()
        }),
        transform: Transform::from_xyz(0., 0., 0.)
            .looking_to(Vec3::Y, Vec3::Z)
            .with_scale(Vec3::new(1.6 / 0.9, 1., 1.)),
        ..default()
    });

    // battlefield
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(10.0))),
            material: materials.add(StandardMaterial {
                perceptual_roughness: 0.9,
                ..Color::rgb(0.8, 0.6, 0.3).into()
            }),
            transform: Transform::from_xyz(0., 0., 0.01)
                .looking_to(Vec3::Y, Vec3::Z)
                .with_scale(Vec3::new((1.6 / 0.9) * (19. / 10.), 1., 1.)),
            ..default()
        },
        OnPointer::<Drop>::run_callback(battlefield_drop_handler),
        RaycastPickTarget::default(),
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2500.0,
            // color: Default::default(),
            shadows_enabled: true,
            // shadow_projection: Default::default(),
            // shadow_depth_bias: 0.0,
            // shadow_normal_bias: 0.0,
            ..default()
        },
        transform: Transform::from_xyz(-0.8, 1., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        camera: Default::default(),
        // projection: Projection::Perspective(PerspectiveProjection::default()),
        projection: Projection::Orthographic(OrthographicProjection {
            area: Rect {
                min: Vec2 {
                    x: -160. / 9.,
                    y: -10.0,
                },
                max: Vec2 {
                    x: 160. / 9.,
                    y: 10.0,
                },
            },
            scaling_mode: ScalingMode::FixedVertical(20.),
            near: 0.,
            far: 20.1,
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 20.).looking_at(Vec3::ZERO, Vec3::Y),
        // camera_3d: Default::default(),
        tonemapping: Default::default(),
        ..default()
    });
}
