use three_d::*;

use crate::clip_plane::{ClipPlane, MAX_CLIP_PLANES};

/// A material that renders per-vertex colored triangles (as produced by
/// [`crate::tet_to_mesh`]) and hides, in the fragment shader, any fragment
/// that lies on the positive side of any enabled [`ClipPlane`]. This is what
/// lets the mesh be cut open by dragging/rotating planes at runtime without
/// touching the underlying geometry.
#[derive(Clone, Default)]
pub struct ClippedColorMaterial {
    pub render_states: RenderStates,
    pub planes: Vec<ClipPlane>,
}

impl Material for ClippedColorMaterial {
    fn id(&self) -> EffectMaterialId {
        // 0x0000-0x4FFF is reserved for materials defined outside of three_d.
        EffectMaterialId(0x0001)
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        let mut shader = format!("#define MAX_CLIP_PLANES {}\n", MAX_CLIP_PLANES);
        shader.push_str(ColorMapping::fragment_shader_source());
        shader.push_str(include_str!("shaders/clipped_color_material.frag"));
        shader
    }

    fn use_uniforms(&self, program: &Program, viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        viewer.color_mapping().use_uniforms(program);

        let mut plane_data = [Vector4::new(0.0f32, 0.0, 0.0, 0.0); MAX_CLIP_PLANES];
        let mut count = 0usize;
        for plane in self.planes.iter().filter(|p| p.enabled) {
            if count >= MAX_CLIP_PLANES {
                break;
            }
            plane_data[count] = plane.as_vec4();
            count += 1;
        }
        program.use_uniform("numClipPlanes", count as i32);
        program.use_uniform_array("clipPlanes", &plane_data);
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
