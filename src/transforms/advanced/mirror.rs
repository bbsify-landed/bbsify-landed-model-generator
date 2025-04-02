use crate::{Model, Result, Transform};

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
