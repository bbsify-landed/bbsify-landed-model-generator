//! Core geometric types for the model-generator library.

use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

/// A 3D vertex with position, normal, and texture coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    /// 3D position
    pub position: Point3<f32>,
    /// Surface normal
    pub normal: Vector3<f32>,
    /// Texture coordinates (u, v)
    pub tex_coords: Option<(f32, f32)>,
}

impl Vertex {
    /// Create a new vertex with position, normal, and optional texture coordinates.
    pub fn new(
        position: Point3<f32>,
        normal: Vector3<f32>,
        tex_coords: Option<(f32, f32)>,
    ) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }

    /// Create a new vertex with only position data.
    pub fn with_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Point3::new(x, y, z),
            normal: Vector3::zeros(),
            tex_coords: None,
        }
    }
}

/// A face consisting of vertex indices.
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    /// Indices of vertices that form the face
    pub indices: Vec<usize>,
}

impl Face {
    /// Create a new face from vertex indices.
    pub fn new(indices: Vec<usize>) -> Self {
        Self { indices }
    }

    /// Create a triangle face.
    pub fn triangle(v1: usize, v2: usize, v3: usize) -> Self {
        Self {
            indices: vec![v1, v2, v3],
        }
    }

    /// Create a quad face.
    pub fn quad(v1: usize, v2: usize, v3: usize, v4: usize) -> Self {
        Self {
            indices: vec![v1, v2, v3, v4],
        }
    }
}

/// A mesh consisting of vertices and faces.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    /// Vertices in the mesh
    pub vertices: Vec<Vertex>,
    /// Faces in the mesh
    pub faces: Vec<Face>,
    /// Material properties
    pub materials: HashMap<String, Material>,
    /// Material assignments for faces
    pub face_materials: Vec<Option<String>>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    /// Create a new empty mesh.
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            materials: HashMap::new(),
            face_materials: Vec::new(),
        }
    }

    /// Add a vertex to the mesh.
    /// 
    /// Returns the index of the added vertex.
    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    /// Add a face to the mesh.
    /// 
    /// Returns the index of the added face.
    pub fn add_face(&mut self, face: Face, material_name: Option<String>) -> usize {
        let index = self.faces.len();
        self.faces.push(face);
        self.face_materials.push(material_name);
        index
    }

    /// Compute surface normals for vertices by averaging face normals.
    pub fn compute_normals(&mut self) {
        // Clear existing normals
        for vertex in &mut self.vertices {
            vertex.normal = Vector3::zeros();
        }

        // Compute face normals and add to vertex normals
        for face in &self.faces {
            if face.indices.len() < 3 {
                continue;
            }

            // Get the first three vertices to compute normal
            let v0 = self.vertices[face.indices[0]].position;
            let v1 = self.vertices[face.indices[1]].position;
            let v2 = self.vertices[face.indices[2]].position;

            // Compute face normal
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(&edge2).normalize();

            // Add the face normal to all vertices of this face
            for &idx in &face.indices {
                self.vertices[idx].normal += normal;
            }
        }

        // Normalize the accumulated vertex normals
        for vertex in &mut self.vertices {
            if vertex.normal.magnitude() > 0.0 {
                vertex.normal = vertex.normal.normalize();
            } else {
                // Default normal if we couldn't calculate one
                vertex.normal = Vector3::new(0.0, 1.0, 0.0);
            }
        }
    }
}

/// Material properties for a mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Material name
    pub name: String,
    /// Ambient color (RGBA)
    pub ambient: [f32; 4],
    /// Diffuse color (RGBA)
    pub diffuse: [f32; 4],
    /// Specular color (RGBA)
    pub specular: [f32; 4],
    /// Shininess
    pub shininess: f32,
    /// Texture file paths
    pub textures: HashMap<TextureType, String>,
}

impl Material {
    /// Create a new basic material.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ambient: [0.2, 0.2, 0.2, 1.0],
            diffuse: [0.8, 0.8, 0.8, 1.0],
            specular: [1.0, 1.0, 1.0, 1.0],
            shininess: 32.0,
            textures: HashMap::new(),
        }
    }
}

/// Types of textures that can be used in materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureType {
    /// Diffuse color map
    Diffuse,
    /// Normal map
    Normal,
    /// Specular map
    Specular,
    /// Roughness map
    Roughness,
    /// Metallic map
    Metallic,
    /// Emission map
    Emission,
    /// Occlusion map
    Occlusion,
} 