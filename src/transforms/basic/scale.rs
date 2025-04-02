use nalgebra::Vector3;
use crate::{Model, Transform, Result, Error};

/// Scales a model uniformly or non-uniformly.
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    x: f32,
    y: f32,
    z: f32,
}

impl Scale {
    /// Create a new scaling transformation.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    /// Create a uniform scaling transformation.
    pub fn uniform(scale: f32) -> Self {
        Self { x: scale, y: scale, z: scale }
    }
}

impl Transform for Scale {
    fn apply(&self, model: &mut Model) -> Result<()> {
        for vertex in &mut model.mesh.vertices {
            vertex.position.x *= self.x;
            vertex.position.y *= self.y;
            vertex.position.z *= self.z;
            
            // Handle normal scaling (inverse transpose for non-uniform scaling)
            if self.x == self.y && self.y == self.z {
                // Uniform scaling doesn't change the normal direction
                // but we should normalize to keep unit length
                if self.x != 0.0 {
                    vertex.normal = vertex.normal.normalize();
                }
            } else {
                // Non-uniform scaling requires the inverse transpose
                // which for a diagonal matrix is just 1/scale
                if self.x != 0.0 && self.y != 0.0 && self.z != 0.0 {
                    let nx = vertex.normal.x / self.x;
                    let ny = vertex.normal.y / self.y;
                    let nz = vertex.normal.z / self.z;
                    vertex.normal = Vector3::new(nx, ny, nz).normalize();
                } else {
                    return Err(Error::TransformError(
                        "Cannot scale by zero in any dimension".to_string(),
                    ));
                }
            }
        }
        
        Ok(())
    }
} 