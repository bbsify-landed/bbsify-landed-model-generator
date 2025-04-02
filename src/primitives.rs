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
        let v0 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx - half, cy - half, cz + half),
            Vector3::new(0.0, 0.0, 0.0), // Normals will be computed later
            if self.with_uvs {
                Some((0.0, 0.0))
            } else {
                None
            },
        ));

        let v1 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx + half, cy - half, cz + half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((1.0, 0.0))
            } else {
                None
            },
        ));

        let v2 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx + half, cy + half, cz + half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((1.0, 1.0))
            } else {
                None
            },
        ));

        let v3 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx - half, cy + half, cz + half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((0.0, 1.0))
            } else {
                None
            },
        ));

        let v4 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx - half, cy - half, cz - half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((0.0, 0.0))
            } else {
                None
            },
        ));

        let v5 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx - half, cy + half, cz - half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((0.0, 1.0))
            } else {
                None
            },
        ));

        let v6 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx + half, cy + half, cz - half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((1.0, 1.0))
            } else {
                None
            },
        ));

        let v7 = model.mesh.add_vertex(Vertex::new(
            Point3::new(cx + half, cy - half, cz - half),
            Vector3::new(0.0, 0.0, 0.0),
            if self.with_uvs {
                Some((1.0, 0.0))
            } else {
                None
            },
        ));

        // Define faces (6 faces of the cube, each as 2 triangles)
        // Front face (z+)
        model.mesh.add_face(Face::triangle(v0, v1, v2), None);
        model.mesh.add_face(Face::triangle(v0, v2, v3), None);

        // Back face (z-)
        model.mesh.add_face(Face::triangle(v4, v5, v6), None);
        model.mesh.add_face(Face::triangle(v4, v6, v7), None);

        // Top face (y+)
        model.mesh.add_face(Face::triangle(v3, v2, v6), None);
        model.mesh.add_face(Face::triangle(v3, v6, v5), None);

        // Bottom face (y-)
        model.mesh.add_face(Face::triangle(v0, v4, v7), None);
        model.mesh.add_face(Face::triangle(v0, v7, v1), None);

        // Right face (x+)
        model.mesh.add_face(Face::triangle(v1, v7, v6), None);
        model.mesh.add_face(Face::triangle(v1, v6, v2), None);

        // Left face (x-)
        model.mesh.add_face(Face::triangle(v0, v3, v5), None);
        model.mesh.add_face(Face::triangle(v0, v5, v4), None);

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

                model
                    .mesh
                    .add_face(Face::triangle(ring1[j], ring2[j], ring1[next_j]), None);
                model
                    .mesh
                    .add_face(Face::triangle(ring1[next_j], ring2[j], ring2[next_j]), None);
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
