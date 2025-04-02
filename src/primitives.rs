//! Primitive 3D shapes that can be used as building blocks.

use nalgebra::{Point3, Vector3};
use crate::{Model, Vertex, Face};

/// Builder for creating a cube primitive.
pub struct Cube {
    size: f32,
    center: (f32, f32, f32),
    with_uvs: bool,
}

impl Cube {
    /// Create a new cube builder with default settings.
    pub fn new() -> Self {
        Self {
            size: 1.0,
            center: (0.0, 0.0, 0.0),
            with_uvs: true,
        }
    }
    
    /// Set the size of the cube.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    /// Set the center position of the cube.
    pub fn center(mut self, x: f32, y: f32, z: f32) -> Self {
        self.center = (x, y, z);
        self
    }
    
    /// Set whether to generate texture coordinates.
    pub fn with_uvs(mut self, with_uvs: bool) -> Self {
        self.with_uvs = with_uvs;
        self
    }
    
    /// Build the cube model.
    pub fn build(self) -> Model {
        let mut model = Model::new("Cube");
        let half = self.size / 2.0;
        let (cx, cy, cz) = self.center;
        
        // Create vertices (8 corners of the cube)
        let vertices = [
            // Front face
            Vertex::new(
                Point3::new(cx - half, cy - half, cz + half),
                Vector3::new(0.0, 0.0, 1.0),
                if self.with_uvs { Some((0.0, 0.0)) } else { None },
            ),
            Vertex::new(
                Point3::new(cx + half, cy - half, cz + half),
                Vector3::new(0.0, 0.0, 1.0),
                if self.with_uvs { Some((1.0, 0.0)) } else { None },
            ),
            Vertex::new(
                Point3::new(cx + half, cy + half, cz + half),
                Vector3::new(0.0, 0.0, 1.0),
                if self.with_uvs { Some((1.0, 1.0)) } else { None },
            ),
            Vertex::new(
                Point3::new(cx - half, cy + half, cz + half),
                Vector3::new(0.0, 0.0, 1.0),
                if self.with_uvs { Some((0.0, 1.0)) } else { None },
            ),
            
            // Back face
            Vertex::new(
                Point3::new(cx - half, cy - half, cz - half),
                Vector3::new(0.0, 0.0, -1.0),
                if self.with_uvs { Some((1.0, 0.0)) } else { None },
            ),
            Vertex::new(
                Point3::new(cx - half, cy + half, cz - half),
                Vector3::new(0.0, 0.0, -1.0),
                if self.with_uvs { Some((1.0, 1.0)) } else { None },
            ),
            Vertex::new(
                Point3::new(cx + half, cy + half, cz - half),
                Vector3::new(0.0, 0.0, -1.0),
                if self.with_uvs { Some((0.0, 1.0)) } else { None },
            ),
            Vertex::new(
                Point3::new(cx + half, cy - half, cz - half),
                Vector3::new(0.0, 0.0, -1.0),
                if self.with_uvs { Some((0.0, 0.0)) } else { None },
            ),
        ];
        
        // Add vertices to mesh
        let mut indices = Vec::new();
        for vertex in &vertices {
            indices.push(model.mesh.add_vertex(vertex.clone()));
        }
        
        // Define faces (6 faces of the cube, each as 2 triangles)
        // Front face
        model.mesh.add_face(Face::triangle(indices[0], indices[1], indices[2]), None);
        model.mesh.add_face(Face::triangle(indices[0], indices[2], indices[3]), None);
        
        // Back face
        model.mesh.add_face(Face::triangle(indices[4], indices[5], indices[6]), None);
        model.mesh.add_face(Face::triangle(indices[4], indices[6], indices[7]), None);
        
        // Top face
        model.mesh.add_face(Face::triangle(indices[3], indices[2], indices[6]), None);
        model.mesh.add_face(Face::triangle(indices[3], indices[6], indices[5]), None);
        
        // Bottom face
        model.mesh.add_face(Face::triangle(indices[0], indices[4], indices[7]), None);
        model.mesh.add_face(Face::triangle(indices[0], indices[7], indices[1]), None);
        
        // Right face
        model.mesh.add_face(Face::triangle(indices[1], indices[7], indices[6]), None);
        model.mesh.add_face(Face::triangle(indices[1], indices[6], indices[2]), None);
        
        // Left face
        model.mesh.add_face(Face::triangle(indices[0], indices[3], indices[5]), None);
        model.mesh.add_face(Face::triangle(indices[0], indices[5], indices[4]), None);
        
        model
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating a sphere primitive.
pub struct Sphere {
    radius: f32,
    center: (f32, f32, f32),
    segments: usize,
    rings: usize,
}

impl Sphere {
    /// Create a new sphere builder with default settings.
    pub fn new() -> Self {
        Self {
            radius: 1.0,
            center: (0.0, 0.0, 0.0),
            segments: 32,
            rings: 16,
        }
    }
    
    /// Set the radius of the sphere.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
    
    /// Set the center position of the sphere.
    pub fn center(mut self, x: f32, y: f32, z: f32) -> Self {
        self.center = (x, y, z);
        self
    }
    
    /// Set the number of horizontal segments (longitude).
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }
    
    /// Set the number of vertical rings (latitude).
    pub fn rings(mut self, rings: usize) -> Self {
        self.rings = rings;
        self
    }
    
    /// Build the sphere model.
    pub fn build(self) -> Model {
        let mut model = Model::new("Sphere");
        let (cx, cy, cz) = self.center;
        
        // Add top vertex
        let top_idx = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx, cy + self.radius, cz),
            Vector3::new(0.0, 1.0, 0.0),
            Some((0.5, 1.0)),
        ));
        
        // Add bottom vertex
        let bottom_idx = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx, cy - self.radius, cz),
            Vector3::new(0.0, -1.0, 0.0),
            Some((0.5, 0.0)),
        ));
        
        // Generate vertices for rings
        let mut ring_indices = Vec::new();
        for i in 0..self.rings - 1 {
            let phi = std::f32::consts::PI * (i as f32 + 1.0) / self.rings as f32;
            let y = cy + self.radius * phi.cos();
            let ring_radius = self.radius * phi.sin();
            
            let mut ring = Vec::new();
            for j in 0..self.segments {
                let theta = 2.0 * std::f32::consts::PI * j as f32 / self.segments as f32;
                let x = cx + ring_radius * theta.cos();
                let z = cz + ring_radius * theta.sin();
                
                let nx = theta.cos() * phi.sin();
                let ny = phi.cos();
                let nz = theta.sin() * phi.sin();
                
                let u = j as f32 / self.segments as f32;
                let v = 1.0 - (i as f32 + 1.0) / self.rings as f32;
                
                let idx = model.mesh.add_vertex(Vertex::new(
                    Point3::new(x, y, z),
                    Vector3::new(nx, ny, nz),
                    Some((u, v)),
                ));
                
                ring.push(idx);
            }
            
            ring_indices.push(ring);
        }
        
        // Create faces for the top cap
        let first_ring = &ring_indices[0];
        for i in 0..self.segments {
            let next_i = (i + 1) % self.segments;
            model.mesh.add_face(
                Face::triangle(top_idx, first_ring[i], first_ring[next_i]),
                None,
            );
        }
        
        // Create faces for the middle rings
        for i in 0..self.rings - 2 {
            let ring1 = &ring_indices[i];
            let ring2 = &ring_indices[i + 1];
            
            for j in 0..self.segments {
                let next_j = (j + 1) % self.segments;
                
                model.mesh.add_face(
                    Face::triangle(ring1[j], ring2[j], ring1[next_j]),
                    None,
                );
                model.mesh.add_face(
                    Face::triangle(ring1[next_j], ring2[j], ring2[next_j]),
                    None,
                );
            }
        }
        
        // Create faces for the bottom cap
        let last_ring = &ring_indices[self.rings - 2];
        for i in 0..self.segments {
            let next_i = (i + 1) % self.segments;
            model.mesh.add_face(
                Face::triangle(bottom_idx, last_ring[next_i], last_ring[i]),
                None,
            );
        }
        
        model
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating a cylinder primitive.
pub struct Cylinder {
    radius: f32,
    height: f32,
    center: (f32, f32, f32),
    segments: usize,
    caps: bool,
}

