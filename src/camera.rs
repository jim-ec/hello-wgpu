use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
}

impl Camera {
    pub fn matrix(&self) -> Mat4 {
        let yaw = Quat::from_rotation_y(self.yaw);
        let pitch = Quat::from_rotation_x(self.pitch);
        let translation = Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0 * self.radius));
        translation * Mat4::from_quat(pitch * yaw)
    }

    /// Interpolate between this camera and another camera in a frame-rate independent way.
    pub fn lerp_exp(&mut self, other: &Self, stiffness: f32, dt: f32) {
        let rate = -60.0 * (1.0 - stiffness).ln();
        let interpolant = 1.0 - (-rate * dt).exp();
        self.yaw += interpolant * (other.yaw - self.yaw);
        self.pitch += interpolant * (other.pitch - self.pitch);
        self.radius += interpolant * (other.radius - self.radius);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            yaw: 1.0,
            pitch: 0.5,
            radius: 4.0,
        }
    }
}
