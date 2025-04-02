use nalgebra::Vector3;
use crate::{Model, Transform, Result};

/// Applies a cylindrical projection to a model.
#[derive(Debug, Clone, Copy)]
pub struct Cylindrical {
    axis: Vector3<f32>,
    center: Vector3<f32>,
    radius: f32,
    preserve_radius: bool,
}

impl Cylindrical {
    /// Create a new cylindrical projection transformation.
    /// 
    /// # Arguments
    /// * `axis` - The axis of the cylinder
    /// * `center` - A point on the axis of the cylinder
    /// * `radius` - The radius of the cylinder
    /// * `preserve_radius` - If true, original distances from axis are preserved;
    ///                       if false, all points are mapped to the cylinder surface
    pub fn new(axis: Vector3<f32>, center: Vector3<f32>, radius: f32, preserve_radius: bool) -> Self {
        Self {
            axis: axis.normalize(),
            center,
            radius,
            preserve_radius,
        }
    }
    
    /// Create a cylindrical projection along the X axis.
    pub fn x_axis(center_y: f32, center_z: f32, radius: f32) -> Self {
        Self::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, center_y, center_z),
            radius,
            false,
        )
    }
    
    /// Create a cylindrical projection along the Y axis.
    pub fn y_axis(center_x: f32, center_z: f32, radius: f32) -> Self {
        Self::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(center_x, 0.0, center_z),
            radius,
            false,
        )
    }
    
    /// Create a cylindrical projection along the Z axis.
    pub fn z_axis(center_x: f32, center_y: f32, radius: f32) -> Self {
        Self::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(center_x, center_y, 0.0),
            radius,
            false,
        )
    }
}

impl Transform for Cylindrical {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // Get two perpendicular axes to form a coordinate system
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
            
            // Vector from center to position
            let center_to_pos = pos_vec - self.center;
            
            // Project onto the cylinder axis to find height along axis
            let height = center_to_pos.dot(&self.axis);
            let height_component = self.axis * height;
            
            // Component perpendicular to axis
            let perp_component = center_to_pos - height_component;
            let dist_from_axis = perp_component.magnitude();
            
            // Skip if point is on the axis (avoid division by zero)
            if dist_from_axis < 1e-6 {
                continue;
            }
            
            // Calculate angle around the cylinder
            let dot_perp1 = perp_component.dot(&perp1);
            let dot_perp2 = perp_component.dot(&perp2);
            let angle = dot_perp2.atan2(dot_perp1);
            
            // Determine the new radius based on preserve_radius flag
            let new_radius = if self.preserve_radius { dist_from_axis } else { self.radius };
            
            // Calculate the point on the cylindrical surface
            let new_perp = perp1 * new_radius * angle.cos() + perp2 * new_radius * angle.sin();
            
            // Combine axis and perpendicular components
            let new_pos = self.center + height_component + new_perp;
            position.x = new_pos.x;
            position.y = new_pos.y;
            position.z = new_pos.z;
            
            // Transform the normal to point outward from the cylinder axis
            // (approximation - a full transform would be more complex)
            if !self.preserve_radius {
                // Normal points outward from the axis
                vertex.normal = new_perp.normalize();
            } else {
                // For preserve_radius mode, normal direction mostly maintained
                // but still needs to be properly transformed
                let normal = &mut vertex.normal;
                
                // Project normal onto the axis and perpendicular plane
                let normal_axis_comp = normal.dot(&self.axis) * self.axis;
                let normal_perp_comp = *normal - normal_axis_comp;
                
                // Calculate the angle-based transform for the perpendicular component
                let new_normal_perp = perp1 * angle.cos() + perp2 * angle.sin();
                
                // Combine components
                *normal = normal_axis_comp + new_normal_perp.normalize() * normal_perp_comp.magnitude();
                
                // Normalize the final normal
                if normal.magnitude() > 0.0 {
                    *normal = normal.normalize();
                }
            }
        }
        
        Ok(())
    }
} 