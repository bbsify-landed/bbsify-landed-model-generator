//! glTF file format exporter.

use crate::{Model, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// Export a model to glTF format.
///
/// glTF (GL Transmission Format) is a modern, efficient 3D file format that is
/// widely supported by game engines, web viewers, and 3D applications like Blender.
/// This implementation creates a simple glTF 2.0 file with a binary buffer.
pub fn export_gltf<P: AsRef<Path>>(model: &Model, path: P) -> Result<()> {
    let path = path.as_ref();

    // Make sure the path has the correct extension
    let mut path_with_ext = PathBuf::from(path);
    if path_with_ext.extension().is_none_or(|ext| ext != "gltf") {
        path_with_ext.set_extension("gltf");
    }

    // Create the binary buffer file (.bin)
    let bin_path = path_with_ext.with_extension("bin");
    let bin_filename = bin_path.file_name().unwrap().to_string_lossy().to_string();

    // Export the binary buffer
    export_binary_buffer(model, &bin_path)?;

    // Create the JSON file (.gltf)
    let json_file = File::create(path_with_ext)?;
    let mut json_writer = BufWriter::new(json_file);

    // Get buffer size
    let buffer_size = calculate_buffer_size(model);

    // Write glTF JSON structure
    write!(
        json_writer,
        r#"{{
  "asset": {{
    "version": "2.0",
    "generator": "model-generator"
  }},
  "scene": 0,
  "scenes": [
    {{
      "nodes": [0]
    }}
  ],
  "nodes": [
    {{
      "mesh": 0,
      "name": "{}"
    }}
  ],
  "meshes": [
    {{
      "primitives": [
        {{
          "attributes": {{
            "POSITION": 0,
            "NORMAL": 1{}"
          }},
          "indices": 2,
          "mode": 4
        }}
      ]
    }}
  ],
  "accessors": [
    {{
      "bufferView": 0,
      "componentType": 5126,
      "count": {},
      "type": "VEC3",
      "min": [{}, {}, {}],
      "max": [{}, {}, {}]
    }},
    {{
      "bufferView": 1,
      "componentType": 5126,
      "count": {},
      "type": "VEC3"
    }},
    {{
      "bufferView": 2,
      "componentType": 5123,
      "count": {},
      "type": "SCALAR"
    }}{}
  ],
  "bufferViews": [
    {{
      "buffer": 0,
      "byteOffset": 0,
      "byteLength": {},
      "target": 34962
    }},
    {{
      "buffer": 0,
      "byteOffset": {},
      "byteLength": {},
      "target": 34962
    }},
    {{
      "buffer": 0,
      "byteOffset": {},
      "byteLength": {},
      "target": 34963
    }}{}
  ],
  "buffers": [
    {{
      "uri": "{}",
      "byteLength": {}
    }}
  ]
}}"#,
        // Node name
        model.name,
        // Add TEXCOORD_0 to attributes if model has texture coordinates
        if model.mesh.vertices.iter().any(|v| v.tex_coords.is_some()) {
            ",\n            \"TEXCOORD_0\": 3"
        } else {
            ""
        },
        // Position accessor
        model.mesh.vertices.len(),
        // Min bounds
        calculate_min_x(model),
        calculate_min_y(model),
        calculate_min_z(model),
        // Max bounds
        calculate_max_x(model),
        calculate_max_y(model),
        calculate_max_z(model),
        // Normal accessor count
        model.mesh.vertices.len(),
        // Index accessor count
        count_indices(model),
        // Add TEXCOORD_0 accessor if needed
        if model.mesh.vertices.iter().any(|v| v.tex_coords.is_some()) {
            format!(
                r#",
    {{
      "bufferView": 3,
      "componentType": 5126,
      "count": {},
      "type": "VEC2"
    }}"#,
                model.mesh.vertices.len()
            )
        } else {
            String::new()
        },
        // Position buffer view
        model.mesh.vertices.len() * 12, // 3 floats * 4 bytes
        // Normal buffer view
        model.mesh.vertices.len() * 12, // offset
        model.mesh.vertices.len() * 12, // 3 floats * 4 bytes
        // Index buffer view
        model.mesh.vertices.len() * 24, // offset
        count_indices(model) * 2,       // 1 unsigned short * 2 bytes
        // Add TEXCOORD_0 buffer view if needed
        if model.mesh.vertices.iter().any(|v| v.tex_coords.is_some()) {
            format!(
                r#",
    {{
      "buffer": 0,
      "byteOffset": {},
      "byteLength": {},
      "target": 34962
    }}"#,
                model.mesh.vertices.len() * 24 + count_indices(model) * 2, // offset
                model.mesh.vertices.len() * 8
            ) // 2 floats * 4 bytes
        } else {
            String::new()
        },
        // Buffer URI
        bin_filename,
        // Buffer size
        buffer_size
    )?;

    Ok(())
}

