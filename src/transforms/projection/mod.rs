//! Projection transformations for 3D models.
//!
//! This module contains transformations that project 3D models onto
//! different spaces or surfaces.

mod cylindrical;
mod orthographic;
mod perspective;

pub use cylindrical::Cylindrical;
pub use orthographic::Orthographic;
pub use perspective::Perspective;
