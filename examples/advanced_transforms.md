# Example demonstrating advanced transforms: Matrix, Mirror, and Quaternion.

[Back to Examples Index](./README.md)

## Usage

```rust
let mut model = Cube::new().size(1.0).center(0.0, 0.0, 0.0).build();
let skew_matrix = Matrix4::new(
model.apply(Matrix::new(skew_matrix));
model.export_obj("examples/output/advanced_matrix_skew.obj")?;
let mut model = Cube::new().size(1.0).build();
let cos = angle.cos();
let sin = angle.sin();
let complex_matrix = Matrix4::new(
model.apply(Matrix::new(complex_matrix));
model.export_obj("examples/output/advanced_matrix_combined.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Mirror::x());
model.export_obj("examples/output/advanced_mirror_x.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Mirror::y());
model.export_obj("examples/output/advanced_mirror_y.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Mirror::z());
model.export_obj("examples/output/advanced_mirror_z.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Mirror::new(true, true, false)); // Mirror across X and Y
model.export_obj("examples/output/advanced_mirror_xy.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Quaternion::from_axis_angle(
    Vector3::new(0.0, 1.0, 0.0),
model.export_obj("examples/output/advanced_quaternion_y45.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Quaternion::from_euler_angles(30.0, 45.0, 60.0));
model.export_obj("examples/output/advanced_quaternion_euler.obj")?;
let mut model = Cube::new().size(1.0).build();
model.apply(Quaternion::from_directions(
    Vector3::new(0.0, 0.0, 1.0), // Starting looking along Z
    Vector3::new(1.0, 1.0, 1.0), // Rotate to look along diagonal
model.export_obj("examples/output/advanced_quaternion_direction.obj")?;
let mut model = Cube::new().size(1.0).build();
let quat_z = UnitQuaternion::from_axis_angle(
    &nalgebra::Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
let quat_x = UnitQuaternion::from_axis_angle(
    &nalgebra::Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0)),
model.apply(Quaternion::new(combined_quat));
model.export_obj("examples/output/advanced_quaternion_combined.obj")?;
Ok(())
```

## Complete Source Code

```rust
//! Example demonstrating advanced transforms: Matrix, Mirror, and Quaternion.

use mg::primitives::Cube;
use mg::transforms::advanced::{Matrix, Mirror, Quaternion};
use nalgebra::{Matrix4, UnitQuaternion, Vector3};
use std::f32::consts::PI;

fn main() -> mg::Result<()> {
    println!("Creating models with advanced transformations...");

    // Create a basic cube
    let mut model = Cube::new().size(1.0).center(0.0, 0.0, 0.0).build();

    // Apply a custom transformation matrix (this one performs a skew)
    let skew_matrix = Matrix4::new(
        1.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    model.apply(Matrix::new(skew_matrix));
    model.export_obj("examples/output/advanced_matrix_skew.obj")?;
    println!("Exported matrix-skewed cube: examples/output/advanced_matrix_skew.obj");

    // Apply a more complex matrix that combines rotation, scale and translation
    let mut model = Cube::new().size(1.0).build();
    let angle = PI / 6.0; // 30 degrees
    let cos = angle.cos();
    let sin = angle.sin();
    let complex_matrix = Matrix4::new(
        1.5 * cos,
        0.0,
        1.5 * sin,
        1.0, // Scale 1.5 + rotate + translate x=1
        0.0,
        2.0,
        0.0,
        0.5, // Scale y by 2 + translate y=0.5
        -0.5 * sin,
        0.0,
        0.5 * cos,
        0.0, // Scale 0.5 + rotate
        0.0,
        0.0,
        0.0,
        1.0,
    );
    model.apply(Matrix::new(complex_matrix));
    model.export_obj("examples/output/advanced_matrix_combined.obj")?;
    println!("Exported matrix-combined cube: examples/output/advanced_matrix_combined.obj");

    // Apply mirror transformations

    // Mirror across YZ plane (flipping X)
    let mut model = Cube::new().size(1.0).build();
    model.apply(Mirror::x());
    model.export_obj("examples/output/advanced_mirror_x.obj")?;
    println!("Exported X-mirrored cube: examples/output/advanced_mirror_x.obj");

    // Mirror across XZ plane (flipping Y)
    let mut model = Cube::new().size(1.0).build();
    model.apply(Mirror::y());
    model.export_obj("examples/output/advanced_mirror_y.obj")?;
    println!("Exported Y-mirrored cube: examples/output/advanced_mirror_y.obj");

    // Mirror across XY plane (flipping Z)
    let mut model = Cube::new().size(1.0).build();
    model.apply(Mirror::z());
    model.export_obj("examples/output/advanced_mirror_z.obj")?;
    println!("Exported Z-mirrored cube: examples/output/advanced_mirror_z.obj");

    // Mirror across multiple planes
    let mut model = Cube::new().size(1.0).build();
    model.apply(Mirror::new(true, true, false)); // Mirror across X and Y
    model.export_obj("examples/output/advanced_mirror_xy.obj")?;
    println!("Exported XY-mirrored cube: examples/output/advanced_mirror_xy.obj");

    // Apply quaternion-based rotations

    // Rotate 45 degrees around Y axis using quaternion from axis-angle
    let mut model = Cube::new().size(1.0).build();
    model.apply(Quaternion::from_axis_angle(
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    ));
    model.export_obj("examples/output/advanced_quaternion_y45.obj")?;
    println!("Exported quaternion Y-rotated cube: examples/output/advanced_quaternion_y45.obj");

    // Rotate using Euler angles (30° roll, 45° pitch, 60° yaw)
    let mut model = Cube::new().size(1.0).build();
    model.apply(Quaternion::from_euler_angles(30.0, 45.0, 60.0));
    model.export_obj("examples/output/advanced_quaternion_euler.obj")?;
    println!(
        "Exported quaternion Euler-rotated cube: examples/output/advanced_quaternion_euler.obj"
    );

    // Rotate by finding shortest rotation from one direction to another
    let mut model = Cube::new().size(1.0).build();
    model.apply(Quaternion::from_directions(
        Vector3::new(0.0, 0.0, 1.0), // Starting looking along Z
        Vector3::new(1.0, 1.0, 1.0), // Rotate to look along diagonal
    ));
    model.export_obj("examples/output/advanced_quaternion_direction.obj")?;
    println!("Exported quaternion direction-rotated cube: examples/output/advanced_quaternion_direction.obj");

    // Build a custom, more complex quaternion
    let mut model = Cube::new().size(1.0).build();
    // Create a rotation of 90° around Z then 45° around X
    let quat_z = UnitQuaternion::from_axis_angle(
        &nalgebra::Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
        PI / 2.0,
    );
    let quat_x = UnitQuaternion::from_axis_angle(
        &nalgebra::Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0)),
        PI / 4.0,
    );
    // Combine the rotations (apply Z then X)
    let combined_quat = quat_x * quat_z;
    model.apply(Quaternion::new(combined_quat));
    model.export_obj("examples/output/advanced_quaternion_combined.obj")?;
    println!("Exported quaternion combined-rotated cube: examples/output/advanced_quaternion_combined.obj");

    println!("Done with advanced transformations!");
    Ok(())
}
```

---

Generated for model-generator library