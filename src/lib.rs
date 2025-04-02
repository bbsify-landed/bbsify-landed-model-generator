//! A library for programmatically generating 3D models through transformations and plugins.
//! 
//! This crate provides tools for creating and manipulating 3D models with a focus on
//! composable transformations and an extensible plugin system.

use std::path::Path;
use thiserror::Error;

// Re-exports for convenience
pub use types::{Vertex, Face, Mesh};

// Module declarations
pub mod primitives;
pub mod transforms;
pub mod exporters;
pub mod types;
pub mod plugin;

/// Error types for the model-generator library.
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid model data: {0}")]
    InvalidModelData(String),
    
    #[error("Export error: {0}")]
    ExportError(String),
    
    #[error("Import error: {0}")]
    ImportError(String),
    
    #[error("Transform error: {0}")]
    TransformError(String),
    
    #[error("Plugin error: {0}")]
    PluginError(String),
}

/// Result type for operations in the model-generator library.
pub type Result<T> = std::result::Result<T, Error>;

/// The core data structure representing a 3D model.
#[derive(Debug, Clone)]
pub struct Model {
    /// The main mesh containing geometry data
    pub mesh: Mesh,
    /// Name of the model
    pub name: String,
}

impl Model {
    /// Create a new empty model.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            mesh: Mesh::new(),
            name: name.into(),
        }
    }
    
    /// Apply a transformation to this model.
    pub fn apply<T: Transform>(&mut self, transform: T) -> &mut Self {
        let _ = transform.apply(self);
        self
    }
    
    /// Export the model to OBJ format.
    pub fn export_obj<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        exporters::obj::export_obj(self, path)
    }
    
    /// Export the model to STL format.
    pub fn export_stl<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        exporters::stl::export_stl(self, path)
    }
    
    /// Export the model to glTF format.
    pub fn export_gltf<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        exporters::gltf::export_gltf(self, path)
    }
}

/// Trait for implementing transformations that can be applied to a model.
pub trait Transform {
    /// Apply the transformation to the given model.
    fn apply(&self, model: &mut Model) -> Result<()>;
} 