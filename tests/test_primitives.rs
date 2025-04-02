use mg::primitives::{Cube, Cylinder, Sphere};

#[test]
fn test_cube_creation() {
    let cube = Cube::new().build();

    // A cube should have 8 vertices
    assert_eq!(cube.mesh.vertices.len(), 8);

    // A cube should have 12 triangular faces (6 quad faces, each split into 2 triangles)
    assert_eq!(cube.mesh.faces.len(), 12);

    // Test with custom parameters
    let custom_cube = Cube::new()
        .size(2.0)
        .center(1.0, 2.0, 3.0)
        .with_uvs(false)
        .build();

    assert_eq!(custom_cube.mesh.vertices.len(), 8);
    assert_eq!(custom_cube.mesh.faces.len(), 12);

    // Verify center position
    let sum_x: f32 = custom_cube.mesh.vertices.iter().map(|v| v.position.x).sum();
    let sum_y: f32 = custom_cube.mesh.vertices.iter().map(|v| v.position.y).sum();
    let sum_z: f32 = custom_cube.mesh.vertices.iter().map(|v| v.position.z).sum();

    assert!((sum_x / 8.0 - 1.0).abs() < 1e-5);
    assert!((sum_y / 8.0 - 2.0).abs() < 1e-5);
    assert!((sum_z / 8.0 - 3.0).abs() < 1e-5);

    // Verify no texture coordinates
    assert!(custom_cube
        .mesh
        .vertices
        .iter()
        .all(|v| v.tex_coords.is_none()));
}

#[test]
fn test_sphere_creation() {
    // Create a low-resolution sphere for testing
    let sphere = Sphere::new().segments(8).rings(4).build();

    // A sphere should have at least 2 + segments * (rings-1) vertices
    let expected_vertices = 2 + 8 * (4 - 1);
    assert_eq!(sphere.mesh.vertices.len(), expected_vertices);

    // There should be (segments * 2) + (segments * (rings-2) * 2) triangular faces
    let expected_faces = (8 * 2) + (8 * (4 - 2) * 2);
    assert_eq!(sphere.mesh.faces.len(), expected_faces);

    // Test with custom parameters
    let custom_sphere = Sphere::new()
        .radius(2.0)
        .center(1.0, 2.0, 3.0)
        .segments(12)
        .rings(6)
        .build();

    // Verify center position by checking that the average of all vertices is close to the center
    let avg_x: f32 = custom_sphere
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.x)
        .sum::<f32>()
        / custom_sphere.mesh.vertices.len() as f32;
    let avg_y: f32 = custom_sphere
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.y)
        .sum::<f32>()
        / custom_sphere.mesh.vertices.len() as f32;
    let avg_z: f32 = custom_sphere
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.z)
        .sum::<f32>()
        / custom_sphere.mesh.vertices.len() as f32;

    assert!((avg_x - 1.0).abs() < 0.1);
    assert!((avg_y - 2.0).abs() < 0.1);
    assert!((avg_z - 3.0).abs() < 0.1);

    // Verify radius by checking that all vertices are approximately radius distance from center
    for vertex in &custom_sphere.mesh.vertices {
        let dx = vertex.position.x - 1.0;
        let dy = vertex.position.y - 2.0;
        let dz = vertex.position.z - 3.0;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        assert!((distance - 2.0).abs() < 0.01);
    }
}

#[test]
fn test_cylinder_creation() {
    // Create a low-resolution cylinder for testing
    let cylinder = Cylinder::new().segments(8).build();

    // A cylinder with caps should have 2 + segments*2 vertices
    let expected_vertices = 2 + 8 * 2;
    assert_eq!(cylinder.mesh.vertices.len(), expected_vertices);

    // There should be segments*2 (sides) + segments*2 (caps) triangular faces
    let expected_faces = 8 * 2 + 8 * 2;
    assert_eq!(cylinder.mesh.faces.len(), expected_faces);

    // Test with custom parameters and no caps
    let custom_cylinder = Cylinder::new()
        .radius(2.0)
        .height(4.0)
        .center(1.0, 2.0, 3.0)
        .segments(12)
        .caps(false)
        .build();

    // A cylinder without caps should have segments*2 vertices
    assert_eq!(custom_cylinder.mesh.vertices.len(), 12 * 2);

    // There should be segments*2 triangular faces for the sides only
    assert_eq!(custom_cylinder.mesh.faces.len(), 12 * 2);

    // Verify height
    let max_y = custom_cylinder
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.y)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = custom_cylinder
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.y)
        .fold(f32::INFINITY, f32::min);
    assert!((max_y - min_y - 4.0).abs() < 0.01);
}
