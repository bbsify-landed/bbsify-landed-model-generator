use nalgebra::{Vector3, UnitQuaternion, Unit};
use std::f32::consts::PI;
use crate::{Model, Transform, Result};

/// Applies a quaternion-based rotation to a model.
/// 
/// Quaternions are particularly useful for avoiding gimbal lock and creating
/// smooth interpolation between rotations.
#[derive(Debug, Clone)]
pub struct Quaternion {
    quaternion: UnitQuaternion<f32>,
}

impl Quaternion {
    /// Create a new quaternion transformation from a unit quaternion.
    pub fn new(quaternion: UnitQuaternion<f32>) -> Self {
        Self { quaternion }
    }
    
    /// Create a quaternion rotation from axis-angle representation.
    pub fn from_axis_angle(axis: Vector3<f32>, angle_degrees: f32) -> Self {
        let unit_axis = Unit::new_normalize(axis);
        let angle_rad = angle_degrees * PI / 180.0;
        Self {
            quaternion: UnitQuaternion::from_axis_angle(&unit_axis, angle_rad),
        }
    }
    
    /// Create a quaternion from Euler angles (in degrees).
    pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> Self {
        let roll_rad = roll * PI / 180.0;
        let pitch_rad = pitch * PI / 180.0;
        let yaw_rad = yaw * PI / 180.0;
        
        Self {
            quaternion: UnitQuaternion::from_euler_angles(roll_rad, pitch_rad, yaw_rad),
        }
    }
    
    /// Create a quaternion that represents the shortest rotation from one direction to another.
    pub fn from_directions(from: Vector3<f32>, to: Vector3<f32>) -> Self {
        let from_unit = Unit::new_normalize(from);
        let to_unit = Unit::new_normalize(to);
        
        Self {
            quaternion: UnitQuaternion::rotation_between(&from_unit, &to_unit)
                .unwrap_or_else(|| UnitQuaternion::identity()),
        }
    }
}

impl Transform for Quaternion {
    fn apply(&self, model: &mut Model) -> Result<()> {
        for vertex in &mut model.mesh.vertices {
            // Rotate position
            let position = &mut vertex.position;
            let rotated_position = self.quaternion * Vector3::new(position.x, position.y, position.z);
            position.x = rotated_position.x;
            position.y = rotated_position.y;
            position.z = rotated_position.z;
            
            // Rotate normal
            vertex.normal = self.quaternion * vertex.normal;
        }
        
        Ok(())
    }
} 