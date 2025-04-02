//! Example demonstrating transforms from all categories working together.

use model_generator::primitives::{Cube, Cylinder, Sphere};
use model_generator::transforms::advanced::{Mirror, Quaternion};
use model_generator::transforms::basic::{Rotate, Scale};
use model_generator::transforms::deform::{Bend, Taper, Twist};
use model_generator::transforms::projection::{Cylindrical, Perspective};
use nalgebra::{Point3, Vector3};
use std::f32::consts::PI;

fn main() -> model_generator::Result<()> {
    println!("Creating complex models using transforms from all categories...");

    // Model 1: Twisted and Bent Tower
    create_twisted_tower()?;

    // Model 2: Mirrored Sculpture
    create_mirrored_sculpture()?;

    // Model 3: Space Station
    create_space_station()?;

    println!("Done with all transform examples!");
    Ok(())
}

/// Creates a twisted and bent tower model using multiple transforms
fn create_twisted_tower() -> model_generator::Result<()> {
    // Start with a cylinder
    let mut model = Cylinder::new().radius(0.5).height(5.0).segments(48).build();

    // Create a tapered, twisted tower
    model.apply(Taper::z_axis(
        (1.0, 1.0),  // Start scale
        (0.4, 0.4),  // End scale
        (-2.5, 2.5), // Z range
    ));

    // Twist it around its main axis
    model.apply(Twist::around_z(45.0, 0.0, 0.0));

    // Bend it slightly
    model.apply(Bend::x_axis(15.0, -2.5, 2.5));

    // Add some cubes along its length for decoration
    for i in 0..5 {
        let height = -2.0 + i as f32;
        let size = 0.2;
        let radius = if i % 2 == 0 { 0.5 } else { 0.4 };

        // Create 4 small cubes at this height
        for j in 0..4 {
            let angle = j as f32 * PI / 2.0;
            let mut cube = Cube::new()
                .size(size)
                .center(radius * angle.cos(), radius * angle.sin(), height)
                .build();

            // Apply same deformations as the main tower
            cube.apply(Twist::around_z(45.0 * (height + 2.5) / 5.0, 0.0, 0.0));
            cube.apply(Bend::x_axis(15.0, -2.5, 2.5));

            // Add to the main model
            for vertex in &cube.mesh.vertices {
                model.mesh.add_vertex(vertex.clone());
            }
            let offset = model.mesh.vertices.len() - cube.mesh.vertices.len();
            for face in &cube.mesh.faces {
                let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
                model.mesh.add_face(
                    model_generator::Face {
                        indices: new_indices,
                    },
                    None,
                );
            }
        }
    }

    // Export the final model
    model.export_obj("output/all_twisted_tower.obj")?;
    println!("Exported twisted tower model: output/all_twisted_tower.obj");

    Ok(())
}

/// Creates a mirrored sculpture using advanced transforms
fn create_mirrored_sculpture() -> model_generator::Result<()> {
    // Start with a cube
    let mut model = Cube::new().size(0.5).center(0.5, 0.0, 0.0).build();

    // Add a sphere
    let sphere = Sphere::new()
        .radius(0.3)
        .center(0.0, 0.5, 0.0)
        .segments(24)
        .rings(12)
        .build();

    // Manual mesh merge
    let sphere_vertices_len = sphere.mesh.vertices.len();
    for vertex in &sphere.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - sphere_vertices_len;
    for face in &sphere.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Add a cylinder
    let cylinder = Cylinder::new()
        .radius(0.2)
        .height(0.8)
        .center(0.0, 0.0, 0.5)
        .segments(24)
        .build();

    // Manual mesh merge for cylinder
    let cylinder_vertices_len = cylinder.mesh.vertices.len();
    for vertex in &cylinder.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - cylinder_vertices_len;
    for face in &cylinder.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Apply quaternion rotation to make it more interesting
    model.apply(Quaternion::from_euler_angles(30.0, -15.0, 45.0));

    // Create mirrored copies of the model

    // Create XY mirrored copy (across Z)
    let mut mirror_z = model.clone();
    mirror_z.apply(Mirror::z());

    // Create YZ mirrored copy (across X)
    let mut mirror_x = model.clone();
    mirror_x.apply(Mirror::x());

    // Create XZ mirrored copy (across Y)
    let mut mirror_y = model.clone();
    mirror_y.apply(Mirror::y());

    // Create a copy mirrored across all axes
    let mut mirror_xyz = model.clone();
    mirror_xyz.apply(Mirror::new(true, true, true));

    // Add all mirrored copies to the original model using our manual merge function
    // Mirror Z
    let mirror_z_vertices_len = mirror_z.mesh.vertices.len();
    for vertex in &mirror_z.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - mirror_z_vertices_len;
    for face in &mirror_z.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Mirror X
    let mirror_x_vertices_len = mirror_x.mesh.vertices.len();
    for vertex in &mirror_x.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - mirror_x_vertices_len;
    for face in &mirror_x.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Mirror Y
    let mirror_y_vertices_len = mirror_y.mesh.vertices.len();
    for vertex in &mirror_y.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - mirror_y_vertices_len;
    for face in &mirror_y.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Mirror XYZ
    let mirror_xyz_vertices_len = mirror_xyz.mesh.vertices.len();
    for vertex in &mirror_xyz.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - mirror_xyz_vertices_len;
    for face in &mirror_xyz.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Scale the whole sculpture up
    model.apply(Scale::uniform(1.5));

    // Export the final model
    model.export_obj("output/all_mirrored_sculpture.obj")?;
    println!("Exported mirrored sculpture model: output/all_mirrored_sculpture.obj");

    Ok(())
}

