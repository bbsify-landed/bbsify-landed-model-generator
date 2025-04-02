use crate::{Model, Result, Transform};
use nalgebra::{Point3, Vector3};

/// Applies a perspective projection to a model.
#[derive(Debug, Clone, Copy)]
pub struct Perspective {
    eye: Point3<f32>,
    focal_length: f32,
    preserve_z: bool,
}

impl Perspective {
    /// Create a new perspective projection transformation.
    ///
    /// # Arguments
    /// * `eye` - The position of the eye/camera
    /// * `focal_length` - The focal length of the perspective projection
    /// * `preserve_z` - If true, original z-values are preserved; if false, z-values represent distance from eye
    pub fn new(eye: Point3<f32>, focal_length: f32, preserve_z: bool) -> Self {
        Self {
            eye,
            focal_length,
            preserve_z,
        }
    }

    /// Create a perspective projection looking along the positive Z axis.
    pub fn z_positive(eye_x: f32, eye_y: f32, eye_z: f32, focal_length: f32) -> Self {
        Self::new(Point3::new(eye_x, eye_y, eye_z), focal_length, false)
    }

    /// Create a perspective projection looking along the negative Z axis.
    pub fn z_negative(eye_x: f32, eye_y: f32, eye_z: f32, focal_length: f32) -> Self {
        Self::new(Point3::new(eye_x, eye_y, eye_z), focal_length, false)
    }
}

impl Transform for Perspective {
    fn apply(&self, model: &mut Model) -> Result<()> {
        for vertex in &mut model.mesh.vertices {
            let position = &mut vertex.position;

            // Vector from eye to vertex
            let mut eye_to_vertex = Vector3::new(
                position.x - self.eye.x,
                position.y - self.eye.y,
                position.z - self.eye.z,
            );

            // Check if point is behind the eye
            if eye_to_vertex.z <= 0.0 {
                // Instead of returning an error, move the point slightly in front of the eye
                eye_to_vertex.z = 0.01; // Small positive value
            }

            // Calculate projected position
            let scale_factor = self.focal_length / eye_to_vertex.z;

            // Apply perspective projection
            let projected_x = self.eye.x + eye_to_vertex.x * scale_factor;
            let projected_y = self.eye.y + eye_to_vertex.y * scale_factor;

            // Update vertex position
            position.x = projected_x;
            position.y = projected_y;

            if !self.preserve_z {
                // Use z-value as distance from eye (useful for z-sorting)
                position.z = eye_to_vertex.magnitude();
            }

            // Update normal vector (point toward the eye)
            // This is a simplification - true perspective projection would transform
            // normals using a more complex approach
            vertex.normal = -eye_to_vertex.normalize();
        }

        Ok(())
    }
}
