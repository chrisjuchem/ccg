use crate::game::actions::{take_action, Action, ActionParams};
use crate::game::card::object::zone::{InZone, TargetTransform, Zone};
use crate::game::card::object::Card;
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

#[derive(Component, Clone, Debug)]
pub struct Hovered;
#[derive(Component, Clone, Debug)]
pub struct Dragged;

#[derive(Bundle)]
pub struct Draggable {
    drag_start: OnPointer<DragStart>,
    // drag_start2: OnPointer<DragStart>,
    drag: OnPointer<Drag>,
    drag_end: OnPointer<DragEnd>,
    // drop: OnPointer<Drop>,
    hover_start: OnPointer<Over>,
    hover_end: OnPointer<Out>,
}

impl Draggable {
    pub fn new() -> Self {
        Self {
            drag_start: OnPointer::commands_mut(|drag, commands| {
                commands.get_entity(drag.listener).unwrap().insert(Dragged);
                commands
                    .get_entity(drag.target)
                    .unwrap()
                    .remove::<Pickable>();
            }),

            // drag_start2: OnPointer::<DragStart>::target_remove::<Pickable>(),
            drag: OnPointer::run_callback(
                |In(drag): In<ListenedEvent<Drag>>,
                 pointer_utils: PointerUtils,
                 mut transforms: Query<&mut TargetTransform>| {
                    let mut transform = transforms.get_mut(drag.listener).unwrap();
                    transform.translation += drag.delta.extend(0.0)
                        / pointer_utils.scale_movement(drag.pointer_location); //  follow the mouse
                    transform.rotation = Quat::default();
                    Bubble::Up
                },
            ),
            drag_end: OnPointer::commands_mut(|drag, commands| {
                commands
                    .get_entity(drag.listener)
                    .unwrap()
                    .remove::<Dragged>();
                commands.get_entity(drag.target).unwrap().insert(Pickable);
            }),
            // drop: drop_handler,
            hover_start: OnPointer::listener_insert(Hovered),
            hover_end: OnPointer::listener_remove::<Hovered>(),
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

pub fn battlefield_drop_handler(
    In(event): In<ListenedEvent<Drop>>,
    parents: Query<&Parent>,
    action_params: ActionParams,
) -> Bubble {
    let card = parents.get(event.dropped).unwrap().get();
    take_action(Action::PlayCard(card), action_params);
    Bubble::Up
}

//=========

trait OnPointerExt<E: IsPointerEvent + Sync> {
    // should take `OnPointer`s, :'(
    fn run_callbacks<const N: usize, M, CB: IntoSystem<ListenedEvent<E>, Bubble, M> + Send + Sync>(
        callbacks: [CB; N],
    ) -> Self;

    fn listener_insert(bundle: impl Bundle + Clone) -> Self;
    fn listener_remove<B: Bundle>() -> Self;
}

impl<E: IsPointerEvent + Sync> OnPointerExt<E> for OnPointer<E> {
    fn run_callbacks<
        const N: usize,
        M,
        CB: IntoSystem<ListenedEvent<E>, Bubble, M> + Send + Sync,
    >(
        callbacks: [CB; N],
    ) -> Self {
        let mut cbs = callbacks
            .into_iter()
            .map(|cb| IntoSystem::into_system(cb))
            .collect::<Vec<_>>();
        Self::run_callback(move |In(event): In<ListenedEvent<E>>, world: &mut World| {
            let mut ret = Bubble::Up;
            for cb in cbs.iter_mut() {
                match cb.run(event.clone(), world) {
                    Bubble::Burst => ret = Bubble::Burst,
                    _ => (),
                }
            }
            ret
        })
    }

    fn listener_insert(bundle: impl Bundle + Clone) -> Self {
        Self::run_callback(
            move |In(event): In<ListenedEvent<E>>, mut commands: Commands| {
                let bundle = bundle.clone();
                commands.entity(event.listener).insert(bundle);
                Bubble::Up
            },
        )
    }

    /// Remove a bundle from the target entity any time this event listener is triggered.
    fn listener_remove<B: Bundle>() -> Self {
        Self::run_callback(
            move |In(event): In<ListenedEvent<E>>, mut commands: Commands| {
                commands.entity(event.listener).remove::<B>();
                Bubble::Up
            },
        )
    }
}
