use crate::{Model, Result, Transform};
use nalgebra::Vector3;

/// Applies a tapering deformation along an axis.
#[derive(Debug, Clone, Copy)]
pub struct Taper {
    axis: Vector3<f32>,
    start_scale: Vector3<f32>,
    end_scale: Vector3<f32>,
    bounds: (f32, f32),
}

impl Taper {
    /// Create a new tapering transformation.
    ///
    /// Vertices will be scaled perpendicular to the main axis, with the
    /// scale factor varying linearly from start_scale to end_scale
    /// as the vertex position moves from bounds.0 to bounds.1 along the axis.
    ///
    /// # Arguments
    /// * `axis` - The axis along which to taper
    /// * `start_scale` - The scale factor at the start bound (x, y, z)
    /// * `end_scale` - The scale factor at the end bound (x, y, z)
    /// * `bounds` - The range along the axis to apply the taper (start, end)
    pub fn new(
        axis: Vector3<f32>,
        start_scale: Vector3<f32>,
        end_scale: Vector3<f32>,
        bounds: (f32, f32),
    ) -> Self {
        Self {
            axis: axis.normalize(),
            start_scale,
            end_scale,
            bounds,
        }
    }

    /// Create a taper along the X axis.
    pub fn x_axis(start_scale: (f32, f32), end_scale: (f32, f32), x_range: (f32, f32)) -> Self {
        Self::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, start_scale.0, start_scale.1),
            Vector3::new(1.0, end_scale.0, end_scale.1),
            x_range,
        )
    }

    /// Create a taper along the Y axis.
    pub fn y_axis(start_scale: (f32, f32), end_scale: (f32, f32), y_range: (f32, f32)) -> Self {
        Self::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(start_scale.0, 1.0, start_scale.1),
            Vector3::new(end_scale.0, 1.0, end_scale.1),
            y_range,
        )
    }

    /// Create a taper along the Z axis.
    pub fn z_axis(start_scale: (f32, f32), end_scale: (f32, f32), z_range: (f32, f32)) -> Self {
        Self::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(start_scale.0, start_scale.1, 1.0),
            Vector3::new(end_scale.0, end_scale.1, 1.0),
            z_range,
        )
    }
}

impl Transform for Taper {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // Calculate range length for the tapering region
        let range_length = self.bounds.1 - self.bounds.0;

        // Ensure we don't divide by zero
        if range_length.abs() < 1e-5 {
            return Ok(());
        }

        // Get two perpendicular axes
        let p1 = if self.axis.x.abs() < 0.9 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };

        let perp1 = p1 - (p1.dot(&self.axis) * self.axis);
        let perp1 = perp1.normalize();
        let perp2 = self.axis.cross(&perp1).normalize();

        for vertex in &mut model.mesh.vertices {
            let position = &mut vertex.position;
            let pos_vec = Vector3::new(position.x, position.y, position.z);

            // Calculate position along taper axis
            let pos_along_axis = pos_vec.dot(&self.axis);

            // Skip vertices outside the taper range
            if pos_along_axis < self.bounds.0 || pos_along_axis > self.bounds.1 {
                continue;
            }

            // Calculate interpolation factor (0.0 at start, 1.0 at end)
            let t = (pos_along_axis - self.bounds.0) / range_length;

            // Interpolate scale factors
            let scale_x = self.start_scale.x * (1.0 - t) + self.end_scale.x * t;
            let scale_y = self.start_scale.y * (1.0 - t) + self.end_scale.y * t;
            let scale_z = self.start_scale.z * (1.0 - t) + self.end_scale.z * t;

            // Find a point on the axis
            let axis_point = self.axis * pos_along_axis;

            // Vector from axis to the point
            let from_axis = pos_vec - axis_point;

            // Decompose into components along our perpendicular axes
            let comp1 = from_axis.dot(&perp1) * perp1;
            let comp2 = from_axis.dot(&perp2) * perp2;

            // Apply scale to the components appropriately based on axis orientation
            let scaled_comp1: Vector3<f32>;
            let scaled_comp2: Vector3<f32>;

            if self.axis.x.abs() > 0.9 {
                // X is the main axis
                scaled_comp1 = comp1 * scale_y;
                scaled_comp2 = comp2 * scale_z;
            } else if self.axis.y.abs() > 0.9 {
                // Y is the main axis
                scaled_comp1 = comp1 * scale_x;
                scaled_comp2 = comp2 * scale_z;
            } else {
                // Z is the main axis
                scaled_comp1 = comp1 * scale_x;
                scaled_comp2 = comp2 * scale_y;
            }

            // Calculate new position
            let new_pos = axis_point + scaled_comp1 + scaled_comp2;
            position.x = new_pos.x;
            position.y = new_pos.y;
            position.z = new_pos.z;

            // Handle normal transformation
            // For a taper, the normals get more complex - this is a first approximation
            // A proper solution would compute the Jacobian matrix of the deformation
            // For now, we'll use the inverse of the scale factors
            let normal = &mut vertex.normal;

            let normal_along_axis = normal.dot(&self.axis) * self.axis;
            let normal_perp1 = normal.dot(&perp1) * perp1;
            let normal_perp2 = normal.dot(&perp2) * perp2;

            let scaled_normal_perp1: Vector3<f32>;
            let scaled_normal_perp2: Vector3<f32>;

            if self.axis.x.abs() > 0.9 {
                // X is the main axis
                scaled_normal_perp1 = if scale_y != 0.0 {
                    normal_perp1 / scale_y
                } else {
                    normal_perp1
                };
                scaled_normal_perp2 = if scale_z != 0.0 {
                    normal_perp2 / scale_z
                } else {
                    normal_perp2
                };
            } else if self.axis.y.abs() > 0.9 {
                // Y is the main axis
                scaled_normal_perp1 = if scale_x != 0.0 {
                    normal_perp1 / scale_x
                } else {
                    normal_perp1
                };
                scaled_normal_perp2 = if scale_z != 0.0 {
                    normal_perp2 / scale_z
                } else {
                    normal_perp2
                };
            } else {
                // Z is the main axis
                scaled_normal_perp1 = if scale_x != 0.0 {
                    normal_perp1 / scale_x
                } else {
                    normal_perp1
                };
                scaled_normal_perp2 = if scale_y != 0.0 {
                    normal_perp2 / scale_y
                } else {
                    normal_perp2
                };
            }

            *normal = normal_along_axis + scaled_normal_perp1 + scaled_normal_perp2;

            // Normalize to maintain unit length
            if normal.magnitude() > 0.0 {
                *normal = normal.normalize();
            }
        }

        Ok(())
    }
}
