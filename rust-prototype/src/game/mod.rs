use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};

mod grab;
use grab::{GrabPlugin, Grabbable};

mod card;
use card::base;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.05, 0.0, 0.0)));
        app.add_startup_system(init_objects)
            .add_startup_system(init_camera);
        // .add_system(grab)
        // .add_system(follow);

        app.add_plugin(GrabPlugin);
    }
}

// #[derive(Component)]
// struct Held {
//     offset: Vec3,
// }
//
// #[derive(Resource)]
// struct AlwaysGrab(Entity);

// fn follow(
//     mut commands: Commands,
//     windows: Res<Windows>,
//     camera: Query<(&Camera, &GlobalTransform)>,
//     mut held: Query<(&mut Transform, &Held, Entity)>,
//     mouse: Res<Input<MouseButton>>,
//     grab: Res<AlwaysGrab>,
// ) {
//     let (cam, cam_t) = camera.single();
//     let wnd = if let RenderTarget::Window(id) = cam.target {
//         windows.get(id).unwrap()
//     } else {
//         windows.get_primary().unwrap()
//     };
//
//     if let Some(cur_pos) = wnd.cursor_position() {
//         let wrld_pos = cam.viewport_to_world(cam_t, cur_pos).unwrap().origin;
//
//         for (mut t, h, e) in held.iter_mut() {
//             t.translation.x = wrld_pos.x + h.x;
//             t.translation.y = wrld_pos.y + h.y;
//
//             if mouse.just_released(MouseButton::Left) {
//                 commands.get_entity(e).unwrap().remove::<Held>();
//             }
//         }
//     }
// }

// fn grab(mut commands: Commands, q: Query<(&Transform, &Grab, Entity)>) {
//     for (t, g, e) in &q {
//         commands
//             .get_entity(e)
//             .unwrap()
//             .insert(Held {
//                 x: t.translation.x - g.x,
//                 y: t.translation.y - g.y,
//             })
//             .remove::<Grab>();
//     }
// }

fn init_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
    // cube
    let cube = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    perceptual_roughness: 0.7,
                    ..Color::rgb(0.8, 0.7, 0.6).into()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                ..default()
            },
            Grabbable::default(),
        ))
        .id();
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
        camera_3d: Default::default(),
        tonemapping: Default::default(),
        ..default()
    });
}
