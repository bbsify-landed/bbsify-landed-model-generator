//! OBJ file format exporter.

use crate::{Model, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Export a model to OBJ format.
///
/// This exports the model as a Wavefront OBJ file, which is widely supported
/// by 3D applications including Blender, Maya, and most game engines.
pub fn export_obj<P: AsRef<Path>>(model: &Model, path: P) -> Result<()> {
    let file = File::create(path.as_ref())?;
    let mut writer = BufWriter::new(file);

    // Write header
    writeln!(writer, "# OBJ file generated by model-generator")?;
    writeln!(writer, "# Model name: {}", model.name)?;
    writeln!(writer)?;

    // Write material library reference if we have materials
    if !model.mesh.materials.is_empty() {
        let mtl_filename = format!(
            "{}.mtl",
            path.as_ref()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
        );
        writeln!(writer, "mtllib {}", mtl_filename)?;

        // Create the MTL file
        export_mtl(model, path.as_ref().with_file_name(&mtl_filename))?;
    }

    // Write vertex data
    for vertex in &model.mesh.vertices {
        writeln!(
            writer,
            "v {} {} {}",
            vertex.position.x, vertex.position.y, vertex.position.z
        )?;
    }

    // Write texture coordinates if any vertices have them
    let has_tex_coords = model.mesh.vertices.iter().any(|v| v.tex_coords.is_some());
    if has_tex_coords {
        for vertex in &model.mesh.vertices {
            let (u, v) = vertex.tex_coords.unwrap_or((0.0, 0.0));
            writeln!(writer, "vt {} {}", u, v)?;
        }
    }

    // Write normals
    for vertex in &model.mesh.vertices {
        writeln!(
            writer,
            "vn {} {} {}",
            vertex.normal.x, vertex.normal.y, vertex.normal.z
        )?;
    }

    // Group faces by material
    let mut current_material: Option<String> = None;

    for (face_idx, face) in model.mesh.faces.iter().enumerate() {
        // Check if we need to switch material
        let face_material = model.mesh.face_materials.get(face_idx).cloned().flatten();
        if face_material != current_material {
            if let Some(mat_name) = &face_material {
                writeln!(writer, "usemtl {}", mat_name)?;
            }
            current_material = face_material;
        }

        // Write face with vertex/texcoord/normal indices
        write!(writer, "f")?;
        for &vertex_idx in &face.indices {
            // OBJ is 1-indexed
            let v_idx = vertex_idx + 1;

            if has_tex_coords {
                // Format: v/vt/vn
                write!(writer, " {}/{}/{}", v_idx, v_idx, v_idx)?;
            } else {
                // Format: v//vn
                write!(writer, " {}//{}", v_idx, v_idx)?;
            }
        }
        writeln!(writer)?;
    }

    Ok(())
}

/// Export materials to MTL format.
fn export_mtl<P: AsRef<Path>>(model: &Model, path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write header
    writeln!(writer, "# MTL file generated by model-generator")?;
    writeln!(writer, "# Model name: {}", model.name)?;
    writeln!(writer)?;

    // Write each material
    for (name, material) in &model.mesh.materials {
        writeln!(writer, "newmtl {}", name)?;

        // Write ambient color
        writeln!(
            writer,
            "Ka {} {} {}",
            material.ambient[0], material.ambient[1], material.ambient[2]
        )?;

        // Write diffuse color
        writeln!(
            writer,
            "Kd {} {} {}",
            material.diffuse[0], material.diffuse[1], material.diffuse[2]
        )?;

        // Write specular color
        writeln!(
            writer,
            "Ks {} {} {}",
            material.specular[0], material.specular[1], material.specular[2]
        )?;

        // Write transparency (1 - alpha)
        writeln!(writer, "d {}", material.diffuse[3])?;

        // Write shininess (0-1000)
        writeln!(writer, "Ns {}", material.shininess)?;

        // Write illumination model (2 = highlight on)
        writeln!(writer, "illum 2")?;

        // Write texture maps
        use crate::types::TextureType;
        if let Some(diffuse_map) = material.textures.get(&TextureType::Diffuse) {
            writeln!(writer, "map_Kd {}", diffuse_map)?;
        }

        if let Some(normal_map) = material.textures.get(&TextureType::Normal) {
            writeln!(writer, "map_Bump {}", normal_map)?;
        }

        if let Some(specular_map) = material.textures.get(&TextureType::Specular) {
            writeln!(writer, "map_Ks {}", specular_map)?;
        }

        writeln!(writer)?;
    }

    Ok(())
}