impl Cylinder {
    /// Create a new cylinder builder with default settings.
    pub fn new() -> Self {
        Self {
            radius: 1.0,
            height: 2.0,
            center: (0.0, 0.0, 0.0),
            segments: 32,
            caps: true,
        }
    }
    
    /// Set the radius of the cylinder.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
    
    /// Set the height of the cylinder.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    
    /// Set the center position of the cylinder.
    pub fn center(mut self, x: f32, y: f32, z: f32) -> Self {
        self.center = (x, y, z);
        self
    }
    
    /// Set the number of segments around the circumference.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }
    
    /// Set whether to generate end caps.
    pub fn caps(mut self, caps: bool) -> Self {
        self.caps = caps;
        self
    }
    
    /// Build the cylinder model.
    pub fn build(self) -> Model {
        let mut model = Model::new("Cylinder");
        let (cx, cy, cz) = self.center;
        let half_height = self.height / 2.0;
        
        // Generate vertices for top and bottom rings
        let mut top_indices = Vec::new();
        let mut bottom_indices = Vec::new();
        
        for i in 0..self.segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / self.segments as f32;
            let x = self.radius * theta.cos();
            let z = self.radius * theta.sin();
            
            // Normal for side pointing outward
            let nx = theta.cos();
            let nz = theta.sin();
            
            // Texture coordinates
            let u = i as f32 / self.segments as f32;
            
            // Top vertex
            let top_idx = model.mesh.add_vertex(Vertex::new(
                Point3::new(cx + x, cy + half_height, cz + z),
                Vector3::new(nx, 0.0, nz),
                Some((u, 1.0)),
            ));
            
            // Bottom vertex
            let bottom_idx = model.mesh.add_vertex(Vertex::new(
                Point3::new(cx + x, cy - half_height, cz + z),
                Vector3::new(nx, 0.0, nz),
                Some((u, 0.0)),
            ));
            
            top_indices.push(top_idx);
            bottom_indices.push(bottom_idx);
        }
        
