use std::f32::consts::FRAC_PI_2;

use compute::export::{
    egui::{Context, Key, PointerButton},
    nalgebra::{Matrix4, Point3, Vector3},
};

pub struct Camera {
    pub position: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,

    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn update(&mut self, ctx: &Context) {
        let dragging_ui = ctx.drag_started_id().is_some() || ctx.dragged_id().is_some();
        ctx.input(|input| {
            let facing = self.facing();
            let forward = Vector3::new(facing.x, 0.0, facing.z).normalize();
            let right = facing.cross(&Vector3::new(0.0, 1.0, 0.0));
            let directions = [
                (Key::W, forward),
                (Key::S, -forward),
                (Key::A, -right),
                (Key::D, right),
                (Key::Space, Vector3::new(0.0, 1.0, 0.0)),
            ];

            let mut delta = Vector3::zeros();
            delta -= Vector3::new(0.0, 1.0, 0.0) * input.modifiers.shift as u8 as f32;
            for (key, direction) in directions.iter() {
                delta += direction * input.key_down(*key) as u8 as f32;
            }

            self.position += delta.try_normalize(0.0).unwrap_or_default() * 10.0 * input.stable_dt;

            if input.pointer.button_down(PointerButton::Primary) && !dragging_ui {
                let mouse = -input.pointer.delta() * 0.01;
                self.pitch += mouse.y;
                self.yaw += mouse.x;
            }
        });
    }

    pub fn facing(&self) -> Vector3<f32> {
        Vector3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
    }

    pub fn view_projection(&self, aspect: f32) -> Matrix4<f32> {
        let facing = self.facing();
        Matrix4::new_perspective(aspect, self.fov, self.near, self.far)
            * Matrix4::look_at_rh(
                &Point3::from(self.position),
                &Point3::from(self.position + facing),
                &Vector3::new(0.0, 1.0, 0.0),
            )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector3::zeros(),
            pitch: 0.0,
            yaw: 0.0,

            fov: FRAC_PI_2,
            near: 0.1,
            far: 10_000.0,
        }
    }
}
