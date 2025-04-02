use crate::{Model, Result, Transform};
use nalgebra::{UnitVector3, Vector3};

/// Applies an orthographic projection to a model.
#[derive(Debug, Clone, Copy)]
pub struct Orthographic {
    direction: UnitVector3<f32>,
    preserve_z: bool,
}

impl Orthographic {
    /// Create a new orthographic projection transformation.
    ///
    /// # Arguments
    /// * `direction` - The direction of the projection (unit vector)
    /// * `preserve_z` - If true, original z-values are preserved; if false, z-values are flattened
    pub fn new(direction: Vector3<f32>, preserve_z: bool) -> Self {
        Self {
            direction: UnitVector3::new_normalize(direction),
            preserve_z,
        }
    }

    /// Create an orthographic projection onto the XY plane.
    pub fn onto_xy() -> Self {
        Self::new(Vector3::new(0.0, 0.0, 1.0), false)
    }

    /// Create an orthographic projection onto the XZ plane.
    pub fn onto_xz() -> Self {
        Self::new(Vector3::new(0.0, 1.0, 0.0), false)
    }

    /// Create an orthographic projection onto the YZ plane.
    pub fn onto_yz() -> Self {
        Self::new(Vector3::new(1.0, 0.0, 0.0), false)
    }
}

impl Transform for Orthographic {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // If we want to preserve depth, we need to remember the original positions
        let mut original_depths = Vec::new();

        if self.preserve_z {
            for vertex in &model.mesh.vertices {
                let pos = &vertex.position;
                // Calculate "depth" along the direction vector
                let depth = self.direction.dot(&Vector3::new(pos.x, pos.y, pos.z));
                original_depths.push(depth);
            }
        }

        // Create orthogonal basis where one vector is the direction
        // The orthonormal_basis method was removed in newer nalgebra versions
        // Creating an orthonormal basis manually:
        let dir = self.direction.into_inner();

        // Create a vector that's not parallel to dir
        let not_parallel = if dir.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };

        // Create two orthogonal vectors
        let u = UnitVector3::new_normalize(dir.cross(&not_parallel));
        let v = UnitVector3::new_normalize(dir.cross(&u));

        for (i, vertex) in model.mesh.vertices.iter_mut().enumerate() {
            let position = &mut vertex.position;
            let pos_vec = Vector3::new(position.x, position.y, position.z);

            // Project position onto the two basis vectors perpendicular to the direction
            let u_comp = u.dot(&pos_vec);
            let v_comp = v.dot(&pos_vec);

            // Calculate the component along the projection direction
            let _dir_comp = self.direction.dot(&pos_vec);

            // New position is a combination of the u and v components
            // Need to convert UnitVector3 to Vector3 before multiplying
            let new_pos = u.into_inner() * u_comp + v.into_inner() * v_comp;

            position.x = new_pos.x;
            position.y = new_pos.y;
            position.z = new_pos.z;

            // If preserving depth, restore the original depth along the direction
            if self.preserve_z && i < original_depths.len() {
                let depth = original_depths[i];
                let depth_component = self.direction.into_inner() * depth;
                position.x += depth_component.x;
                position.y += depth_component.y;
                position.z += depth_component.z;
            }

            // For orthographic projection, all normals in the projection direction become zero
            // and other components stay the same
            vertex.normal = vertex.normal
                - vertex.normal.dot(&self.direction.into_inner()) * self.direction.into_inner();

            // Re-normalize if the normal is not zero
            if vertex.normal.magnitude() > 1e-6 {
                vertex.normal = vertex.normal.normalize();
            } else {
                // If normal becomes zero, set it to the projection direction
                vertex.normal = self.direction.into_inner();
            }
        }

        Ok(())
    }
}
