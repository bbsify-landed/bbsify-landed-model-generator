//! Example demonstrating projection transforms: Perspective, Orthographic, and Cylindrical.

use model_generator::primitives::{Cube, Sphere};
use model_generator::transforms::basic::Translate;
use model_generator::transforms::projection::{Cylindrical, Orthographic, Perspective};
use model_generator::{Face, Model, Vertex};
use nalgebra::{Point3, Vector3};

fn main() -> model_generator::Result<()> {
    println!("Creating models with projection transformations...");

    // === Perspective Projections ===

    // Create a cube for perspective projection
    let mut model = Cube::new().size(1.0).center(0.0, 0.0, 0.0).build();

    // Place the eye at (0,0,3) looking toward the origin
    // with focal length 1.0
    let eye = Point3::new(0.0, 0.0, 3.0);
    let perspective = Perspective::new(eye, 1.0, false);
    model.apply(perspective);
    model.export_obj("output/projection_perspective_basic.obj")?;
    println!("Exported basic perspective projection: output/projection_perspective_basic.obj");

    // Create a more interesting perspective scene with multiple objects
    let mut model = Cube::new().size(0.5).center(-0.5, 0.0, 0.0).build();
    let mut sphere = Sphere::new()
        .radius(0.25)
        .center(0.5, 0.0, 0.5)
        .segments(16)
        .rings(8)
        .build();

    // Add the sphere to the model
    for vertex in sphere.mesh.vertices.drain(..) {
        model.mesh.add_vertex(vertex);
    }
    for face in sphere.mesh.faces.drain(..) {
        model.mesh.add_face(face, None);
    }

    // Apply perspective projection from a different angle
    let eye = Point3::new(2.0, 1.5, 2.0);
    let perspective = Perspective::new(eye, 1.0, false);
    model.apply(perspective);
    model.export_obj("output/projection_perspective_complex.obj")?;
    println!("Exported complex perspective projection: output/projection_perspective_complex.obj");

    // === Orthographic Projections ===

    // Create a cube for orthographic projection onto XY plane
    let mut model = Cube::new().size(1.0).build();
    let ortho = Orthographic::onto_xy();
    model.apply(ortho);
    model.export_obj("output/projection_orthographic_xy.obj")?;
    println!("Exported XY orthographic projection: output/projection_orthographic_xy.obj");

    // Create a cube for orthographic projection onto XZ plane
    let mut model = Cube::new().size(1.0).build();
    let ortho = Orthographic::onto_xz();
    model.apply(ortho);
    model.export_obj("output/projection_orthographic_xz.obj")?;
    println!("Exported XZ orthographic projection: output/projection_orthographic_xz.obj");

    // Create a cube for orthographic projection onto YZ plane
    let mut model = Cube::new().size(1.0).build();
    let ortho = Orthographic::onto_yz();
    model.apply(ortho);
    model.export_obj("output/projection_orthographic_yz.obj")?;
    println!("Exported YZ orthographic projection: output/projection_orthographic_yz.obj");

    // Create a custom orthographic projection
    let mut model = Cube::new().size(1.0).build();
    // Project along the (1,1,1) direction (diagonal)
    let direction = Vector3::new(1.0, 1.0, 1.0);
    let ortho = Orthographic::new(direction, false);
    model.apply(ortho);
    model.export_obj("output/projection_orthographic_custom.obj")?;
    println!("Exported custom orthographic projection: output/projection_orthographic_custom.obj");

    // === Cylindrical Projections ===

    // Project a cube onto a cylinder along the Y axis
    let mut model = Cube::new().size(1.0).build();
    let cylindrical = Cylindrical::y_axis(0.0, 0.0, 1.0);
    model.apply(cylindrical);
    model.export_obj("output/projection_cylindrical_y.obj")?;
    println!("Exported Y-axis cylindrical projection: output/projection_cylindrical_y.obj");

    // Project a plane/grid onto a cylinder to create a tube
    // Create a manual grid since Grid isn't available
    let mut model = create_grid(2.0, 2.0, 20, 20);

    // Move the grid so it's in front of the cylinder
    model.apply(Translate::new(0.0, 0.0, 1.0));

    // Project onto a cylinder along the Y axis
    let cylindrical = Cylindrical::y_axis(0.0, 0.0, 1.0);
    model.apply(cylindrical);
    model.export_obj("output/projection_cylindrical_grid.obj")?;
    println!("Exported cylindrical grid projection: output/projection_cylindrical_grid.obj");

    // Create a sphere by projecting a grid onto a cylinder, then bending it
    let mut model = create_grid(3.14159 * 2.0, 3.14159, 36, 18);

    // Transform y coordinates to range from -PI/2 to PI/2
    for vertex in &mut model.mesh.vertices {
        vertex.position.y = vertex.position.y - 3.14159 / 2.0;
    }

    // Project onto a cylinder first
    let cylindrical_z = Cylindrical::z_axis(0.0, 0.0, 1.0);
    model.apply(cylindrical_z);

    // Now use another cylindrical projection along X to create a sphere-like shape
    let cylindrical_x = Cylindrical::new(
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        false,
    );
    model.apply(cylindrical_x);
    model.export_obj("output/projection_spherical.obj")?;
    println!("Exported spherical projection: output/projection_spherical.obj");

    println!("Done with projection transformations!");
    Ok(())
}

/// Create a grid model with specified width, height and subdivisions
fn create_grid(width: f32, height: f32, columns: usize, rows: usize) -> Model {
    let mut model = Model::new("Grid");

    // Calculate spacing
    let dx = width / columns as f32;
    let dy = height / rows as f32;

    // Calculate starting position (centered at origin)
    let start_x = -width / 2.0;
    let start_y = -height / 2.0;

    // Create vertices
    let mut vertices = Vec::new();
    for i in 0..=rows {
        for j in 0..=columns {
            let x = start_x + j as f32 * dx;
            let y = start_y + i as f32 * dy;

            // UV coordinates
            let u = j as f32 / columns as f32;
            let v = i as f32 / rows as f32;

            vertices.push(model.mesh.add_vertex(Vertex::new(
                Point3::new(x, y, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Some((u, v)),
            )));
        }
    }

    // Create faces (each cell becomes 2 triangles)
    for i in 0..rows {
        for j in 0..columns {
            // Get the four corners of the current grid cell
            let top_left = i * (columns + 1) + j;
            let top_right = top_left + 1;
            let bottom_left = (i + 1) * (columns + 1) + j;
            let bottom_right = bottom_left + 1;

            // Create two triangles
            model.mesh.add_face(
                Face::triangle(
                    vertices[top_left],
                    vertices[bottom_left],
                    vertices[top_right],
                ),
                None,
            );

            model.mesh.add_face(
                Face::triangle(
                    vertices[top_right],
                    vertices[bottom_left],
                    vertices[bottom_right],
                ),
                None,
            );
        }
    }

    model
}
