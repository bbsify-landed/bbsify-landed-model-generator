//! Deformation transformations for 3D models.
//! 
//! This module contains transformations that change the shape of a model
//! in ways that aren't rigid body transformations.

mod twist;
mod bend;
mod taper;

pub use twist::Twist;
pub use bend::Bend;
pub use taper::Taper; 