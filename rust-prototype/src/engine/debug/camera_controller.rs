use bevy::ecs::world::EntityMut;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::*;

pub fn add(app: &mut App) {
    println!("a");

    app.init_resource::<CameraTracker>()
        .add_system(add_camera_controller)
        .add_system(camera_controller)
        .add_system(camera_tracker);
}

#[derive(Component)]
struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.3,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            mouse_key_enable_mouse: MouseButton::Left,
            keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 5.0,
            run_speed: 15.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

fn add_camera_controller(
    mut commands: Commands,
    q: Query<Entity, (With<Camera>, Without<CameraController>)>,
) {
    for e in q.iter() {
        commands
            .get_entity(e)
            .unwrap()
            .insert(CameraController::default());
    }
}

fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut move_toggled: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }
        if key_input.just_pressed(options.keyboard_key_enable_mouse) {
            *move_toggled = !*move_toggled;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if mouse_button_input.pressed(options.mouse_key_enable_mouse) || *move_toggled {
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
                .clamp(-PI / 2., PI / 2.);
            options.yaw -= mouse_delta.x * options.sensitivity * dt;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
        }
    }
}

#[derive(Resource, Default)]
struct CameraTracker {
    active_index: Option<usize>,
    cameras: Vec<Entity>,
}

impl CameraTracker {
    fn track_camera(&mut self, entity: Entity) -> bool {
        self.cameras.push(entity);
        if self.active_index.is_none() {
            self.active_index = Some(self.cameras.len() - 1);
            true
        } else {
            false
        }
    }

    fn active_camera(&self) -> Option<Entity> {
        self.active_index.map(|i| self.cameras[i])
    }

    fn set_next_active(&mut self) -> Option<Entity> {
        let active_index = self.active_index?;
        let new_i = (active_index + 1) % self.cameras.len();
        self.active_index = Some(new_i);
        Some(self.cameras[new_i])
    }
}

fn camera_tracker(
    mut camera_tracker: ResMut<CameraTracker>,
    keyboard_input: Res<Input<KeyCode>>,
    mut queries: ParamSet<(
        Query<(Entity, &mut Camera), (Added<Camera>, Without<CameraController>)>,
        Query<(Entity, &mut Camera), (Added<Camera>, With<CameraController>)>,
        Query<&mut Camera>,
    )>,
) {
    // track added scene camera entities first, to ensure they are preferred for the
    // default active camera
    for (entity, mut camera) in queries.p0().iter_mut() {
        camera.is_active = camera_tracker.track_camera(entity);
    }

    // iterate added custom camera entities second
    for (entity, mut camera) in queries.p1().iter_mut() {
        camera.is_active = camera_tracker.track_camera(entity);
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        // disable currently active camera
        if let Some(e) = camera_tracker.active_camera() {
            if let Ok(mut camera) = queries.p2().get_mut(e) {
                camera.is_active = false;
            }
        }

        // enable next active camera
        if let Some(e) = camera_tracker.set_next_active() {
            if let Ok(mut camera) = queries.p2().get_mut(e) {
                camera.is_active = true;
            }
        }
    }
}
