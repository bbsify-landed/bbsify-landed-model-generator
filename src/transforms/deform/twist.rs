use nalgebra::{Vector3, Rotation3, Unit};
use std::f32::consts::PI;
use crate::{Model, Transform, Result};

/// Applies a twist deformation around an axis.
#[derive(Debug, Clone, Copy)]
pub struct Twist {
    axis: Vector3<f32>,
    angle_per_unit: f32,
    center: Vector3<f32>,
}

impl Twist {
    /// Create a new twist transformation.
    /// 
    /// The model will be twisted around the specified axis, with the
    /// twist angle increasing linearly with distance from the center point
    /// along the axis.
    /// 
    /// # Arguments
    /// * `axis` - The axis to twist around
    /// * `angle_per_unit` - The angle (in degrees) to twist per unit of distance along the axis
    /// * `center` - The center point of the twist
    pub fn new(axis: Vector3<f32>, angle_per_unit: f32, center: Vector3<f32>) -> Self {
        Self {
            axis: axis.normalize(), 
            angle_per_unit: angle_per_unit * PI / 180.0,
            center,
        }
    }
    
    /// Create a twist around the X axis.
    pub fn around_x(angle_per_unit: f32, center_y: f32, center_z: f32) -> Self {
        Self::new(
            Vector3::new(1.0, 0.0, 0.0),
            angle_per_unit,
            Vector3::new(0.0, center_y, center_z),
        )
    }
    
    /// Create a twist around the Y axis.
    pub fn around_y(angle_per_unit: f32, center_x: f32, center_z: f32) -> Self {
        Self::new(
            Vector3::new(0.0, 1.0, 0.0),
            angle_per_unit,
            Vector3::new(center_x, 0.0, center_z),
        )
    }
    
    /// Create a twist around the Z axis.
    pub fn around_z(angle_per_unit: f32, center_x: f32, center_y: f32) -> Self {
        Self::new(
            Vector3::new(0.0, 0.0, 1.0),
            angle_per_unit,
            Vector3::new(center_x, center_y, 0.0),
        )
    }
}

impl Transform for Twist {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // Project each vertex onto the axis to determine twist amount
        for vertex in &mut model.mesh.vertices {
            let position = &mut vertex.position;
            
            // Vector from center to current position
            let center_to_pos = Vector3::new(
                position.x - self.center.x,
                position.y - self.center.y,
                position.z - self.center.z,
            );
            
            // Project onto axis to find distance along axis
            let projection = center_to_pos.dot(&self.axis);
            
            // Calculate twist angle based on distance along axis
            let angle = projection * self.angle_per_unit;
            
            // Create rotation around the axis
            let unit_axis = Unit::new_normalize(self.axis);
            let rotation = Rotation3::from_axis_angle(&unit_axis, angle);
            
            // Point relative to center
            let pos_rel = Vector3::new(
                position.x - self.center.x,
                position.y - self.center.y,
                position.z - self.center.z,
            );
            
            // Component along axis (stays the same)
            let component_along_axis = self.axis * projection;
            
            // Component perpendicular to axis (gets rotated)
            let perp_component = pos_rel - component_along_axis;
            
            // Apply rotation to perpendicular component
            let rotated_perp = rotation * perp_component;
            
            // Reconstruct position
            let new_pos = component_along_axis + rotated_perp + self.center;
            position.x = new_pos.x;
            position.y = new_pos.y;
            position.z = new_pos.z;
            
            // Rotate normal
            vertex.normal = rotation * vertex.normal;
            // Normalization is likely needed after multiple deformations
            vertex.normal = vertex.normal.normalize();
        }
        
        Ok(())
    }
} 