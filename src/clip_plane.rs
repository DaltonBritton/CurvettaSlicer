use three_d::{Vector3, Vector4};

/// Maximum number of clipping planes supported by [`crate::ClippedColorMaterial`].
/// Must match the `MAX_CLIP_PLANES` define baked into the fragment shader.
pub const MAX_CLIP_PLANES: usize = 8;

/// A single clipping plane that can be dragged along its normal and rotated.
/// Triangles on the positive side of the plane (in the direction the normal
/// points) are hidden by the shader.
#[derive(Clone, Copy, Debug)]
pub struct ClipPlane {
    pub enabled: bool,
    /// Rotation of the plane's normal around the Y axis, in degrees.
    pub azimuth_deg: f32,
    /// Rotation of the plane's normal away from the XZ plane, in degrees.
    pub elevation_deg: f32,
    /// Signed offset of the plane from the origin along its normal, i.e.
    /// dragging the plane back and forth along the normal direction.
    pub offset: f32,
}

impl Default for ClipPlane {
    fn default() -> Self {
        Self {
            enabled: true,
            azimuth_deg: 0.0,
            elevation_deg: 0.0,
            offset: 0.0,
        }
    }
}

impl ClipPlane {
    /// The unit normal of the plane, derived from the azimuth/elevation angles.
    pub fn normal(&self) -> Vector3<f32> {
        let az = self.azimuth_deg.to_radians();
        let el = self.elevation_deg.to_radians();
        Vector3::new(el.cos() * az.sin(), el.sin(), el.cos() * az.cos())
    }

    /// Packs the plane as `vec4(normal, offset)` for the shader. A world
    /// space point `p` is hidden when `dot(p, normal) > offset`.
    pub fn as_vec4(&self) -> Vector4<f32> {
        let n = self.normal();
        Vector4::new(n.x, n.y, n.z, self.offset)
    }
}
