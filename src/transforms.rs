//! Transformations that can be applied to 3D models.

use nalgebra::{Matrix4, Vector3, Point3, Rotation3};
use std::f32::consts::PI;
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

/// Translates a model in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Translate {
    x: f32,
    y: f32,
    z: f32,
}

impl Translate {
    /// Create a new translation transformation.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Transform for Translate {
    fn apply(&self, model: &mut Model) -> Result<()> {
        for vertex in &mut model.mesh.vertices {
            vertex.position.x += self.x;
            vertex.position.y += self.y;
            vertex.position.z += self.z;
        }
        
        Ok(())
    }
}

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

/// Applies a general 4x4 transformation matrix to a model.
#[derive(Debug, Clone)]
pub struct Matrix {
    matrix: Matrix4<f32>,
    normal_matrix: Matrix4<f32>,
}

impl Matrix {
    /// Create a new matrix transformation.
    pub fn new(matrix: Matrix4<f32>) -> Self {
        // Compute the normal transformation matrix (inverse transpose)
        let normal_matrix = matrix.try_inverse().unwrap_or_else(Matrix4::identity).transpose();
        
        Self { matrix, normal_matrix }
    }
}

impl Transform for Matrix {
    fn apply(&self, model: &mut Model) -> Result<()> {
        for vertex in &mut model.mesh.vertices {
            // Transform position with the full matrix
            let position = &mut vertex.position;
            let homogeneous = self.matrix * Point3::new(position.x, position.y, position.z).to_homogeneous();
            
            if homogeneous.w != 0.0 {
                position.x = homogeneous.x / homogeneous.w;
                position.y = homogeneous.y / homogeneous.w;
                position.z = homogeneous.z / homogeneous.w;
            } else {
                return Err(Error::TransformError(
                    "Matrix transformation resulted in point at infinity".to_string(),
                ));
            }
            
            // Transform normal with the normal matrix
            let normal = &mut vertex.normal;
            let transformed_normal = self.normal_matrix * Vector3::new(normal.x, normal.y, normal.z).to_homogeneous();
            normal.x = transformed_normal.x;
            normal.y = transformed_normal.y;
            normal.z = transformed_normal.z;
            
            if normal.magnitude() > 0.0 {
                *normal = normal.normalize();
            }
        }
        
        Ok(())
    }
}

/// Applies a mirror reflection to a model.
#[derive(Debug, Clone, Copy)]
pub struct Mirror {
    x: bool,
    y: bool,
    z: bool,
}

impl Mirror {
    /// Create a new mirroring transformation.
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }
    
    /// Mirror across the YZ plane.
    pub fn x() -> Self {
        Self::new(true, false, false)
    }
    
    /// Mirror across the XZ plane.
    pub fn y() -> Self {
        Self::new(false, true, false)
    }
    
    /// Mirror across the XY plane.
    pub fn z() -> Self {
        Self::new(false, false, true)
    }
}

impl Transform for Mirror {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // Number of reflection planes (to determine if we need to flip faces)
        let reflection_count = self.x as u8 + self.y as u8 + self.z as u8;
        let flip_winding = reflection_count % 2 == 1;
        
        // Apply mirroring to vertices
        for vertex in &mut model.mesh.vertices {
            if self.x {
                vertex.position.x = -vertex.position.x;
                vertex.normal.x = -vertex.normal.x;
            }
            
            if self.y {
                vertex.position.y = -vertex.position.y;
                vertex.normal.y = -vertex.normal.y;
            }
            
            if self.z {
                vertex.position.z = -vertex.position.z;
                vertex.normal.z = -vertex.normal.z;
            }
        }
        
        // If we need to flip the winding order to maintain correct face orientation
        if flip_winding {
            for face in &mut model.mesh.faces {
                if face.indices.len() >= 3 {
                    // Reverse the winding order by reversing the vertex indices
                    face.indices.reverse();
                }
            }
        }
        
        Ok(())
    }
} 