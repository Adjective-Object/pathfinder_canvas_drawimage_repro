use gl;

use pathfinder_canvas::FillStyle;
use pathfinder_color::rgbu;
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::vec2f;

use pathfinder_geometry::vector::vec2i;
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::DestFramebuffer;
use pathfinder_renderer::gpu::options::{RendererMode, RendererOptions};

use image::io::Reader;
use pathfinder_content::pattern::{Image, Pattern};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;

use pathfinder_resources::embedded::EmbeddedResourceLoader;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

fn load_pathfinder_img(img_path: &str) -> Image {
    let reader = Reader::open(img_path).unwrap();
    if reader.format() == None {
        panic!("Failed to parse image \"{}\"", img_path);
    }

    let decoded_image = reader.decode().unwrap();
    return Image::from_image_buffer(decoded_image.to_rgba8());
}

fn main() {
    let canvas_size = vec2i(500, 500);
    let canvas = pathfinder_canvas::Canvas::new(pathfinder_canvas::Vector2F::new(
        canvas_size.x() as f32,
        canvas_size.y() as f32,
    ));
    let mut context2d =
        canvas.get_context_2d(pathfinder_canvas::CanvasFontContext::from_system_source());

    // load the mushroom image
    let mushroom_img = load_pathfinder_img("./resources/1280px-Vliegenzwam_(Amanita_muscaria)._Locatie_De_Famberhorst._27-09-2020_(d.j.b.).jpg");

    context2d.set_fill_style(FillStyle::Color(rgbu(255, 255, 255)));
    context2d.fill_rect(RectF::new(
        vec2f(0.0, 0.0),
        vec2f(canvas_size.x() as f32, canvas_size.y() as f32),
    ));

    context2d.draw_subimage(
        Pattern::from_image(mushroom_img.clone()),
        RectF::new(vec2f(0.0, 0.0), vec2f(100.0, 100.0)),
        RectF::new(vec2f(10.0, 300.0), vec2f(70.0, 80.0)),
    );

    context2d.draw_subimage(
        Pattern::from_image(mushroom_img.clone()),
        RectF::new(vec2f(300.0, 300.0), vec2f(100.0, 100.0)),
        RectF::new(vec2f(10.0, 20.0), vec2f(70.0, 80.0)),
    );

    context2d.draw_subimage(
        Pattern::from_image(mushroom_img.clone()),
        RectF::new(vec2f(300.0, 300.0), vec2f(150.0, 150.0)),
        RectF::new(vec2f(120.0, 20.0), vec2f(70.0, 80.0)),
    );

    context2d.draw_subimage(
        Pattern::from_image(mushroom_img.clone()),
        RectF::new(vec2f(300.0, 400.0), vec2f(100.0, 100.0)),
        RectF::new(vec2f(10.0, 120.0), vec2f(70.0, 80.0)),
    );

    context2d.draw_subimage(
        Pattern::from_image(mushroom_img.clone()),
        RectF::new(vec2f(300.0, 400.0), vec2f(150.0, 150.0)),
        RectF::new(vec2f(120.0, 120.0), vec2f(70.0, 80.0)),
    );

    context2d.draw_image(
        Pattern::from_image(mushroom_img.clone()),
        RectF::new(vec2f(300.0, 20.0), vec2f(70.0, 80.0)),
    );

    // Create the GL context, and make it current.
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window(
            "draw_subimage repro",
            canvas_size.x() as u32,
            canvas_size.y() as u32,
        )
        // .hidden()
        .opengl()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
    window.gl_make_current(&gl_context).unwrap();

    // output the drawn image to disk
    let resource_loader = EmbeddedResourceLoader::new();
    let pathfinder_device = GLDevice::new(GLVersion::GL3, 0);
    let mode = RendererMode::default_for_device(&pathfinder_device);
    let options = RendererOptions {
        background_color: Some(ColorF::transparent_black()),
        dest: DestFramebuffer::full_window(canvas_size),
        ..RendererOptions::default()
    };
    let mut renderer = Renderer::new(pathfinder_device, &resource_loader, mode, options);

    let mut scene_proxy = SceneProxy::from_scene(
        context2d.into_canvas().into_scene(),
        renderer.mode().level,
        RayonExecutor,
    );

    // Wait for a keypress.
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        match event_pump.wait_event() {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return,
            Event::Window {
                win_event: WindowEvent::Exposed,
                ..
            } => {
                // Render the canvas to screen.
                scene_proxy.build_and_render(&mut renderer, BuildOptions::default());
                window.gl_swap_window();
            }
            _ => {}
        }
    }
}
