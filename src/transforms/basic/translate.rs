use crate::{Model, Result, Transform};

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
