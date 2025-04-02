use crate::{Model, Result, Transform};
use nalgebra::{Rotation3, Vector3};
use std::f32::consts::PI;

/// Rotates a model around an axis.
#[derive(Debug, Clone, Copy)]
pub struct Rotate {
    axis: Vector3<f32>,
    angle_rad: f32,
}

impl Rotate {
    /// Create a new rotation transformation.
    pub fn new(axis: Vector3<f32>, angle_degrees: f32) -> Self {
        Self {
            axis: axis.normalize(),
            angle_rad: angle_degrees * PI / 180.0,
        }
    }

    /// Rotate around the X axis.
    pub fn around_x(angle_degrees: f32) -> Self {
        Self::new(Vector3::new(1.0, 0.0, 0.0), angle_degrees)
    }

    /// Rotate around the Y axis.
    pub fn around_y(angle_degrees: f32) -> Self {
        Self::new(Vector3::new(0.0, 1.0, 0.0), angle_degrees)
    }

    /// Rotate around the Z axis.
    pub fn around_z(angle_degrees: f32) -> Self {
        Self::new(Vector3::new(0.0, 0.0, 1.0), angle_degrees)
    }
}

impl Transform for Rotate {
    fn apply(&self, model: &mut Model) -> Result<()> {
        let unit_axis = nalgebra::Unit::new_normalize(self.axis);
        let rotation = Rotation3::from_axis_angle(&unit_axis, self.angle_rad);

        for vertex in &mut model.mesh.vertices {
            // Rotate position
            let position = &mut vertex.position;
            let rotated_position = rotation * Vector3::new(position.x, position.y, position.z);
            position.x = rotated_position.x;
            position.y = rotated_position.y;
            position.z = rotated_position.z;

            // Rotate normal
            vertex.normal = rotation * vertex.normal;
        }

        Ok(())
    }
}
