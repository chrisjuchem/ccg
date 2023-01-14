pub mod camera_controller;

// fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
//     for mut text in &mut query {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(average) = fps.average() {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{:.2}", average);
//             }
//         }
//     }
// }
