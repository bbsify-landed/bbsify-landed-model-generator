# Example demonstrating deform transforms: Twist, Bend, and Taper.

[Back to Examples Index](./README.md)

## Usage

```rust
let mut model = Cube::new().size(1.0).center(0.0, 0.0, 0.0).build();
model.apply(Twist::around_y(180.0, 0.0, 0.0));
model.export_obj("output/deform_twist_y.obj")?;
let mut model = Cylinder::new()
    .radius(0.5)
    .height(2.0)
    .center(0.0, 0.0, 0.0)
    .segments(36)
    .build();
model.apply(Twist::around_z(90.0, 0.0, 0.0));
model.export_obj("output/deform_twist_cylinder.obj")?;
let mut model = Cube::new().size(2.0).build();
model.apply(Twist::new(
    Vector3::new(1.0, 1.0, 0.0).normalize(), // Twist around diagonal axis
    Vector3::new(0.0, 0.0, 0.0),             // Center of twist
model.export_obj("output/deform_twist_custom.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Bend::x_axis(90.0, -1.0, 1.0));
model.export_obj("output/deform_bend_x.obj")?;
let mut model = Cylinder::new()
    .radius(0.25)
    .height(3.0)
    .segments(36)
    .build();
model.apply(Bend::z_axis(90.0, -1.5, 1.5));
model.export_obj("output/deform_bend_cylinder.obj")?;
let mut model = Cube::new().size(5.0).build();
model.apply(Bend::x_axis(90.0, 0.0, 2.5));
model.export_obj("output/deform_bend_partial.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Taper::y_axis(
    (1.0, 1.0),  // Start scale (x,z)
    (0.5, 0.5),  // End scale (x,z)
    (-1.0, 1.0), // Y range
model.export_obj("output/deform_taper_y.obj")?;
let mut model = Cylinder::new().radius(1.0).height(2.0).segments(36).build();
model.apply(Taper::z_axis(
    (1.0, 1.0),  // Start scale (x,y)
    (0.0, 0.0),  // End scale (x,y) - taper to a point
    (-1.0, 1.0), // Z range
model.export_obj("output/deform_taper_cone.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Taper::y_axis(
    (1.0, 1.0),  // Start scale (x,z)
    (0.1, 2.0),  // End scale - narrow in x, wide in z
    (-0.5, 0.5), // Y range
model.export_obj("output/deform_taper_nonuniform.obj")?;
let mut model = Cylinder::new().radius(0.5).height(3.0).segments(36).build();
model.apply(Taper::z_axis(
    (1.0, 1.0),  // Start scale
    (0.5, 0.5),  // End scale - taper to half size
    (-1.5, 1.5), // Z range
model.apply(Bend::x_axis(60.0, -1.5, 1.5));
model.apply(Twist::around_z(45.0, 0.0, 0.0));
model.export_obj("output/deform_combined.obj")?;
Ok(())
```

## Complete Source Code

