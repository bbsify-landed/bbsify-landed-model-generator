//! Advanced transformations for 3D models.
//! 
//! This module contains more complex transformations like matrix transformations,
//! mirroring, and quaternion-based operations.

mod matrix;
mod mirror;
mod quaternion;

pub use matrix::Matrix;
pub use mirror::Mirror;
pub use quaternion::Quaternion; 