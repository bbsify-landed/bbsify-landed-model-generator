use mg::plugin::{CompositePlugin, Plugin, PluginRegistry, SmoothNormalsPlugin, TransformPlugin};
use mg::primitives::Cube;
use mg::{Model, Result};
use mg::{Rotate, Scale, Translate};

// A test plugin that inverts all vertex z coordinates
struct InvertZPlugin {
    name: String,
    description: String,
}

impl InvertZPlugin {
    fn new() -> Self {
        Self {
            name: "invert_z".to_string(),
            description: "Inverts all Z coordinates".to_string(),
        }
    }
}

impl Plugin for InvertZPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn process(&self, model: &mut Model) -> Result<()> {
        for vertex in &mut model.mesh.vertices {
            vertex.position.z = -vertex.position.z;
            vertex.normal.z = -vertex.normal.z;
        }

        Ok(())
    }
}

#[test]
fn test_plugin_registry() {
    let mut registry = PluginRegistry::new();

    // Register some plugins
    registry.register(SmoothNormalsPlugin::new());
    registry.register(InvertZPlugin::new());
    registry.register(TransformPlugin::new(
        "scale_double",
        "Doubles the size",
        Scale::uniform(2.0),
    ));

    // Test listing plugins
    let plugins = registry.list();
    assert_eq!(plugins.len(), 3);

    // Find registered plugins
    let smooth_plugin = registry.get("smooth_normals");
    assert!(smooth_plugin.is_some());

    let invert_plugin = registry.get("invert_z");
    assert!(invert_plugin.is_some());

    let scale_plugin = registry.get("scale_double");
    assert!(scale_plugin.is_some());

    // Test that nonexistent plugin returns None
    let nonexistent = registry.get("nonexistent");
    assert!(nonexistent.is_none());
}

#[test]
fn test_transform_plugin() {
    let mut model = Cube::new().build();
    let original_size = model.mesh.vertices[0].position.x.abs()
        + model.mesh.vertices[0].position.y.abs()
        + model.mesh.vertices[0].position.z.abs();

    // Create and apply a scale plugin
    let scale_plugin =
        TransformPlugin::new("scale_double", "Doubles the size", Scale::uniform(2.0));

    scale_plugin.process(&mut model).unwrap();

    // Verify the model was scaled
    let new_size = model.mesh.vertices[0].position.x.abs()
        + model.mesh.vertices[0].position.y.abs()
        + model.mesh.vertices[0].position.z.abs();

    assert!((new_size - 2.0 * original_size).abs() < 0.01);
}

#[test]
fn test_custom_plugin() {
    let mut model = Cube::new().build();

    // Save original z coordinates
    let original_z: Vec<f32> = model.mesh.vertices.iter().map(|v| v.position.z).collect();

    // Create and apply invert z plugin
    let invert_plugin = InvertZPlugin::new();
    invert_plugin.process(&mut model).unwrap();

    // Verify z coordinates were inverted
    for (i, vertex) in model.mesh.vertices.iter().enumerate() {
        assert!((vertex.position.z + original_z[i]).abs() < 0.01);
    }
}

#[test]
fn test_composite_plugin() {
    let mut model = Cube::new().build();

    // Get original vertex positions
    let original_positions: Vec<_> = model
        .mesh
        .vertices
        .iter()
        .map(|v| (v.position.x, v.position.y, v.position.z))
        .collect();

    // Create a composite plugin that:
    // 1. Scales the model by 2x
    // 2. Rotates it by 90 degrees around Y
    // 3. Translates it by (1,0,0)
    let mut composite = CompositePlugin::new(
        "transform_sequence",
        "Applies a sequence of transformations",
    );

    composite.add(TransformPlugin::new(
        "scale",
        "Scale by 2x",
        Scale::uniform(2.0),
    ));

    composite.add(TransformPlugin::new(
        "rotate",
        "Rotate 90 degrees around Y",
        Rotate::around_y(90.0),
    ));

    composite.add(TransformPlugin::new(
        "translate",
        "Translate by (1,0,0)",
        Translate::new(1.0, 0.0, 0.0),
    ));

    // Apply the composite plugin
    composite.process(&mut model).unwrap();

    // Verify transformations were applied in sequence
    // For a 90-degree Y rotation, x becomes z and z becomes -x
    for (i, vertex) in model.mesh.vertices.iter().enumerate() {
        let (orig_x, orig_y, orig_z) = original_positions[i];

        // Expected position after transformations:
        // Scale: (2*x, 2*y, 2*z)
        // Rotate: (2*z, 2*y, -2*x)
        // Translate: (2*z+1, 2*y, -2*x)
        let expected_x = 2.0 * orig_z + 1.0;
        let expected_y = 2.0 * orig_y;
        let expected_z = -2.0 * orig_x;

        assert!((vertex.position.x - expected_x).abs() < 0.01);
        assert!((vertex.position.y - expected_y).abs() < 0.01);
        assert!((vertex.position.z - expected_z).abs() < 0.01);
    }
}

#[test]
fn test_plugin_error_handling() {
    // Create a plugin that will fail with a specific error
    struct FailingPlugin;

    impl Plugin for FailingPlugin {
        fn name(&self) -> &str {
            "failing_plugin"
        }

        fn description(&self) -> &str {
            "A plugin that always fails"
        }

        fn process(&self, _model: &mut Model) -> Result<()> {
            Err(mg::Error::PluginError("Intentional failure".to_string()))
        }
    }

    let mut model = Cube::new().build();
    let plugin = FailingPlugin;

    // Apply the plugin and verify it returns an error
    let result = plugin.process(&mut model);
    assert!(result.is_err());

    // Verify error type
    match result {
        Err(mg::Error::PluginError(msg)) => {
            assert_eq!(msg, "Intentional failure");
        }
        _ => panic!("Expected PluginError, got different error or success"),
    }
}

#[test]
fn test_smooth_normals_plugin() {
    // Create a model with normals that need smoothing
    let mut model = Cube::new().build();

    // Zero out all normals
    for vertex in &mut model.mesh.vertices {
        vertex.normal.x = 0.0;
        vertex.normal.y = 0.0;
        vertex.normal.z = 0.0;
    }

    // Apply the smooth normals plugin
    let plugin = SmoothNormalsPlugin::new();
    plugin.process(&mut model).unwrap();

    // Verify normals were computed
    for vertex in &model.mesh.vertices {
        assert!(vertex.normal.magnitude() > 0.99);
    }
}