```rust
//! Example demonstrating deform transforms: Twist, Bend, and Taper.

use mg::primitives::{Cube, Cylinder};
use mg::transforms::deform::{Bend, Taper, Twist};
use nalgebra::Vector3;

fn main() -> mg::Result<()> {
    println!("Creating models with deformation transformations...");

    // === Twist Transformations ===

    // Twist a cube around the Y axis
    let mut model = Cube::new().size(1.0).center(0.0, 0.0, 0.0).build();

    // Apply a 180 degree twist per unit along the Y axis
    model.apply(Twist::around_y(180.0, 0.0, 0.0));
    model.export_obj("output/deform_twist_y.obj")?;
    println!("Exported Y-twisted cube: output/deform_twist_y.obj");

    // Twist a cylinder around its main axis (Z)
    let mut model = Cylinder::new()
        .radius(0.5)
        .height(2.0)
        .center(0.0, 0.0, 0.0)
        .segments(36)
        .build();

    // Apply a 90 degree twist per unit along the Z axis
    model.apply(Twist::around_z(90.0, 0.0, 0.0));
    model.export_obj("output/deform_twist_cylinder.obj")?;
    println!("Exported twisted cylinder: output/deform_twist_cylinder.obj");

    // Create a more complex twisted shape using a custom axis
    let mut model = Cube::new().size(2.0).build();
    model.apply(Twist::new(
        Vector3::new(1.0, 1.0, 0.0).normalize(), // Twist around diagonal axis
        120.0,                                   // 120 degrees per unit
        Vector3::new(0.0, 0.0, 0.0),             // Center of twist
    ));
    model.export_obj("output/deform_twist_custom.obj")?;
    println!("Exported custom-twisted cube: output/deform_twist_custom.obj");

    // === Bend Transformations ===

    // Bend a cube around the X axis
    let mut model = Cube::new().size(1.0).build();

    // Apply a 90 degree bend around X axis along the Y range
    model.apply(Bend::x_axis(90.0, -1.0, 1.0));
    model.export_obj("output/deform_bend_x.obj")?;
    println!("Exported X-bent cube: output/deform_bend_x.obj");

    // Bend a cylinder around Z axis
    let mut model = Cylinder::new()
        .radius(0.25)
        .height(3.0)
        .segments(36)
        .build();

    // Apply a 90 degree bend around Z axis along the X range
    model.apply(Bend::z_axis(90.0, -1.5, 1.5));
    model.export_obj("output/deform_bend_cylinder.obj")?;
    println!("Exported bent cylinder: output/deform_bend_cylinder.obj");

    // Create a partial bend (only affects part of the model)
    let mut model = Cube::new().size(5.0).build();
    // Only bend the top half
    model.apply(Bend::x_axis(90.0, 0.0, 2.5));
    model.export_obj("output/deform_bend_partial.obj")?;
    println!("Exported partially-bent cube: output/deform_bend_partial.obj");

    // === Taper Transformations ===

    // Taper a cube along the Y axis
    let mut model = Cube::new().size(1.0).build();

    // Taper from full size at bottom to half size at top
    model.apply(Taper::y_axis(
        (1.0, 1.0),  // Start scale (x,z)
        (0.5, 0.5),  // End scale (x,z)
        (-1.0, 1.0), // Y range
    ));
    model.export_obj("output/deform_taper_y.obj")?;
    println!("Exported Y-tapered cube: output/deform_taper_y.obj");

    // Create a cone-like shape by tapering a cylinder to a point
    let mut model = Cylinder::new().radius(1.0).height(2.0).segments(36).build();

    // Taper from full size at bottom to zero at top
    model.apply(Taper::z_axis(
        (1.0, 1.0),  // Start scale (x,y)
        (0.0, 0.0),  // End scale (x,y) - taper to a point
        (-1.0, 1.0), // Z range
    ));
    model.export_obj("output/deform_taper_cone.obj")?;
    println!("Exported cone-tapered cylinder: output/deform_taper_cone.obj");

    // Create an interesting shape with non-uniform tapering
    let mut model = Cube::new().size(1.0).build();
    // Taper differently in x and z
    model.apply(Taper::y_axis(
        (1.0, 1.0),  // Start scale (x,z)
        (0.1, 2.0),  // End scale - narrow in x, wide in z
        (-0.5, 0.5), // Y range
    ));
    model.export_obj("output/deform_taper_nonuniform.obj")?;
    println!("Exported non-uniform-tapered cube: output/deform_taper_nonuniform.obj");

    // === Combining Deformations ===

    // Create a model with multiple deformations
    let mut model = Cylinder::new().radius(0.5).height(3.0).segments(36).build();

    // First taper it
    model.apply(Taper::z_axis(
        (1.0, 1.0),  // Start scale
        (0.5, 0.5),  // End scale - taper to half size
        (-1.5, 1.5), // Z range
    ));

    // Then bend it
    model.apply(Bend::x_axis(60.0, -1.5, 1.5));

    // Finally twist it
    model.apply(Twist::around_z(45.0, 0.0, 0.0));

    model.export_obj("output/deform_combined.obj")?;
    println!("Exported combined deformations: output/deform_combined.obj");

    println!("Done with deformation transformations!");
    Ok(())
}
```

---

Generated for model-generator library