//! Projection transformations for 3D models.
//! 
//! This module contains transformations that project 3D models onto
//! different spaces or surfaces.

mod perspective;
mod orthographic;
mod cylindrical;

pub use perspective::Perspective;
pub use orthographic::Orthographic;
pub use cylindrical::Cylindrical; 