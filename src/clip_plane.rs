use three_d::{Vector3, Vector4};

/// A single clipping plane that can be dragged along its normal and rotated.
/// Any tetrahedron with at least one corner on the positive side of the
/// plane (in the direction the normal points) is hidden entirely.
#[derive(Clone, Copy, Debug, PartialEq)]
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

/// Returns the vertex indices (as produced by [`crate::tet_to_mesh`]) of every
/// triangle belonging to a tetrahedron that is fully visible, i.e. none of
/// its 4 corners lies on the clipped side of any enabled plane. A tet with
/// even one clipped corner is left out entirely, hiding all 4 of its faces.
pub fn visible_triangle_indices(
    tet_corners: &[[Vector3<f32>; 4]],
    planes: &[ClipPlane],
) -> Vec<u32> {
    let active_planes: Vec<Vector4<f32>> = planes
        .iter()
        .filter(|p| p.enabled)
        .map(ClipPlane::as_vec4)
        .collect();

    let mut indices = Vec::with_capacity(tet_corners.len() * 12);
    for (tet_index, corners) in tet_corners.iter().enumerate() {
        let hidden = active_planes.iter().any(|plane| {
            corners
                .iter()
                .any(|c| c.x * plane.x + c.y * plane.y + c.z * plane.z > plane.w)
        });
        if !hidden {
            let base = (tet_index * 12) as u32;
            indices.extend(base..base + 12);
        }
    }
    indices
}
