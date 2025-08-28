#![deny(clippy::all)]
#![forbid(unsafe_code)]
use Zero::modules::Renderer;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use Zero::modules::math::Vec2;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// Representation of the application state.
struct Scene {
    // Add your state here
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Zero Physics")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = Scene::new();

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            // Get the frame buffer for this render cycle
            let frame: &mut [u8] = pixels.frame_mut();
            let mut renderer = Renderer::new(frame, WIDTH as usize, HEIGHT as usize);
            world.draw(&mut renderer);

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }
            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl Scene {
    /// Create a new Scene instance.
    fn new() -> Self {
        Self {
            // Initialize your state here
        }
    }
    /// Update the Scene internal state.
    fn update(&mut self) {

    }
    /// Draw the Scene state to the frame buffer.
    ///
    /// Assumes the default texture format: wgpu::TextureFormat::Rgba8UnormSrgb

    fn draw(&self, renderer: &mut Renderer) {
        let bg_color = [0x48, 0xb2, 0xe8, 0xff]; // light blue
        let fg_color = [0xb2, 0xb2, 0xb2, 0xff]; // gray

        // 1️⃣ clear background
        for pixel in renderer.frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&bg_color);
        }

        // 2️⃣ draw objects
        let center_x = (renderer.width / 2) as i32;
        let center_y = (renderer.height / 2) as i32;
        let pixel_pos = Vec2:: 
        for i in 1..=25{

        renderer.put_pixel(center_x + i, center_y, fg_color);
        }

    }
}
