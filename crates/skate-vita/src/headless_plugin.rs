//! Verbindet Bevy's Headless-Rendering mit dem /dev/fb0-Writer.
//! Nutzt Bevy's offizielles ImageCopyPlugin (headless_renderer-Beispiel).

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::render::batching::NoIndirectDrawing;

use crate::fb0_writer::Fb0Writer;

pub struct HeadlessFbPlugin;

impl Plugin for HeadlessFbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_offscreen);
        app.add_systems(Update, copy_to_fb0);
        app.insert_resource(FbState::default());
    }
}

#[derive(Resource, Default)]
struct FbState {
    writer: Option<Fb0Writer>,
    image_handle: Option<Handle<Image>>,
}

fn setup_offscreen(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<FbState>,
) {
    let width = 448u32;
    let height = 336u32;
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let handle = images.add(image);

    commands.spawn((
        Camera3d::default(),
        Camera {
            target: RenderTarget::Image(handle.clone()),
            ..default()
        },
        NoIndirectDrawing,
        RenderLayers::default(),
    ));

    state.image_handle = Some(handle);
    state.writer = Fb0Writer::open().ok();
    if let Some(w) = &state.writer {
        info!(
            "fb0: {}x{}, {} bpp",
            w.width(), w.height(), w.bits_per_pixel()
        );
    } else {
        warn!("fb0 konnte nicht geöffnet werden; Ausgabe nur über Log");
    }
}

fn copy_to_fb0(
    images: Res<Assets<Image>>,
    mut state: ResMut<FbState>,
) {
    let (Some(handle), Some(writer)) = (state.image_handle.as_ref(), state.writer.as_mut()) else {
        return;
    };
    let Some(image) = images.get(handle) else { return; };
    let Some(data) = image.data.as_ref() else { return; };
    if let Err(e) = writer.write_rgba8(data, image.width(), image.height()) {
        warn!("fb0 write: {e}");
    }
          }
