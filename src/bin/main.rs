#![allow(unused_imports)]

use conrod_core::{text::rt::Rect, Scalar};
use conrod_example_shared as example;
use conrod_piston::{draw::primitives as draw_primitives, event::convert};
use example::DemoApp;
use pagepal::{Manga, Novel, fullscreen, library::Library, theme};
use piston_window::{
    texture::{Format::Rgba8, UpdateTexture},
    AdvancedWindow,
    Button,
    EventLoop,
    Flip,
    G2d,
    G2dTexture,
    Key,
    OpenGL,
    PistonWindow,
    PressEvent,
    ResizeEvent,
    Size,
    Texture,
    TextureSettings,
    UpdateEvent,
    Window,
    WindowSettings,
};
use sdl2_window::Sdl2Window;
use std::path::PathBuf;

// pub fn main() {
#[tokio::main]
pub async fn main() {
    let gl = OpenGL::V4_5;
    const WIDTH: u32 = example::WIN_W;
    const HEIGHT: u32 = example::WIN_H;
    let assets = PathBuf::from("assets");
    let font_path = assets.join("NotoSans-Regular.ttf");
    let _library: Library<Novel, Manga> = Library::default();

    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Pagepal conrod testing", [WIDTH, HEIGHT])
            .samples(16)
            .exit_on_esc(true)
            .vsync(true)
            .resizable(true)
            .graphics_api(gl)
            .build()
            .expect("You encountered this error");
    window.set_capture_cursor(false);
    window.set_max_fps(120);
    window.set_ups(60);

    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(theme())
        .build();
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut ctx = window.create_texture_context();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(WIDTH, HEIGHT)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let texture = G2dTexture::from_memory_alpha(
            &mut ctx, &init, WIDTH, HEIGHT, &settings,
        )
        .unwrap();
        (cache, texture)
    };

    // Instantiate the generated list of widget identifiers.
    let ids = example::Ids::new(ui.widget_id_generator());

    // Load the rust logo from file to a piston_window texture.
    let rust_logo: G2dTexture = {
        let path = assets.join("images/rust.png");
        let settings = TextureSettings::new();
        Texture::from_path(&mut ctx, &path, Flip::None, &settings).unwrap()
    };

    // Create our `conrod_core::image::Map` which describes each of our
    // widget->image mappings.
    let mut image_map = conrod_core::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);

    // A demonstration of some state that we'd like to control with the App.
    let mut app = DemoApp::new(rust_logo);

    while let Some(e) = window.next() {
        // Convert the src event to a conrod event.
        let size = window.size();
        let (win_w, win_h) = (size.width as Scalar, size.height as Scalar);
        if let Some(e) = convert(e.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        e.update(|_| {
            let mut ui = ui.set_widgets();
            example::gui(&mut ui, &ids, &mut app);
        });

        window.draw_2d(&e, |context, graphics, device| {
            if let Some(prim) = ui.draw_if_changed() {
                // A function used for caching glyphs to the texture cache.
                let cache_queued_glyphs =
                    |_graphics: &mut G2d,
                     cache: &mut G2dTexture,
                     rect: Rect<u32>,
                     data: &[u8]| {
                        let offset = [rect.min.x, rect.min.y];
                        let size = [rect.width(), rect.height()];
                        let format = Rgba8;
                        text_vertex_data.clear();
                        text_vertex_data.extend(
                            data.iter().flat_map(|&b| vec![255, 255, 255, b]),
                        );
                        UpdateTexture::update(
                            cache,
                            &mut ctx,
                            format,
                            &text_vertex_data[..],
                            offset,
                            size,
                        )
                        .expect("failed to update texture")
                    };

                // Draw the conrod `render::Primitives`.
                draw_primitives(
                    prim,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image,
                );

                ctx.encoder.flush(device);
            }
        });
        if let Some(button) = e.press_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::R => println!("{}", "This"),
                    Key::Q => break,
                    Key::F | Key::F12 => {
                        fullscreen(&mut window);
                        ui.needs_redraw()
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Specify how to get the drawable texture from the image.
/// In this case, the image *is* the texture.
fn texture_from_image<T>(img: &T) -> &T { img }
