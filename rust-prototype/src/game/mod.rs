use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy_text_mesh::TextMeshPlugin;

mod grab;
use grab::{GrabPlugin, Grabbable};

mod card;
use card::CardPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.05, 0.0, 0.0)));
        app.add_startup_system(init_objects)
            .add_startup_system(init_camera);

        app.add_plugin(GrabPlugin);
        app.add_plugin(CardPlugin);

        app.add_plugin(TextMeshPlugin);
    }
}

fn init_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // table
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 19.0 })),
        material: materials.add(StandardMaterial {
            perceptual_roughness: 0.9,
            ..Color::rgb(0.3, 0.5, 0.3).into()
        }),
        transform: Transform::from_xyz(0., 0., 0.)
            .looking_at(Vec3::Y, Vec3::Z)
            .with_scale(Vec3::new(1.6 / 0.9, 1., 1.)),
        ..default()
    });

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
            left: -160. / 9.,
            right: 160. / 9.,
            bottom: 10.,
            top: -10.,
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
