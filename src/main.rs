mod camera;
mod render;

use std::{cell::OnceCell, collections::HashSet, sync::Arc, time::Instant};

use camera::Camera;
use glam::Vec3;
use render::Renderer;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: OnceCell<Arc<Window>>,
    renderer: OnceCell<Renderer>,
    camera_smoothed: Camera,
    camera: Camera,
    last_render_time: Option<Instant>,
    dragging: Option<MouseButton>,
    pressed_keys: HashSet<KeyCode>,
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
                let now = Instant::now();
                match self.last_render_time {
                    None => {
                        self.camera_smoothed = self.camera;
                    }
                    Some(t) => {
                        let dt = (now - t).as_secs_f32();

                        let mut translation = Vec3::ZERO;

                        for (code, anti_code, delta_translation) in [
                            (KeyCode::KeyW, KeyCode::KeyS, Vec3::Z),
                            (KeyCode::KeyA, KeyCode::KeyD, Vec3::X),
                        ] {
                            let step = self.pressed_keys.contains(&code) as i32
                                - self.pressed_keys.contains(&anti_code) as i32;
                            let mut dt = self.camera.rotation().inverse()
                                * (step as f32 * delta_translation);
                            dt.y = 0.0;
                            translation += dt;
                        }

                        for (code, anti_code, delta_translation) in
                            [(KeyCode::KeyQ, KeyCode::KeyE, Vec3::Y)]
                        {
                            let step = self.pressed_keys.contains(&code) as i32
                                - self.pressed_keys.contains(&anti_code) as i32;
                            translation += step as f32 * delta_translation;
                        }

                        translation = 0.01 * translation.normalize_or_zero();

                        if self.pressed_keys.contains(&KeyCode::ShiftLeft)
                            || self.pressed_keys.contains(&KeyCode::ShiftRight)
                        {
                            translation *= 4.0;
                        }
                        if self.pressed_keys.contains(&KeyCode::AltLeft)
                            || self.pressed_keys.contains(&KeyCode::AltRight)
                        {
                            translation /= 4.0;
                        }

                        self.camera.translate(translation);

                        self.camera_smoothed.lerp_exp(&self.camera, dt);
                    }
                };
                self.last_render_time = Some(now);

                let renderer = self.renderer.get_mut().unwrap();
                renderer.render(self.camera_smoothed.matrix());
                self.window.get().unwrap().request_redraw();
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::MouseInput {
                button: MouseButton::Back,
                state: ElementState::Pressed,
                ..
            } => {
                self.camera.reset();
            }

            WindowEvent::MouseInput { button, state, .. } => {
                self.dragging = if state == ElementState::Pressed {
                    Some(button)
                } else {
                    None
                };
            }

            WindowEvent::Focused(false) => {
                self.dragging = None;
            }

            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                self.camera
                    .orbit(0.01 * delta.x as f32, 0.01 * delta.y as f32);
            }

            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, delta),
                ..
            } => {
                self.camera.zoom(-0.2 * delta as f32);
            }

            WindowEvent::PinchGesture { delta, .. } => {
                self.camera.zoom(delta as f32);
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        repeat: false,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        repeat: false,
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => {
                    self.pressed_keys.insert(code);
                }
                ElementState::Released => {
                    self.pressed_keys.remove(&code);
                }
            },

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
            DeviceEvent::MouseMotion { delta: (x, y) }
                if matches!(self.dragging, Some(MouseButton::Left)) =>
            {
                self.camera.orbit(0.01 * x as f32, 0.01 * y as f32);
            }

            DeviceEvent::MouseMotion { delta: (x, y) }
                if matches!(self.dragging, Some(MouseButton::Right)) =>
            {
                self.camera.pan(0.01 * x as f32, -0.01 * y as f32);
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
