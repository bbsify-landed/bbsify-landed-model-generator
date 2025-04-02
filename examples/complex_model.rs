//! A complex example demonstrating the plugin system and combining multiple primitives.

use model_generator::{Model, Result};
use model_generator::primitives::{Sphere, Cylinder};
use model_generator::transforms::{Scale, Rotate, Translate};
use model_generator::plugin::{Plugin, PluginRegistry, TransformPlugin, CompositePlugin, SmoothNormalsPlugin};
use model_generator::types::Material;

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
        // Create the three spheres for the body
        let bottom = Sphere::new()
            .radius(1.0)
            .center(0.0, 1.0, 0.0)
            .build();
        
        let middle = Sphere::new()
            .radius(0.7)
            .center(0.0, 2.4, 0.0)
            .build();
        
        let head = Sphere::new()
            .radius(0.5)
            .center(0.0, 3.3, 0.0)
            .build();
        
        // Create the nose (cone using a thin cylinder)
        let mut nose = Cylinder::new()
            .radius(0.1)
            .height(0.5)
            .segments(16)
            .build();
        
        nose.apply(Scale::new(1.0, 1.0, 1.0))
            .apply(Rotate::around_x(90.0))
            .apply(Translate::new(0.0, 3.3, 0.5));
        
        // Create coal eyes
        let left_eye = Sphere::new()
            .radius(0.07)
            .center(0.2, 3.5, 0.4)
            .segments(8)
            .rings(8)
            .build();
        
        let right_eye = Sphere::new()
            .radius(0.07)
            .center(-0.2, 3.5, 0.4)
            .segments(8)
            .rings(8)
            .build();
        
        // Create stick arms
        let mut left_arm = Cylinder::new()
            .radius(0.05)
            .height(0.8)
            .segments(8)
            .build();
        
        left_arm.apply(Rotate::around_z(45.0))
               .apply(Translate::new(0.9, 2.5, 0.0));
        
        let mut right_arm = Cylinder::new()
            .radius(0.05)
            .height(0.8)
            .segments(8)
            .build();
        
        right_arm.apply(Rotate::around_z(-45.0))
                .apply(Translate::new(-0.9, 2.5, 0.0));
        
        // Create hat (cylinder and disk)
        let mut hat_base = Cylinder::new()
            .radius(0.6)
            .height(0.1)
            .segments(32)
            .build();
        
        hat_base.apply(Translate::new(0.0, 3.7, 0.0));
        
        let mut hat_top = Cylinder::new()
            .radius(0.4)
            .height(0.4)
            .segments(32)
            .build();
        
        hat_top.apply(Translate::new(0.0, 4.0, 0.0));
        
        // Combine all parts into the model
        merge_model(model, &bottom)?;
        merge_model(model, &middle)?;
        merge_model(model, &head)?;
        merge_model(model, &nose)?;
        merge_model(model, &left_eye)?;
        merge_model(model, &right_eye)?;
        merge_model(model, &left_arm)?;
        merge_model(model, &right_arm)?;
        merge_model(model, &hat_base)?;
        merge_model(model, &hat_top)?;
        
        // Add a simple material for the snowman
        let snow_material = Material::new("snow");
        let coal_material = Material::new("coal");
        let carrot_material = Material::new("carrot");
        let wood_material = Material::new("wood");
        let hat_material = Material::new("hat");
        
        model.mesh.materials.insert("snow".to_string(), snow_material);
        model.mesh.materials.insert("coal".to_string(), coal_material);
        model.mesh.materials.insert("carrot".to_string(), carrot_material);
        model.mesh.materials.insert("wood".to_string(), wood_material);
        model.mesh.materials.insert("hat".to_string(), hat_material);
        
        Ok(())
    }
}

/// Helper function to merge one model into another.
fn merge_model(target: &mut Model, source: &Model) -> Result<()> {
    let offset = target.mesh.vertices.len();
    
    // Add all vertices from source
    for vertex in &source.mesh.vertices {
        target.mesh.add_vertex(vertex.clone());
    }
    
    // Add all faces from source, adjusting indices
    for (i, face) in source.mesh.faces.iter().enumerate() {
        let new_indices = face.indices
            .iter()
            .map(|&idx| idx + offset)
            .collect();
        
        let material = source.mesh.face_materials
            .get(i)
            .cloned()
            .flatten();
        
        target.mesh.add_face(
            model_generator::types::Face::new(new_indices),
            material,
        );
    }
    
    Ok(())
}

fn main() -> Result<()> {
    println!("Creating a complex model using plugins...");
    
    // Create an empty model
    let mut model = Model::new("ComplexModel");
    
    // Create a plugin registry and register plugins
    let mut registry = PluginRegistry::new();
    
    // Register the snowman plugin
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
    let mut rocket_plugin = CompositePlugin::new(
        "rocket", 
        "Creates a simple rocket from primitives"
    );
    
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
    model.export_obj("examples/output/complex_model.obj")?;
    println!("Exported as OBJ: examples/output/complex_model.obj");
    
    model.export_stl("examples/output/complex_model.stl")?;
    println!("Exported as STL: examples/output/complex_model.stl");
    
    model.export_gltf("examples/output/complex_model.gltf")?;
    println!("Exported as glTF: examples/output/complex_model.gltf");
    
    println!("Done!");
    Ok(())
} 