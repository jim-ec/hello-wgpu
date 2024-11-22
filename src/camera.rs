use cgmath::{Matrix4, Quaternion, Rotation3, Vector3};

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
}

impl Camera {
    pub fn matrix(&self) -> Matrix4<f32> {
        let yaw = Quaternion::from_angle_y(cgmath::Rad(self.yaw));
        let pitch = Quaternion::from_angle_x(cgmath::Rad(self.pitch));
        let translation = Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.0 * self.radius));
        translation * Matrix4::from(pitch * yaw)
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
