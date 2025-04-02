# A basic example showing how to create a cube and export it in multiple formats.

[Back to Examples Index](./index.md)

## Usage

```rust
let mut model = Cube::new().size(2.0).center(0.0, 0.0, 0.0).build();
    .apply(Scale::new(1.5, 1.0, 0.8)) // Make it rectangular
    .apply(Rotate::around_y(45.0)) // Rotate 45 degrees around Y axis
    .apply(Translate::new(0.0, 1.0, 0.0)); // Move up 1 unit
model.export_obj("examples/output/basic_cube.obj")?;
model.export_stl("examples/output/basic_cube.stl")?;
model.export_gltf("examples/output/basic_cube.gltf")?;
Ok(())
```

## Complete Source Code

```rust
//! A basic example showing how to create a cube and export it in multiple formats.

use mg::primitives::Cube;
use mg::{Rotate, Scale, Translate};

fn main() -> mg::Result<()> {
    println!("Creating a basic cube model...");

    // Create a cube primitive
    let mut model = Cube::new().size(2.0).center(0.0, 0.0, 0.0).build();

    // Apply some transformations
    model
        .apply(Scale::new(1.5, 1.0, 0.8)) // Make it rectangular
        .apply(Rotate::around_y(45.0)) // Rotate 45 degrees around Y axis
        .apply(Translate::new(0.0, 1.0, 0.0)); // Move up 1 unit

    // Export the model in multiple formats
    model.export_obj("examples/output/basic_cube.obj")?;
    println!("Exported as OBJ: examples/output/basic_cube.obj");

    model.export_stl("examples/output/basic_cube.stl")?;
    println!("Exported as STL: examples/output/basic_cube.stl");

    model.export_gltf("examples/output/basic_cube.gltf")?;
    println!("Exported as glTF: examples/output/basic_cube.gltf");

    println!("Done!");
    Ok(())
}
```

---

Generated for model-generator library