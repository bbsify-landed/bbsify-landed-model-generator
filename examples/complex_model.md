# A complex example demonstrating the plugin system and combining multiple primitives.

[Back to Examples Index](./README.md)

## Usage

```rust
let mut model = Model::new("ComplexModel");
let mut registry = PluginRegistry::new();
registry.register(SnowmanPlugin::new());
registry.register(TransformPlugin::new(
    Scale::uniform(0.5),
registry.register(TransformPlugin::new(
    Translate::new(0.0, -1.0, 0.0),
    CompositePlugin::new("rocket", "Creates a simple rocket from primitives");
rocket_plugin.add(SmoothNormalsPlugin::new());
registry.register(rocket_plugin);
let snowman_plugin = registry.get("snowman").unwrap();
snowman_plugin.process(&mut model)?;
let scale_plugin = registry.get("scale_down").unwrap();
scale_plugin.process(&mut model)?;
for (name, desc) in registry.list() {
model.export_stl("examples/output/complex_model.stl")?;
model.export_gltf("examples/output/complex_model.gltf")?;
Ok(())
```

## Complete Source Code

```rust
//! A complex example demonstrating the plugin system and combining multiple primitives.

use mg::plugin::{CompositePlugin, Plugin, PluginRegistry, SmoothNormalsPlugin, TransformPlugin};
use mg::primitives::{Cylinder, Sphere};
use mg::types::Material;
use mg::{Model, Result, Rotate, Scale, Translate};

// Custom plugin that combines primitives to create a snowman
struct SnowmanPlugin {
    name: String,
    description: String,
}

impl SnowmanPlugin {
    fn new() -> Self {
        Self {
            name: "snowman".to_string(),
            description: "Creates a snowman from basic primitives".to_string(),
        }
    }
}

impl Plugin for SnowmanPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn process(&self, model: &mut Model) -> Result<()> {
        let mut snowman = Model::new("snowman");

        // Create snowman components
        let bottom = Sphere::new().radius(1.0).build();
        let middle = Sphere::new().radius(0.7).center(0.0, 2.4, 0.0).build();
        let nose = Cylinder::new().radius(0.1).height(0.5).segments(16).build();
        let mut left_arm = Cylinder::new().radius(0.05).height(0.8).segments(8).build();
        let mut right_arm = Cylinder::new().radius(0.05).height(0.8).segments(8).build();
        let mut hat_top = Cylinder::new().radius(0.4).height(0.4).segments(32).build();
        let mut hat_base = Cylinder::new().radius(0.05).height(0.8).segments(8).build();

        // Apply transformations
        left_arm
            .apply(Rotate::around_z(45.0))
            .apply(Translate::new(0.9, 2.5, 0.0));
        right_arm
            .apply(Rotate::around_z(-45.0))
            .apply(Translate::new(-0.9, 2.5, 0.0));
        hat_top.apply(Translate::new(0.0, 4.0, 0.0));
        hat_base.apply(Translate::new(0.0, 3.7, 0.0));

        // Add components to snowman model by merging meshes
        merge_model(&mut snowman, &bottom)?;
        merge_model(&mut snowman, &middle)?;
        merge_model(&mut snowman, &nose)?;
        merge_model(&mut snowman, &left_arm)?;
        merge_model(&mut snowman, &right_arm)?;
        merge_model(&mut snowman, &hat_top)?;
        merge_model(&mut snowman, &hat_base)?;

        // Merge snowman into the main model
        merge_model(model, &snowman)?;

        // Set materials
        let snow_material = Material::new("snow");
        let coal_material = Material::new("coal");
        let carrot_material = Material::new("carrot");
        let wood_material = Material::new("wood");
        let hat_material = Material::new("hat");

        model
            .mesh
            .materials
            .insert("snow".to_string(), snow_material);
        model
            .mesh
            .materials
            .insert("coal".to_string(), coal_material);
        model
            .mesh
            .materials
            .insert("carrot".to_string(), carrot_material);
        model
            .mesh
            .materials
            .insert("wood".to_string(), wood_material);
        model.mesh.materials.insert("hat".to_string(), hat_material);

        Ok(())
    }
}

/// Helper function to merge one model into another.
fn merge_model(target: &mut Model, source: &Model) -> Result<()> {
    let model_offset = target.mesh.vertices.len();

    // Add all vertices from source, adjusting indices
    for vertex in source.mesh.vertices.iter() {
        target.mesh.vertices.push(vertex.clone());
    }

    // Add all faces from source, adjusting indices
    for (i, face) in source.mesh.faces.iter().enumerate() {
        let new_indices = face.indices.iter().map(|&idx| idx + model_offset).collect();

        let material = source.mesh.face_materials.get(i).cloned().flatten();

        target
            .mesh
            .add_face(mg::types::Face::new(new_indices), material);
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("Creating a complex model.");

    let mut model = Model::new("ComplexModel");

    // Create a plugin registry and register the snowman plugin
    let mut registry = PluginRegistry::new();
    registry.register(SnowmanPlugin::new());

    // Register transform plugins
    registry.register(TransformPlugin::new(
        "scale_down",
        "Scales model down by 50%",
        Scale::uniform(0.5),
    ));

    registry.register(TransformPlugin::new(
        "ground",
        "Places the model on the xz plane",
        Translate::new(0.0, -1.0, 0.0),
    ));

    // Create a composite plugin
    let mut rocket_plugin =
        CompositePlugin::new("rocket", "Creates a simple rocket from primitives");

    // Add the plugins to create the rocket (simplified for example)
    rocket_plugin.add(SmoothNormalsPlugin::new());

    registry.register(rocket_plugin);

    // Use the snowman plugin to create our model
    let snowman_plugin = registry.get("snowman").unwrap();
    snowman_plugin.process(&mut model)?;

    // Use the scale_down plugin
    let scale_plugin = registry.get("scale_down").unwrap();
    scale_plugin.process(&mut model)?;

    // List all available plugins
    println!("Available plugins:");
    for (name, desc) in registry.list() {
        println!("  - {}: {}", name, desc);
    }

    // Export the model in multiple formats
    println!("Exported as OBJ: examples/output/complex_model.obj");
    model.export_stl("examples/output/complex_model.stl")?;
    println!("Exported as STL: examples/output/complex_model.stl");
    model.export_gltf("examples/output/complex_model.gltf")?;
    println!("Exported as glTF: examples/output/complex_model.gltf");

    println!("Done!");
    Ok(())
}
```

---

Generated for model-generator library