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
