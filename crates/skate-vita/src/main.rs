use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy::render::RenderApp;
use bevy::render::batching::gpu_preprocessing::{
    GpuPreprocessingMode, GpuPreprocessingSupport,
};
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuSettings, Backends};

mod fb0_writer;
mod headless_plugin;

use headless_plugin::HeadlessFbPlugin;

fn main() {
    // R36S-Patch: GL-Backend und WebGL2-Fallback erzwingen.
    // Wird zusätzlich über .cargo/config.toml [env] gesetzt, aber hier
    // explizit, falls die Config nicht greift.
    std::env::set_var("WGPU_BACKEND", "gl");
    std::env::set_var("WGPU_SETTINGS_PRIO", "webgl2");

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None, // Headless: kein Fenster
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                }),
                ..default()
            })
            .disable::<bevy::winit::WinitPlugin>() // Winit komplett aus
    )
    .add_plugins(ScheduleRunnerPlugin::run_loop(
        std::time::Duration::from_secs_f64(1.0 / 30.0), // 30 fps Cap
    ))
    .add_plugins(HeadlessFbPlugin);

    // R36S-Patch: GPU-Preprocessing deaktivieren, BEVOR der RenderApp startet.
    // Mali-G31 meldet max_compute_workgroup_size_x = 128, also würde Bevy
    // GPU-Preprocessing aktivieren und mit 4 Storage-Buffern crashen.
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.insert_resource(GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::None,
        });
    }

    // ... deine Skate-3-Systeme ...
    // app.add_systems(Startup, ...);
    // app.add_systems(Update, ...);

    app.run();
}
