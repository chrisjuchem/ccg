use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};

pub struct GrabPlugin;
impl Plugin for GrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<GrabRaycastSet>::default())
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_grab)
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(cache_cursor_pos.label(GrabLabel::Cursor))
                    .with_system(
                        set_ray_source
                            .after(GrabLabel::Cursor)
                            .before(RaycastSystem::BuildRays::<GrabRaycastSet>), // .with_run_criteria(clicked),
                    ),
            )
            .add_system(grab.label(GrabLabel::Pickup))
            .add_system(move_held.label(GrabLabel::Move).after(GrabLabel::Pickup))
            .add_system(drop.label(GrabLabel::Drop).after(GrabLabel::Move))
            .init_resource::<CursorPos>();

        // app.add_system(test);
    }
}

pub struct GrabRaycastSet;

pub type Grabbable = RaycastMesh<GrabRaycastSet>;
#[derive(Component)]
pub struct Grabbed {
    start: Vec3,
    cursor_start: Vec3,
}
type GrabRaycastSource = RaycastSource<GrabRaycastSet>;

#[derive(Resource, Default)]
struct CursorPos {
    pos: Option<Vec2>,
    world_pos: Option<Vec3>,
}

#[derive(SystemLabel)]
enum GrabLabel {
    Cursor,
    Pickup,
    Move,
    Drop,
}

fn setup_grab(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    // add raycast source to the camera
    commands
        .get_entity(camera.single())
        .unwrap()
        .insert(GrabRaycastSource::new());

    // commands.insert_resource(DefaultPluginState::<GrabRaycastSet>::default().with_debug_cursor());
}

fn cache_cursor_pos(
    windows: Res<Windows>,
    mut cursor: ResMut<CursorPos>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    // let window = if let RenderTarget::Window(id) = camera.target {
    //     windows.get(id).unwrap()
    // } else {
    //     windows.get_primary().unwrap()
    // };

    let cursor_pos = windows.get_primary().unwrap().cursor_position();
    cursor.pos = cursor_pos;
    cursor.world_pos = camera.get_single().ok().and_then(|(cam, trans)| {
        cursor_pos.and_then(|pos| cam.viewport_to_world(trans, pos).map(|ray| ray.origin))
    })
}

fn set_ray_source(
    mut source_obj: Query<&mut GrabRaycastSource>,
    cursor: Res<CursorPos>,
    click: Res<Input<MouseButton>>,
) {
    let mut source = match source_obj.get_single_mut() {
        Ok(src) => src,
        Err(_) => return,
    };

    if click.just_pressed(MouseButton::Left) && cursor.pos.is_some() {
        source.enabled = true;
        source.cast_method = RaycastMethod::Screenspace(cursor.pos.unwrap());
    } else {
        source.enabled = false;
    }
}

fn grab(
    ints: Query<(&Transform, Entity), (With<Intersection<GrabRaycastSet>>, Without<Grabbed>)>,
    mut commands: Commands,
    cursor: Res<CursorPos>,
) {
    for (t, e) in &ints {
        commands.entity(e).insert(Grabbed {
            start: t.translation,
            cursor_start: cursor.world_pos.unwrap(),
        });
    }
}

fn drop(
    input: Res<Input<MouseButton>>,
    held: Query<Entity, With<Grabbed>>,
    mut commands: Commands,
) {
    if input.just_released(MouseButton::Left) {
        for e in &held {
            commands.entity(e).remove::<Grabbed>();
        }
    }
}

fn move_held(mut held: Query<(&Grabbed, &mut Transform)>, cursor: Res<CursorPos>) {
    if let Some(pos) = cursor.world_pos {
        for (g, mut t) in &mut held {
            t.translation = g.start + pos - g.cursor_start;
            // println!("{} {} {}", t.translation, g.start, pos - g.cursor_start)
        }
    }
}
