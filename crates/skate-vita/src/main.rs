use bevy::prelude::*;
use bevy::app::{ScheduleRunnerPlugin, AppExit};
use bevy::render::RenderApp;
use bevy::render::batching::gpu_preprocessing::{
    GpuPreprocessingMode, GpuPreprocessingSupport,
};
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuSettings, Backends};
use bevy::window::ExitCondition;
use bevy::winit::WinitPlugin;

mod fb0_writer;
mod headless_plugin;

use headless_plugin::HeadlessFbPlugin;

fn main() {
    // R36S: GL-Backend + WebGL2-Fallback erzwingen
    std::env::set_var("WGPU_BACKEND", "gl");
    std::env::set_var("WGPU_SETTINGS_PRIO", "webgl2");

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,               // Headless: kein Fenster
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                })),
                ..default()
            })
            .disable::<WinitPlugin>()               // Winit komplett aus
    )
    .add_plugins(ScheduleRunnerPlugin::run_loop(
        std::time::Duration::from_secs_f64(1.0 / 30.0), // 30 fps Cap
    ))
    .add_plugins(HeadlessFbPlugin)
    .add_systems(Update, exit_on_esc);

    // R36S: GPU-Preprocessing deaktivieren, BEVOR der RenderApp startet.
    // Mali-G31 meldet max_compute_workgroup_size_x = 128, also wuerde
    // Bevy GPU-Preprocessing aktivieren und mit 4 Storage-Buffern crashen.
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.insert_resource(GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::None,
        });
    }

    // ... deine Skate-3-Systeme (Input, Simulation, UI) hier ...

    app.run();
}

fn exit_on_esc(mut exit: EventWriter<AppExit>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