/// Creates a space station model using various transforms
fn create_space_station() -> model_generator::Result<()> {
    // Create the central hub (a sphere)
    let mut model = Sphere::new()
        .radius(1.0)
        .center(0.0, 0.0, 0.0)
        .segments(32)
        .rings(16)
        .build();

    // Create a cylindrical projection of a small portion of the sphere
    // to create docking port
    let mut docking_port = model.clone();

    // Keep only vertices near the positive X axis
    docking_port
        .mesh
        .vertices
        .retain(|v| v.position.x > 0.7 && v.position.y.abs() < 0.5 && v.position.z.abs() < 0.5);

    // Rebuild faces (removing any faces that reference missing vertices)
    let valid_indices: Vec<_> = docking_port
        .mesh
        .vertices
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect();
    docking_port
        .mesh
        .faces
        .retain(|face| face.indices.iter().all(|idx| valid_indices.contains(idx)));

    // Project outward to make a cylinder
    let cyl_projection = Cylindrical::new(
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        1.5,
        false,
    );
    docking_port.apply(cyl_projection);

    // Add the docking port to the main model
    let docking_port_vertices_len = docking_port.mesh.vertices.len();
    for vertex in &docking_port.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - docking_port_vertices_len;
    for face in &docking_port.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Create rings around the station

    // Ring 1 (horizontal)
    let mut ring1 = Cylinder::new()
        .radius(2.5)
        .height(0.3)
        .center(0.0, 0.0, 0.0)
        .segments(64)
        .build();

    // Apply a taper to make it thinner
    ring1.apply(Taper::new(
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.7, 0.7, 1.0),
        (-0.15, 0.15),
    ));

    // Ring 2 (vertical)
    let mut ring2 = ring1.clone();
    ring2.apply(Rotate::around_x(90.0));

    // Add the rings to the main model
    // Ring 1
    let ring1_vertices_len = ring1.mesh.vertices.len();
    for vertex in &ring1.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - ring1_vertices_len;
    for face in &ring1.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Ring 2
    let ring2_vertices_len = ring2.mesh.vertices.len();
    for vertex in &ring2.mesh.vertices {
        model.mesh.add_vertex(vertex.clone());
    }
    let offset = model.mesh.vertices.len() - ring2_vertices_len;
    for face in &ring2.mesh.faces {
        let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
        model.mesh.add_face(
            model_generator::Face {
                indices: new_indices,
            },
            None,
        );
    }

    // Create struts connecting the rings to the central hub
    for i in 0..8 {
        let angle = i as f32 * PI / 4.0;

        // Horizontal ring struts
        let mut strut_h = Cylinder::new()
            .radius(0.1)
            .height(1.5)
            .center(0.75 * angle.cos(), 0.75 * angle.sin(), 0.0)
            .segments(12)
            .build();

        // Rotate to point outward
        let rot_quat = Quaternion::from_directions(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(angle.cos(), angle.sin(), 0.0),
        );
        strut_h.apply(rot_quat);

        // Vertical ring struts
        let mut strut_v = Cylinder::new()
            .radius(0.1)
            .height(1.5)
            .center(0.75 * angle.cos(), 0.0, 0.75 * angle.sin())
            .segments(12)
            .build();

        // Rotate to point outward
        let rot_quat = Quaternion::from_directions(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(angle.cos(), 0.0, angle.sin()),
        );
        strut_v.apply(rot_quat);

        // Add to the main model
        // Horizontal strut
        let strut_h_vertices_len = strut_h.mesh.vertices.len();
        for vertex in &strut_h.mesh.vertices {
            model.mesh.add_vertex(vertex.clone());
        }
        let offset = model.mesh.vertices.len() - strut_h_vertices_len;
        for face in &strut_h.mesh.faces {
            let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
            model.mesh.add_face(
                model_generator::Face {
                    indices: new_indices,
                },
                None,
            );
        }

        // Vertical strut
        let strut_v_vertices_len = strut_v.mesh.vertices.len();
        for vertex in &strut_v.mesh.vertices {
            model.mesh.add_vertex(vertex.clone());
        }
        let offset = model.mesh.vertices.len() - strut_v_vertices_len;
        for face in &strut_v.mesh.faces {
            let new_indices = face.indices.iter().map(|&idx| idx + offset).collect();
            model.mesh.add_face(
                model_generator::Face {
                    indices: new_indices,
                },
                None,
            );
        }
    }

    // Export the final model
    model.export_obj("output/all_space_station.obj")?;
    println!("Exported space station model: output/all_space_station.obj");

    // Export a perspective view of the space station
    let mut perspective_view = model.clone();
    let eye = Point3::new(6.0, 5.0, 4.0);
    perspective_view.apply(Perspective::new(eye, 2.0, false));
    perspective_view.export_obj("output/all_space_station_perspective.obj")?;
    println!("Exported space station perspective: output/all_space_station_perspective.obj");

    Ok(())
}
