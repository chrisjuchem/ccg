use bevy::prelude::*;
use bevy::window::*;

pub fn window_settings() -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            width: 1280.,
            height: 720.,
            position: WindowPosition::Centered,
            monitor: MonitorSelection::Index(0), // TODO MonitorSelection::Current/Primary
            resize_constraints: WindowResizeConstraints::default(),
            scale_factor_override: None,
            title: "ccg".into(),
            present_mode: PresentMode::AutoVsync,
            resizable: true,
            decorations: true,
            cursor_visible: true,
            cursor_grab_mode: CursorGrabMode::None,
            mode: WindowMode::Windowed,
            transparent: true,
            canvas: None,
            fit_canvas_to_parent: false,
            alpha_mode: CompositeAlphaMode::Auto,
        },
        add_primary_window: true,
        exit_on_all_closed: true,
        close_when_requested: true,
    }
}

pub struct WindowUtilPlugin;
impl Plugin for WindowUtilPlugin {
    fn build(&self, app: &mut App) {
        //     app.insert_resource(ClearColor(Color::rgb(0.55, 0.0, 0.0)));

        // app.add_startup_system(init_cameras);
        app.add_system(f_toggle_fullscreen);
    }
}

// fn init_cameras(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

fn f_toggle_fullscreen(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::F) {
        window.set_mode(match window.mode() {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            _ => WindowMode::Windowed,
        })
    }
}
