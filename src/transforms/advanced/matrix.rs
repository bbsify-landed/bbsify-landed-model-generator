use nalgebra::{Matrix4, Point3, Vector3};
use crate::{Model, Transform, Result, Error};

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