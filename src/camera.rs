use std::f32::consts::TAU;

use glam::{Mat4, Quat, Vec3};

const STIFFNESS: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    yaw: f32,
    pitch: f32,
    zoom: f32,
}

impl Camera {
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.zoom += zoom;
    }

    pub fn pan(&mut self, rightwards: f32, upwards: f32) {
        let rotation = self.rotation().inverse();
        self.origin += rotation * Vec3::new(rightwards, upwards, 0.0);
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.origin += translation;
    }

    pub fn reset(&mut self) {
        let default = Self::default();
        *self = Self {
            yaw: TAU * (self.yaw / TAU).round() + default.yaw,
            pitch: TAU * (self.pitch / TAU).round() + default.pitch,
            ..default
        }
    }

    pub fn matrix(&self) -> Mat4 {
        let rotation = Mat4::from_quat(self.rotation());
        let translation_radius = Mat4::from_translation(Vec3::new(0.0, 0.0, -self.zoom.exp()));
        let translation_origin = Mat4::from_translation(self.origin);
        translation_radius * rotation * translation_origin
    }

    pub fn rotation(&self) -> Quat {
        let yaw = Quat::from_rotation_y(self.yaw);
        let pitch = Quat::from_rotation_x(self.pitch);
        pitch * yaw
    }

    /// Interpolate between this camera and another camera in a frame-rate independent way.
    pub fn lerp_exp(&mut self, other: &Self, dt: f32) {
        const REFERENCE_FPS: f32 = 60.0;
        let rate = -REFERENCE_FPS * (1.0 - STIFFNESS).ln();
        let interpolant = 1.0 - (-rate * dt).exp();
        self.yaw += interpolant * (other.yaw - self.yaw);
        self.pitch += interpolant * (other.pitch - self.pitch);
        self.zoom += interpolant * (other.zoom - self.zoom);
        self.origin += interpolant * (other.origin - self.origin);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::ZERO,
            yaw: 1.0,
            pitch: 0.5,
            zoom: 2.0,
        }
    }
}
