use crate::{Model, Result, Transform};
use nalgebra::{Rotation3, Unit, Vector3};
use std::f32::consts::PI;

/// Applies a bend deformation along an axis.
#[derive(Debug, Clone, Copy)]
pub struct Bend {
    bend_axis: Vector3<f32>,
    bend_angle: f32,
    bend_region: (f32, f32),
    direction_axis: Vector3<f32>,
}

impl Bend {
    /// Create a new bend transformation.
    ///
    /// # Arguments
    /// * `bend_axis` - The axis around which to bend the model
    /// * `bend_angle` - The total angle (in degrees) to bend
    /// * `bend_region` - The range along the direction_axis to apply the bend (start, end)
    /// * `direction_axis` - The axis along which to measure the bend region
    pub fn new(
        bend_axis: Vector3<f32>,
        bend_angle: f32,
        bend_region: (f32, f32),
        direction_axis: Vector3<f32>,
    ) -> Self {
        Self {
            bend_axis: bend_axis.normalize(),
            bend_angle: bend_angle * PI / 180.0,
            bend_region,
            direction_axis: direction_axis.normalize(),
        }
    }

    /// Create a bend around the X axis, with Y as the direction axis.
    pub fn x_axis(bend_angle: f32, y_min: f32, y_max: f32) -> Self {
        Self::new(
            Vector3::new(1.0, 0.0, 0.0),
            bend_angle,
            (y_min, y_max),
            Vector3::new(0.0, 1.0, 0.0),
        )
    }

    /// Create a bend around the Y axis, with X as the direction axis.
    pub fn y_axis(bend_angle: f32, x_min: f32, x_max: f32) -> Self {
        Self::new(
            Vector3::new(0.0, 1.0, 0.0),
            bend_angle,
            (x_min, x_max),
            Vector3::new(1.0, 0.0, 0.0),
        )
    }

    /// Create a bend around the Z axis, with X as the direction axis.
    pub fn z_axis(bend_angle: f32, x_min: f32, x_max: f32) -> Self {
        Self::new(
            Vector3::new(0.0, 0.0, 1.0),
            bend_angle,
            (x_min, x_max),
            Vector3::new(1.0, 0.0, 0.0),
        )
    }
}

impl Transform for Bend {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // Calculate the center of the bend region
        let start = self.bend_region.0;
        let end = self.bend_region.1;
        let bend_angle = self.bend_angle;

        let _region_center = (self.bend_region.0 + self.bend_region.1) / 2.0;

        // Ensure we don't divide by zero
        if (end - start).abs() < 1e-5 {
            return Ok(());
        }

        // Create the third axis (perpendicular to bend_axis and direction_axis)
        let offset_axis = self.bend_axis.cross(&self.direction_axis).normalize();

        for vertex in &mut model.mesh.vertices {
            let position = &mut vertex.position;

            // Project vertex position onto direction axis to determine where in the bend region it falls
            let pos_vec = Vector3::new(position.x, position.y, position.z);
            let pos_along_dir = pos_vec.dot(&self.direction_axis);

            // Skip vertices outside the bend region
            if pos_along_dir < start || pos_along_dir > end {
                continue;
            }

            // Calculate bend factor (0 at start of region, 1 at end)
            let bend_factor = (pos_along_dir - start) / (end - start);

            // Calculate the angle for this vertex
            let vertex_angle = bend_angle * bend_factor;

            // Create rotation around the bend axis
            let unit_axis = Unit::new_normalize(self.bend_axis);
            let rotation = Rotation3::from_axis_angle(&unit_axis, vertex_angle);

            // Calculate pivot point on the bend axis at the start of the region
            let pivot = self.direction_axis * start;

            // Calculate position relative to pivot
            let rel_pos = pos_vec - pivot;

            // Calculate projection of relative position onto direction axis
            let proj_dir = rel_pos.dot(&self.direction_axis) * self.direction_axis;

            // Calculate component perpendicular to the bend and direction axes
            let offset_component = rel_pos.dot(&offset_axis) * offset_axis;

            // Rotate the projection and offset
            let rotated_proj = rotation * proj_dir;
            let rotated_offset = rotation * offset_component;

            // Recalculate position with rotated components
            let new_pos = pivot + rotated_proj + rotated_offset;
            position.x = new_pos.x;
            position.y = new_pos.y;
            position.z = new_pos.z;

            // Transform normal
            vertex.normal = rotation * vertex.normal;
            vertex.normal = vertex.normal.normalize();
        }

        Ok(())
    }
}