        // Create side faces
        for i in 0..self.segments {
            let next_i = (i + 1) % self.segments;
            
            model.mesh.add_face(
                Face::triangle(bottom_indices[i], top_indices[i], top_indices[next_i]),
                None,
            );
            
            model.mesh.add_face(
                Face::triangle(bottom_indices[i], top_indices[next_i], bottom_indices[next_i]),
                None,
            );
        }
        
        // Create caps if requested
        if self.caps {
            // Center vertices for caps
            let top_center = model.mesh.add_vertex(Vertex::new(
                Point3::new(cx, cy + half_height, cz),
                Vector3::new(0.0, 1.0, 0.0),
                Some((0.5, 0.5)),
            ));
            
            let bottom_center = model.mesh.add_vertex(Vertex::new(
                Point3::new(cx, cy - half_height, cz),
                Vector3::new(0.0, -1.0, 0.0),
                Some((0.5, 0.5)),
            ));
            
            // Create top cap faces
            for i in 0..self.segments {
                let next_i = (i + 1) % self.segments;
                model.mesh.add_face(
                    Face::triangle(top_center, top_indices[i], top_indices[next_i]),
                    None,
                );
            }
            
            // Create bottom cap faces
            for i in 0..self.segments {
                let next_i = (i + 1) % self.segments;
                model.mesh.add_face(
                    Face::triangle(bottom_center, bottom_indices[next_i], bottom_indices[i]),
                    None,
                );
            }
        }
        
        model
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new()
    }
} 