use bevy::prelude::*;
use bevy::window::*;

pub fn window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            cursor: Default::default(),
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::Windowed,
            position: WindowPosition::Centered(MonitorSelection::Index(0)),
            resolution: WindowResolution::new(1280., 720.),
            title: "ccg".into(),
            composite_alpha_mode: Default::default(),
            resize_constraints: WindowResizeConstraints::default(),
            resizable: true,
            decorations: true,
            transparent: true,
            focused: false,
            window_level: WindowLevel::Normal,
            canvas: None,
            fit_canvas_to_parent: false,
            prevent_default_event_handling: false,
            internal: Default::default(),
            ime_enabled: false,
            ime_position: Default::default(),
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
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

fn f_toggle_fullscreen(
    input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
    if input.just_pressed(KeyCode::F) {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            _ => WindowMode::Windowed,
        }
    }
}
