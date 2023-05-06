use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_mod_picking::pointer::Location;
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct RootEntity(Entity);

#[derive(Bundle)]
pub struct Grabbable {
    pickable: PickableBundle,       // <- Makes the mesh pickable.
    raycastable: RaycastPickTarget, // <- Needed for the raycast backend.
}
impl Grabbable {
    pub fn new(root: Entity) -> Self {
        Self {
            pickable: PickableBundle::default(),
            raycastable: RaycastPickTarget::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Draggable {
    drag_start: OnPointer<DragStart>,
    drag: OnPointer<Drag>,
    drag_end: OnPointer<DragEnd>,
    // drop: OnPointer<Drop>,
}

impl Draggable {
    pub fn new() -> Self {
        Self {
            drag_start: OnPointer::<DragStart>::target_remove::<Pickable>(),
            drag: OnPointer::<Drag>::run_callback(
                |In(drag): In<ListenedEvent<Drag>>,
                 pointer_utils: PointerUtils,
                 mut transforms: Query<&mut Transform>| {
                    let mut transform = transforms.get_mut(drag.listener).unwrap();
                    transform.translation += drag.delta.extend(0.0)
                        / pointer_utils.scale_movement(drag.pointer_location); //  follow the mouse
                    Bubble::Up
                },
            ),
            drag_end: OnPointer::<DragEnd>::target_insert(Pickable),
            // drop: drop_handler,
        }
    }
}

pub struct GrabPlugin;
impl Plugin for GrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_startup_system(setup_grab.in_base_set(StartupSet::PostStartup));
        // .add_system(drag);
    }
}

fn setup_grab(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    // add raycast source to the camera
    commands
        .get_entity(camera.single())
        .unwrap()
        .insert(RaycastPickCamera::default());
}

// // modified from https://github.com/aevyrie/bevy_mod_picking/blob/beta/examples/drag_and_drop.rs
// #[allow(clippy::too_many_arguments)]
// fn drag(
//     mut commands: Commands,
//     // Pointer Events
//     mut drag_start_events: EventReader<PointerDragStart>,
//     mut drag_events: EventReader<PointerDrag>,
//     mut drag_end_events: EventReader<PointerDragEnd>,
//     // Inputs
//     pointers: Res<PointerMap>,
//     windows: Res<Windows>,
//     images: Res<Assets<Image>>,
//     locations: Query<&PointerLocation>,
//     // Outputs
//     mut objects: Query<(Entity, &mut Transform)>,
//     grabbables: Query<(Entity, &RootEntity)>,
// ) {
//     // When we start dragging a square, we need to change the focus policy so that picking passes
//     // through it. Because the square will be locked to the cursor, it will block the pointer and we
//     // won't be able to tell what we are dropping it onto unless we do this.
//     for drag_start in drag_start_events.iter() {
//         let (entity, _) = grabbables.get(drag_start.target()).unwrap();
//         commands.entity(entity).remove::<RaycastPickTarget>();
//     }
//
//     // While being dragged, update the position of the square to be under the pointer.
//     for dragging in drag_events.iter() {
//         let pointer_entity = pointers.get_entity(dragging.pointer_id()).unwrap();
//         let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
//         let pointer_position = pointer_location.position;
//         let target = pointer_location
//             .target
//             .get_render_target_info(&windows, &images)
//             .unwrap();
//         let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;
//
//         let (_, root) = grabbables.get(dragging.target()).unwrap();
//         let (_, mut obj_transform) = objects.get_mut(root.0).unwrap();
//         let z = obj_transform.translation.z;
//
//         let cood_fudge = target_size.y / 20.; // TODO: not magic number world FixedVertical height
//         obj_transform.translation =
//             ((pointer_position - (target_size / 2.0)) / cood_fudge).extend(z);
//     }
//
//     // Add focus back
//     for drag_end in drag_end_events.iter() {
//         let (entity, _) = grabbables.get(drag_end.target()).unwrap();
//         commands.entity(entity).insert(RaycastPickTarget::default());
//     }
// }

#[derive(SystemParam)]
struct PointerUtils<'w, 's> {
    windows: Query<'w, 's, (Entity, &'static Window)>,
    images: Res<'w, Assets<Image>>,
}

impl PointerUtils<'_, '_> {
    fn scale_movement(&self, pointer_location: Location) -> f32 {
        let target = pointer_location
            .target
            .get_render_target_info(&self.windows, &self.images)
            .unwrap();
        let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;

        target_size.y / 20. // TODO: not magic number world FixedVertical height
    }
}
