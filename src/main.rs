mod camera;
mod render;

use std::{cell::OnceCell, sync::Arc, time::Instant};

use camera::Camera;
use render::Renderer;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: OnceCell<Arc<Window>>,
    renderer: OnceCell<Renderer>,
    camera_smoothed: Camera,
    camera: Camera,
    last_render_time: Option<Instant>,
    dragging: bool,
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

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer.take();
        self.window.take();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.renderer.get_mut().unwrap().resize(size);
                self.window.get().unwrap().request_redraw();
            }

            WindowEvent::RedrawRequested => {
                let dt = match self.last_render_time {
                    None => 0.0,
                    Some(t) => (Instant::now() - t).as_secs_f32(),
                };
                self.last_render_time = Some(Instant::now());
                self.camera_smoothed.lerp_exp(&self.camera, 0.9, dt);

                let renderer = self.renderer.get_mut().unwrap();
                renderer.render(self.camera_smoothed.matrix());
                self.window.get().unwrap().request_redraw();
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.dragging = state == ElementState::Pressed;
            }

            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                self.camera.yaw += 0.01 * delta.x as f32;
                self.camera.pitch += 0.01 * delta.y as f32;
            }

            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, delta),
                ..
            } => {
                self.camera.radius /= 1.0 + 0.2 * delta as f32;
            }

            WindowEvent::PinchGesture { delta, .. } => {
                self.camera.radius /= 1.0 + delta as f32;
            }

            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta: (x, y) } if self.dragging => {
                self.camera.yaw += 0.01 * x as f32;
                self.camera.pitch += 0.01 * y as f32;
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
