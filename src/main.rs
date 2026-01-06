mod camera;
mod render;

use std::{collections::HashSet, sync::Arc, time::Instant};

use camera::Camera;
use futures::executor::block_on;
use glam::{Mat2, vec2, vec3};
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
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
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
        self.window = Some(window.clone());
        self.renderer = Some(block_on(Renderer::new(window)));
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.take();
        self.renderer.take();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer.take();
        self.window.take();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) else {
            return;
        };

        match event {
            WindowEvent::Resized(size) => {
                renderer.resize(size);
                window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                match self.last_render_time {
                    None => {
                        self.camera_smoothed = self.camera;
                    }
                    Some(t) => {
                        let dt = (now - t).as_secs_f32();

                        let input_axis = |pos: KeyCode, neg: KeyCode| {
                            (self.pressed_keys.contains(&pos) as i32
                                - self.pressed_keys.contains(&neg) as i32)
                                as f32
                        };

                        let mut factor = dt;
                        factor *= 6.0;
                        if self.pressed_keys.contains(&KeyCode::ShiftLeft)
                            || self.pressed_keys.contains(&KeyCode::ShiftRight)
                        {
                            factor *= 2.0;
                        }
                        if self.pressed_keys.contains(&KeyCode::AltLeft)
                            || self.pressed_keys.contains(&KeyCode::AltRight)
                        {
                            factor /= 2.0;
                        }

                        let ws = factor * input_axis(KeyCode::KeyD, KeyCode::KeyA);
                        let ad = factor * input_axis(KeyCode::KeyS, KeyCode::KeyW);
                        let wasd = factor * vec2(ws, ad).normalize_or_zero();
                        let zx = factor * input_axis(KeyCode::KeyZ, KeyCode::KeyX);
                        let qe = factor * input_axis(KeyCode::KeyQ, KeyCode::KeyE);

                        let wasd = Mat2::from_angle(-self.camera.yaw) * wasd;

                        self.camera.translate(vec3(wasd.x, qe, wasd.y));

                        self.camera.orbit(zx, 0.0);

                        self.camera_smoothed.lerp_exp(&self.camera, dt);
                    }
                };
                self.last_render_time = Some(now);

                renderer.render(self.camera_smoothed.matrix().inverse());
                window.request_redraw();
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::MouseInput {
                button: MouseButton::Back,
                state: ElementState::Pressed,
                ..
            }
            | WindowEvent::DoubleTapGesture { .. } => {
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
                    .orbit(-0.01 * delta.x as f32, -0.01 * delta.y as f32);
            }

            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, delta),
                ..
            } => {
                self.camera.zoom(-0.2 * delta as f32);
            }

            WindowEvent::PinchGesture { delta, .. } => {
                self.camera.zoom(-delta as f32);
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
                self.camera.orbit(-0.01 * x as f32, -0.01 * y as f32);
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
