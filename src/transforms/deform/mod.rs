//! Deformation transformations for 3D models.
//!
//! This module contains transformations that change the shape of a model
//! in ways that aren't rigid body transformations.

mod bend;
mod taper;
mod twist;

pub use bend::Bend;
pub use taper::Taper;
pub use twist::Twist;
