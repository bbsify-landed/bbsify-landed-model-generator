//! Plugin system for extending model-generator functionality.

use crate::{Model, Result, Transform};
use std::sync::Arc;

/// A plugin that can modify a model.
pub trait Plugin: Send + Sync {
    /// Name of the plugin.
    fn name(&self) -> &str;

    /// Description of the plugin.
    fn description(&self) -> &str;

    /// Process a model using this plugin.
    fn process(&self, model: &mut Model) -> Result<()>;
}

/// Registry for managing plugins.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<Arc<dyn Plugin>>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Register a plugin with the registry.
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Arc::new(plugin));
    }

    /// Get a plugin by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Plugin>> {
        self.plugins.iter().find(|p| p.name() == name).cloned()
    }

    /// List all registered plugins.
    pub fn list(&self) -> Vec<(&str, &str)> {
        self.plugins
            .iter()
            .map(|p| (p.name(), p.description()))
            .collect()
    }
}

/// A transform plugin that wraps a Transform implementation.
pub struct TransformPlugin<T: Transform + Send + Sync + 'static> {
    name: String,
    description: String,
    transform: T,
}

impl<T: Transform + Send + Sync> TransformPlugin<T> {
    /// Create a new transform plugin.
    pub fn new(name: impl Into<String>, description: impl Into<String>, transform: T) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            transform,
        }
    }
}

impl<T: Transform + Send + Sync> Plugin for TransformPlugin<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn process(&self, model: &mut Model) -> Result<()> {
        self.transform.apply(model)
    }
}

/// A plugin that applies a sequence of transforms.
pub struct CompositePlugin {
    name: String,
    description: String,
    plugins: Vec<Arc<dyn Plugin>>,
}

impl CompositePlugin {
    /// Create a new composite plugin.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            plugins: Vec::new(),
        }
    }

    /// Add a plugin to the sequence.
    pub fn add<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Arc::new(plugin));
        self
    }

    /// Add an existing plugin to the sequence.
    pub fn add_existing(&mut self, plugin: Arc<dyn Plugin>) -> &mut Self {
        self.plugins.push(plugin);
        self
    }
}

impl Plugin for CompositePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn process(&self, model: &mut Model) -> Result<()> {
        for plugin in &self.plugins {
            plugin.process(model)?;
        }

        Ok(())
    }
}

/// A basic plugin for smoothing vertex normals.
pub struct SmoothNormalsPlugin {
    name: String,
    description: String,
}

impl SmoothNormalsPlugin {
    /// Create a new smooth normals plugin.
    pub fn new() -> Self {
        Self {
            name: "smooth_normals".to_string(),
            description: "Smooths vertex normals by averaging face normals".to_string(),
        }
    }
}

impl Plugin for SmoothNormalsPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn process(&self, model: &mut Model) -> Result<()> {
        model.mesh.compute_normals();
        Ok(())
    }
}

impl Default for SmoothNormalsPlugin {
    fn default() -> Self {
        Self::new()
    }
}
