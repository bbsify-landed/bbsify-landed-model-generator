//! Example demonstrating basic transforms: Scale, Translate, and Rotate.

use model_generator::primitives::Cube;
use model_generator::transforms::basic::{Scale, Translate, Rotate};
use nalgebra::Vector3;

fn main() -> model_generator::Result<()> {
    println!("Creating models with basic transformations...");
    
    // Create a basic cube
    let mut model = Cube::new()
        .size(1.0)
        .center(0.0, 0.0, 0.0)
        .build();
    
    // Apply uniform scaling
    model.apply(Scale::uniform(2.0));
    model.export_obj("examples/output/basic_scale_uniform.obj")?;
    println!("Exported uniform-scaled cube: examples/output/basic_scale_uniform.obj");
    
    // Apply non-uniform scaling
    let mut model = Cube::new().size(1.0).build();
    model.apply(Scale::new(1.0, 2.0, 0.5));
    model.export_obj("examples/output/basic_scale_nonuniform.obj")?;
    println!("Exported non-uniform-scaled cube: examples/output/basic_scale_nonuniform.obj");
    
    // Apply translation
    let mut model = Cube::new().size(1.0).build();
    model.apply(Translate::new(2.0, 1.0, -1.0));
    model.export_obj("examples/output/basic_translate.obj")?;
    println!("Exported translated cube: examples/output/basic_translate.obj");
    
    // Apply rotation around X axis
    let mut model = Cube::new().size(1.0).build();
    model.apply(Rotate::around_x(45.0));
    model.export_obj("examples/output/basic_rotate_x.obj")?;
    println!("Exported X-rotated cube: examples/output/basic_rotate_x.obj");
    
    // Apply rotation around Y axis
    let mut model = Cube::new().size(1.0).build();
    model.apply(Rotate::around_y(45.0));
    model.export_obj("examples/output/basic_rotate_y.obj")?;
    println!("Exported Y-rotated cube: examples/output/basic_rotate_y.obj");
    
    // Apply rotation around Z axis
    let mut model = Cube::new().size(1.0).build();
    model.apply(Rotate::around_z(45.0));
    model.export_obj("examples/output/basic_rotate_z.obj")?;
    println!("Exported Z-rotated cube: examples/output/basic_rotate_z.obj");
    
    // Apply rotation around custom axis
    let mut model = Cube::new().size(1.0).build();
    model.apply(Rotate::new(Vector3::new(1.0, 1.0, 1.0), 45.0));
    model.export_obj("examples/output/basic_rotate_custom.obj")?;
    println!("Exported custom-axis-rotated cube: examples/output/basic_rotate_custom.obj");
    
    // Apply a chain of basic transformations
    let mut model = Cube::new().size(1.0).build();
    model.apply(Scale::new(1.5, 0.5, 1.0))
         .apply(Rotate::around_y(30.0))
         .apply(Translate::new(0.0, 1.0, 0.0));
    model.export_obj("examples/output/basic_combined.obj")?;
    println!("Exported combined basic transforms: examples/output/basic_combined.obj");
    
    println!("Done with basic transformations!");
    Ok(())
} 