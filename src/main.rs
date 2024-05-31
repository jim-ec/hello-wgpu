mod camera;
mod render;

use std::{cell::OnceCell, sync::Arc};

use camera::Camera;
use render::Renderer;
use winit::{
    application::ApplicationHandler,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: OnceCell<Arc<Window>>,
    renderer: OnceCell<Renderer>,
    camera_smoothed: Camera,
    camera: Camera,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title(env!("CARGO_PKG_NAME")))
                .unwrap(),
        );
        self.window.set(window.clone()).unwrap();

        let renderer = Renderer::new(window);
        self.renderer
            .set(futures::executor::block_on(renderer))
            .unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.renderer.get_mut().unwrap().resize(size);
                self.window.get().unwrap().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let renderer = self.renderer.get_mut().unwrap();

                let inverse_smoothness = 0.5;
                self.camera_smoothed.yaw +=
                    inverse_smoothness * (self.camera.yaw - self.camera_smoothed.yaw);
                self.camera_smoothed.pitch +=
                    inverse_smoothness * (self.camera.pitch - self.camera_smoothed.pitch);
                self.camera_smoothed.radius +=
                    inverse_smoothness * (self.camera.radius - self.camera_smoothed.radius);

                renderer.render(self.camera_smoothed.matrix());

                self.window.get().unwrap().request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                self.camera.yaw += 0.01 * delta.x as f32;
                self.camera.pitch += 0.01 * delta.y as f32;
            }
            WindowEvent::PinchGesture { delta, .. } => {
                self.camera.radius /= 1.0 + delta as f32;
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
