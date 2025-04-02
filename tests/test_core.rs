use model_generator::{Model, Vertex, Face, Mesh};
use nalgebra::{Point3, Vector3};

#[test]
fn test_create_empty_model() {
    let model = Model::new("TestModel");
    assert_eq!(model.name, "TestModel");
    assert!(model.mesh.vertices.is_empty());
    assert!(model.mesh.faces.is_empty());
}

#[test]
fn test_mesh_add_vertex() {
    let mut mesh = Mesh::new();
    let v0 = Vertex::with_position(0.0, 0.0, 0.0);
    let v1 = Vertex::with_position(1.0, 0.0, 0.0);
    let v2 = Vertex::with_position(1.0, 1.0, 0.0);
    
    let idx0 = mesh.add_vertex(v0);
    let idx1 = mesh.add_vertex(v1);
    let idx2 = mesh.add_vertex(v2);
    
    assert_eq!(idx0, 0);
    assert_eq!(idx1, 1);
    assert_eq!(idx2, 2);
    assert_eq!(mesh.vertices.len(), 3);
}

#[test]
fn test_mesh_add_face() {
    let mut mesh = Mesh::new();
    let v0 = Vertex::with_position(0.0, 0.0, 0.0);
    let v1 = Vertex::with_position(1.0, 0.0, 0.0);
    let v2 = Vertex::with_position(1.0, 1.0, 0.0);
    
    let idx0 = mesh.add_vertex(v0);
    let idx1 = mesh.add_vertex(v1);
    let idx2 = mesh.add_vertex(v2);
    
    let face = Face::triangle(idx0, idx1, idx2);
    let face_idx = mesh.add_face(face, Some("material1".to_string()));
    
    assert_eq!(face_idx, 0);
    assert_eq!(mesh.faces.len(), 1);
    assert_eq!(mesh.face_materials.len(), 1);
    assert_eq!(mesh.face_materials[0], Some("material1".to_string()));
}

#[test]
fn test_vertex_creation() {
    let position = Point3::new(1.0, 2.0, 3.0);
    let normal = Vector3::new(0.0, 1.0, 0.0);
    let tex_coords = Some((0.5, 0.5));
    
    let vertex = Vertex::new(position, normal, tex_coords);
    
    assert_eq!(vertex.position, position);
    assert_eq!(vertex.normal, normal);
    assert_eq!(vertex.tex_coords, tex_coords);
    
    let simple_vertex = Vertex::with_position(1.0, 2.0, 3.0);
    assert_eq!(simple_vertex.position, position);
    assert_eq!(simple_vertex.normal, Vector3::zeros());
    assert_eq!(simple_vertex.tex_coords, None);
}

#[test]
fn test_face_creation() {
    let triangle = Face::triangle(0, 1, 2);
    assert_eq!(triangle.indices, vec![0, 1, 2]);
    
    let quad = Face::quad(0, 1, 2, 3);
    assert_eq!(quad.indices, vec![0, 1, 2, 3]);
    
    let custom = Face::new(vec![0, 1, 2, 3, 4]);
    assert_eq!(custom.indices, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_compute_normals() {
    let mut mesh = Mesh::new();
    
    // Create a simple triangle
    let v0 = mesh.add_vertex(Vertex::with_position(0.0, 0.0, 0.0));
    let v1 = mesh.add_vertex(Vertex::with_position(1.0, 0.0, 0.0));
    let v2 = mesh.add_vertex(Vertex::with_position(0.0, 1.0, 0.0));
    
    mesh.add_face(Face::triangle(v0, v1, v2), None);
    
    // Initially normals are zero
    for vertex in &mesh.vertices {
        assert_eq!(vertex.normal, Vector3::zeros());
    }
    
    // Compute normals
    mesh.compute_normals();
    
    // The normal for this triangle should point in the positive Z direction
    for vertex in &mesh.vertices {
        assert!(vertex.normal.z > 0.0);
        assert_eq!(vertex.normal.magnitude(), 1.0);
    }
} 