/// Calculate the total buffer size for the model.
fn calculate_buffer_size(model: &Model) -> usize {
    let position_size = model.mesh.vertices.len() * 12; // 3 floats * 4 bytes
    let normal_size = model.mesh.vertices.len() * 12; // 3 floats * 4 bytes
    let index_size = count_indices(model) * 2; // 1 unsigned short * 2 bytes

    let texcoord_size = if model.mesh.vertices.iter().any(|v| v.tex_coords.is_some()) {
        model.mesh.vertices.len() * 8 // 2 floats * 4 bytes
    } else {
        0
    };

    position_size + normal_size + index_size + texcoord_size
}

/// Count total indices in the model.
fn count_indices(model: &Model) -> usize {
    let mut count = 0;

    for face in &model.mesh.faces {
        match face.indices.len().cmp(&3) {
            std::cmp::Ordering::Less => {
                continue;
            }
            std::cmp::Ordering::Equal => {
                count += 3;
            }
            std::cmp::Ordering::Greater => {
                // Triangulate the face
                count += (face.indices.len() - 2) * 3;
            }
        }
    }

    count
}

/// Find the minimum X coordinate in the model.
fn calculate_min_x(model: &Model) -> f32 {
    model
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.x)
        .fold(f32::INFINITY, f32::min)
}

/// Find the minimum Y coordinate in the model.
fn calculate_min_y(model: &Model) -> f32 {
    model
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.y)
        .fold(f32::INFINITY, f32::min)
}

/// Find the minimum Z coordinate in the model.
fn calculate_min_z(model: &Model) -> f32 {
    model
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.z)
        .fold(f32::INFINITY, f32::min)
}

/// Find the maximum X coordinate in the model.
fn calculate_max_x(model: &Model) -> f32 {
    model
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.x)
        .fold(f32::NEG_INFINITY, f32::max)
}

/// Find the maximum Y coordinate in the model.
fn calculate_max_y(model: &Model) -> f32 {
    model
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.y)
        .fold(f32::NEG_INFINITY, f32::max)
}

/// Find the maximum Z coordinate in the model.
fn calculate_max_z(model: &Model) -> f32 {
    model
        .mesh
        .vertices
        .iter()
        .map(|v| v.position.z)
        .fold(f32::NEG_INFINITY, f32::max)
}

/// Export the binary buffer for glTF.
fn export_binary_buffer<P: AsRef<Path>>(model: &Model, path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write vertex positions
    for vertex in &model.mesh.vertices {
        writer.write_all(&vertex.position.x.to_le_bytes())?;
        writer.write_all(&vertex.position.y.to_le_bytes())?;
        writer.write_all(&vertex.position.z.to_le_bytes())?;
    }

    // Write vertex normals
    for vertex in &model.mesh.vertices {
        writer.write_all(&vertex.normal.x.to_le_bytes())?;
        writer.write_all(&vertex.normal.y.to_le_bytes())?;
        writer.write_all(&vertex.normal.z.to_le_bytes())?;
    }

    // Write indices
    let mut indices = Vec::new();

    for face in &model.mesh.faces {
        match face.indices.len().cmp(&3) {
            std::cmp::Ordering::Less => {
                continue;
            }
            std::cmp::Ordering::Equal => {
                // Simple triangle
                for &idx in &face.indices {
                    indices.push(idx as u16);
                }
            }
            std::cmp::Ordering::Greater => {
                // Triangulate the face
                let v0 = face.indices[0];
                for i in 1..face.indices.len() - 1 {
                    indices.push(v0 as u16);
                    indices.push(face.indices[i] as u16);
                    indices.push(face.indices[i + 1] as u16);
                }
            }
        }
    }

    for idx in indices {
        writer.write_all(&idx.to_le_bytes())?;
    }

    // Write texture coordinates if any
    if model.mesh.vertices.iter().any(|v| v.tex_coords.is_some()) {
        for vertex in &model.mesh.vertices {
            let (u, v) = vertex.tex_coords.unwrap_or((0.0, 0.0));
            writer.write_all(&u.to_le_bytes())?;
            writer.write_all(&v.to_le_bytes())?;
        }
    }

    Ok(())
}
