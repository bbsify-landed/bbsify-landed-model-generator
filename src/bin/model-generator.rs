use model_generator::{Model};
use model_generator::primitives::{Cube, Sphere, Cylinder};
use model_generator::transforms::{Scale, Rotate, Translate};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

fn main() {
    // Simple CLI argument parsing
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    match args[1].as_str() {
        "cube" => create_cube(&args[2..]),
        "sphere" => create_sphere(&args[2..]),
        "cylinder" => create_cylinder(&args[2..]),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown shape: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("3D Model Generator CLI");
    println!("Usage: model-generator SHAPE [OPTIONS] OUTPUT_FILE");
    println!();
    println!("Shapes:");
    println!("  cube      Generate a cube");
    println!("  sphere    Generate a sphere");
    println!("  cylinder  Generate a cylinder");
    println!();
    println!("Options for cube:");
    println!("  --size SIZE              Set cube size (default: 1.0)");
    println!("  --center X,Y,Z           Set center position (default: 0,0,0)");
    println!();
    println!("Options for sphere:");
    println!("  --radius RADIUS          Set radius (default: 1.0)");
    println!("  --segments SEGMENTS      Set number of segments (default: 32)");
    println!("  --rings RINGS            Set number of rings (default: 16)");
    println!("  --center X,Y,Z           Set center position (default: 0,0,0)");
    println!();
    println!("Options for cylinder:");
    println!("  --radius RADIUS          Set radius (default: 1.0)");
    println!("  --height HEIGHT          Set height (default: 2.0)");
    println!("  --segments SEGMENTS      Set number of segments (default: 32)");
    println!("  --center X,Y,Z           Set center position (default: 0,0,0)");
    println!("  --no-caps                Remove end caps");
    println!();
    println!("Common options:");
    println!("  --scale X,Y,Z            Apply scaling (default: 1,1,1)");
    println!("  --rotate AXIS,DEGREES    Apply rotation (e.g., y,45)");
    println!("  --translate X,Y,Z        Apply translation");
    println!();
    println!("Output formats are determined by file extension:");
    println!("  .obj     Wavefront OBJ format");
    println!("  .stl     STL format");
    println!("  .gltf    glTF format");
}

fn create_cube(args: &[String]) {
    let mut size = 1.0;
    let mut center = (0.0, 0.0, 0.0);
    let mut scale = None;
    let mut rotate = None;
    let mut translate = None;
    let mut output_file = None;
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--size" => {
                if i + 1 < args.len() {
                    size = args[i + 1].parse().unwrap_or(1.0);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--center" => {
                if i + 1 < args.len() {
                    center = parse_vector3(&args[i + 1]).unwrap_or((0.0, 0.0, 0.0));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--scale" => {
                if i + 1 < args.len() {
                    scale = Some(parse_vector3(&args[i + 1]).unwrap_or((1.0, 1.0, 1.0)));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--rotate" => {
                if i + 1 < args.len() {
                    rotate = parse_rotation(&args[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--translate" => {
                if i + 1 < args.len() {
                    translate = Some(parse_vector3(&args[i + 1]).unwrap_or((0.0, 0.0, 0.0)));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            _ => {
                output_file = Some(args[i].clone());
                i += 1;
            }
        }
    }
    
    // Create a cube with the specified parameters
    let mut cube = Cube::new()
        .size(size)
        .center(center.0, center.1, center.2)
        .build();
    
    // Apply transformations if specified
    if let Some(s) = scale {
        cube.apply(Scale::new(s.0, s.1, s.2));
    }
    
    if let Some((axis, angle)) = rotate {
        match axis.as_str() {
            "x" => cube.apply(Rotate::around_x(angle)),
            "y" => cube.apply(Rotate::around_y(angle)),
            "z" => cube.apply(Rotate::around_z(angle)),
            _ => &mut cube
        };
    }
    
    if let Some(t) = translate {
        cube.apply(Translate::new(t.0, t.1, t.2));
    }
    
    // Export the model to the specified file
    if let Some(file) = output_file {
        export_model(&cube, &file);
    } else {
        eprintln!("No output file specified");
        process::exit(1);
    }
}

fn create_sphere(args: &[String]) {
    let mut radius = 1.0;
    let mut segments = 32;
    let mut rings = 16;
    let mut center = (0.0, 0.0, 0.0);
    let mut scale = None;
    let mut rotate = None;
    let mut translate = None;
    let mut output_file = None;
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--radius" => {
                if i + 1 < args.len() {
                    radius = args[i + 1].parse().unwrap_or(1.0);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--segments" => {
                if i + 1 < args.len() {
                    segments = args[i + 1].parse().unwrap_or(32);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--rings" => {
                if i + 1 < args.len() {
                    rings = args[i + 1].parse().unwrap_or(16);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--center" => {
                if i + 1 < args.len() {
                    center = parse_vector3(&args[i + 1]).unwrap_or((0.0, 0.0, 0.0));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--scale" => {
                if i + 1 < args.len() {
                    scale = Some(parse_vector3(&args[i + 1]).unwrap_or((1.0, 1.0, 1.0)));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--rotate" => {
                if i + 1 < args.len() {
                    rotate = parse_rotation(&args[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--translate" => {
                if i + 1 < args.len() {
                    translate = Some(parse_vector3(&args[i + 1]).unwrap_or((0.0, 0.0, 0.0)));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            _ => {
                output_file = Some(args[i].clone());
                i += 1;
            }
        }
    }
    
    // Create a sphere with the specified parameters
    let mut sphere = Sphere::new()
        .radius(radius)
        .segments(segments)
        .rings(rings)
        .center(center.0, center.1, center.2)
        .build();
    
    // Apply transformations if specified
    if let Some(s) = scale {
        sphere.apply(Scale::new(s.0, s.1, s.2));
    }
    
    if let Some((axis, angle)) = rotate {
        match axis.as_str() {
            "x" => sphere.apply(Rotate::around_x(angle)),
            "y" => sphere.apply(Rotate::around_y(angle)),
            "z" => sphere.apply(Rotate::around_z(angle)),
            _ => &mut sphere
        };
    }
    
    if let Some(t) = translate {
        sphere.apply(Translate::new(t.0, t.1, t.2));
    }
    
    // Export the model to the specified file
    if let Some(file) = output_file {
        export_model(&sphere, &file);
    } else {
        eprintln!("No output file specified");
        process::exit(1);
    }
}

fn create_cylinder(args: &[String]) {
    let mut radius = 1.0;
    let mut height = 2.0;
    let mut segments = 32;
    let mut center = (0.0, 0.0, 0.0);
    let mut caps = true;
    let mut scale = None;
    let mut rotate = None;
    let mut translate = None;
    let mut output_file = None;
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--radius" => {
                if i + 1 < args.len() {
                    radius = args[i + 1].parse().unwrap_or(1.0);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--height" => {
                if i + 1 < args.len() {
                    height = args[i + 1].parse().unwrap_or(2.0);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--segments" => {
                if i + 1 < args.len() {
                    segments = args[i + 1].parse().unwrap_or(32);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--center" => {
                if i + 1 < args.len() {
                    center = parse_vector3(&args[i + 1]).unwrap_or((0.0, 0.0, 0.0));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--no-caps" => {
                caps = false;
                i += 1;
            },
            "--scale" => {
                if i + 1 < args.len() {
                    scale = Some(parse_vector3(&args[i + 1]).unwrap_or((1.0, 1.0, 1.0)));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--rotate" => {
                if i + 1 < args.len() {
                    rotate = parse_rotation(&args[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "--translate" => {
                if i + 1 < args.len() {
                    translate = Some(parse_vector3(&args[i + 1]).unwrap_or((0.0, 0.0, 0.0)));
                    i += 2;
                } else {
                    i += 1;
                }
            },
            _ => {
                output_file = Some(args[i].clone());
                i += 1;
            }
        }
    }
    
    // Create a cylinder with the specified parameters
    let mut cylinder = Cylinder::new()
        .radius(radius)
        .height(height)
        .segments(segments)
        .center(center.0, center.1, center.2)
        .caps(caps)
        .build();
    
    // Apply transformations if specified
    if let Some(s) = scale {
        cylinder.apply(Scale::new(s.0, s.1, s.2));
    }
    
    if let Some((axis, angle)) = rotate {
        match axis.as_str() {
            "x" => cylinder.apply(Rotate::around_x(angle)),
            "y" => cylinder.apply(Rotate::around_y(angle)),
            "z" => cylinder.apply(Rotate::around_z(angle)),
            _ => &mut cylinder
        };
    }
    
    if let Some(t) = translate {
        cylinder.apply(Translate::new(t.0, t.1, t.2));
    }
    
    // Export the model to the specified file
    if let Some(file) = output_file {
        export_model(&cylinder, &file);
    } else {
        eprintln!("No output file specified");
        process::exit(1);
    }
}

fn export_model(model: &Model, file: &str) {
    let path = PathBuf::from_str(file).unwrap();
    
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("obj") => {
            if let Err(e) = model.export_obj(&path) {
                eprintln!("Error exporting to OBJ: {}", e);
                process::exit(1);
            }
            println!("Model exported to {}", file);
        },
        Some("stl") => {
            if let Err(e) = model.export_stl(&path) {
                eprintln!("Error exporting to STL: {}", e);
                process::exit(1);
            }
            println!("Model exported to {}", file);
        },
        Some("gltf") => {
            if let Err(e) = model.export_gltf(&path) {
                eprintln!("Error exporting to glTF: {}", e);
                process::exit(1);
            }
            println!("Model exported to {}", file);
        },
        _ => {
            eprintln!("Unsupported file format: {}", file);
            eprintln!("Supported formats: .obj, .stl, .gltf");
            process::exit(1);
        }
    }
}

fn parse_vector3(s: &str) -> Option<(f32, f32, f32)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    
    let x = parts[0].parse::<f32>().ok()?;
    let y = parts[1].parse::<f32>().ok()?;
    let z = parts[2].parse::<f32>().ok()?;
    
    Some((x, y, z))
}

fn parse_rotation(s: &str) -> Option<(String, f32)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let axis = parts[0].to_lowercase();
    if !["x", "y", "z"].contains(&axis.as_str()) {
        return None;
    }
    
    let angle = parts[1].parse::<f32>().ok()?;
    
    Some((axis, angle))
} 