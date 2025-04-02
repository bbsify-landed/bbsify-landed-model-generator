//! Primitive 3D shapes that can be used as building blocks.

use crate::{Face, Model, Vertex};
use nalgebra::{Point3, Vector3};

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
        assert!(size > 0.0, "Cube size must be positive");
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
        let positions = [
            // Front face - 0,1,2,3
            (cx - half, cy - half, cz + half),
            (cx + half, cy - half, cz + half),
            (cx + half, cy + half, cz + half),
            (cx - half, cy + half, cz + half),
            
            // Back face - 4,5,6,7
            (cx - half, cy - half, cz - half),
            (cx - half, cy + half, cz - half),
            (cx + half, cy + half, cz - half),
            (cx + half, cy - half, cz - half),
        ];
        
        // Define normals for each face
        let normals = [
            Vector3::new(0.0, 0.0, 1.0),   // Front
            Vector3::new(0.0, 0.0, -1.0),  // Back
            Vector3::new(0.0, 1.0, 0.0),   // Top
            Vector3::new(0.0, -1.0, 0.0),  // Bottom
            Vector3::new(1.0, 0.0, 0.0),   // Right
            Vector3::new(-1.0, 0.0, 0.0),  // Left
        ];
        
        // UVs for each vertex on each face
        let uvs = [
            (0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)
        ];
        
        // Create all the vertices with proper normals for each face
        // Front face (z+)
        let front_indices = [0, 1, 2, 3].map(|i| {
            model.mesh.add_vertex(Vertex::new(
                Point3::new(positions[i].0, positions[i].1, positions[i].2),
                normals[0],
                if self.with_uvs { Some(uvs[i]) } else { None },
            ))
        });
        
        // Back face (z-)
        let back_indices = [4, 5, 6, 7].map(|i| {
            model.mesh.add_vertex(Vertex::new(
                Point3::new(positions[i].0, positions[i].1, positions[i].2),
                normals[1],
                if self.with_uvs { Some(uvs[(i + 2) % 4]) } else { None },
            ))
        });
        
        // Top face (y+)
        let top_indices = [3, 2, 6, 5].map(|i| {
            model.mesh.add_vertex(Vertex::new(
                Point3::new(positions[i].0, positions[i].1, positions[i].2),
                normals[2],
                if self.with_uvs { Some(uvs[i % 4]) } else { None },
            ))
        });
        
        // Bottom face (y-)
        let bottom_indices = [0, 4, 7, 1].map(|i| {
            model.mesh.add_vertex(Vertex::new(
                Point3::new(positions[i].0, positions[i].1, positions[i].2),
                normals[3],
                if self.with_uvs { Some(uvs[i % 4]) } else { None },
            ))
        });
        
        // Right face (x+)
        let right_indices = [1, 7, 6, 2].map(|i| {
            model.mesh.add_vertex(Vertex::new(
                Point3::new(positions[i].0, positions[i].1, positions[i].2),
                normals[4],
                if self.with_uvs { Some(uvs[i % 4]) } else { None },
            ))
        });
        
        // Left face (x-)
        let left_indices = [0, 3, 5, 4].map(|i| {
            model.mesh.add_vertex(Vertex::new(
                Point3::new(positions[i].0, positions[i].1, positions[i].2),
                normals[5],
                if self.with_uvs { Some(uvs[i % 4]) } else { None },
            ))
        });
        
        // Define faces (6 faces of the cube, each as 2 triangles)
        // Front face
        model.mesh.add_face(Face::triangle(front_indices[0], front_indices[1], front_indices[2]), None);
        model.mesh.add_face(Face::triangle(front_indices[0], front_indices[2], front_indices[3]), None);
        
        // Back face
        model.mesh.add_face(Face::triangle(back_indices[0], back_indices[1], back_indices[2]), None);
        model.mesh.add_face(Face::triangle(back_indices[0], back_indices[2], back_indices[3]), None);
        
        // Top face
        model.mesh.add_face(Face::triangle(top_indices[0], top_indices[1], top_indices[2]), None);
        model.mesh.add_face(Face::triangle(top_indices[0], top_indices[2], top_indices[3]), None);
        
        // Bottom face
        model.mesh.add_face(Face::triangle(bottom_indices[0], bottom_indices[1], bottom_indices[2]), None);
        model.mesh.add_face(Face::triangle(bottom_indices[0], bottom_indices[2], bottom_indices[3]), None);
        
        // Right face
        model.mesh.add_face(Face::triangle(right_indices[0], right_indices[1], right_indices[2]), None);
        model.mesh.add_face(Face::triangle(right_indices[0], right_indices[2], right_indices[3]), None);
        
        // Left face
        model.mesh.add_face(Face::triangle(left_indices[0], left_indices[1], left_indices[2]), None);
        model.mesh.add_face(Face::triangle(left_indices[0], left_indices[2], left_indices[3]), None);
        
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
        assert!(radius > 0.0, "Sphere radius must be positive");
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
        assert!(segments >= 3, "Sphere must have at least 3 segments");
        self.segments = segments;
        self
    }

    /// Set the number of vertical rings (latitude).
    pub fn rings(mut self, rings: usize) -> Self {
        assert!(rings >= 2, "Sphere must have at least 2 rings");
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
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();
            
            let y = cy + self.radius * cos_phi;
            let ring_radius = self.radius * sin_phi;
            
            let mut ring = Vec::new();
            for j in 0..self.segments {
                let theta = 2.0 * std::f32::consts::PI * j as f32 / self.segments as f32;
                let cos_theta = theta.cos();
                let sin_theta = theta.sin();
                
                let x = cx + ring_radius * cos_theta;
                let z = cz + ring_radius * sin_theta;
                
                // Properly calculate normalized normal vector
                // For a sphere, the normal is simply the normalized direction from center to point
                let nx = sin_phi * cos_theta;
                let ny = cos_phi;
                let nz = sin_phi * sin_theta;
                
                // Better UV mapping with proper wrapping
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
        assert!(radius > 0.0, "Cylinder radius must be positive");
        self.radius = radius;
        self
    }

    /// Set the height of the cylinder.
    pub fn height(mut self, height: f32) -> Self {
        assert!(height > 0.0, "Cylinder height must be positive");
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
        assert!(segments >= 3, "Cylinder must have at least 3 segments");
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
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            let x = self.radius * cos_theta;
            let z = self.radius * sin_theta;

            // Normal for side pointing outward (already normalized since cos²+sin²=1)
            let nx = cos_theta;
            let nz = sin_theta;

            // Texture coordinates with proper wrapping
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
                Face::triangle(
                    bottom_indices[i],
                    top_indices[next_i],
                    bottom_indices[next_i],
                ),
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

            // Create vertices for cap UVs
            let mut top_cap_indices = Vec::new();
            let mut bottom_cap_indices = Vec::new();

            // Create specific vertices for caps with proper UV mapping
            for i in 0..self.segments {
                let theta = 2.0 * std::f32::consts::PI * i as f32 / self.segments as f32;
                let cos_theta = theta.cos();
                let sin_theta = theta.sin();

                let x = self.radius * cos_theta;
                let z = self.radius * sin_theta;

                // Calculate UV coordinates for caps (circular mapping)
                let u = 0.5 + 0.5 * cos_theta;
                let v = 0.5 + 0.5 * sin_theta;

                // Top cap vertex with proper normal and UV
                let top_cap_idx = model.mesh.add_vertex(Vertex::new(
                    Point3::new(cx + x, cy + half_height, cz + z),
                    Vector3::new(0.0, 1.0, 0.0),
                    Some((u, v)),
                ));

                // Bottom cap vertex with proper normal and UV
                let bottom_cap_idx = model.mesh.add_vertex(Vertex::new(
                    Point3::new(cx + x, cy - half_height, cz + z),
                    Vector3::new(0.0, -1.0, 0.0),
                    Some((u, 1.0 - v)),
                ));

                top_cap_indices.push(top_cap_idx);
                bottom_cap_indices.push(bottom_cap_idx);
            }

            // Create top cap faces
            for i in 0..self.segments {
                let next_i = (i + 1) % self.segments;
                model.mesh.add_face(
                    Face::triangle(top_center, top_cap_indices[i], top_cap_indices[next_i]),
                    None,
                );
            }

            // Create bottom cap faces
            for i in 0..self.segments {
                let next_i = (i + 1) % self.segments;
                model.mesh.add_face(
                    Face::triangle(
                        bottom_center,
                        bottom_cap_indices[next_i],
                        bottom_cap_indices[i],
                    ),
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
