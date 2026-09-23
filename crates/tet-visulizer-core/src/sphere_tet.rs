use three_d::*;

/// The four triangular faces of a tetrahedron, wound counter-clockwise when
/// viewed from outside, given as indices into a 4-vertex array.
const TET_FACES: [[usize; 3]; 4] = [[0, 1, 2], [0, 3, 1], [0, 2, 3], [1, 3, 2]];

/// Build a flat-shaded, per-face-colored `CpuMesh` for a single tetrahedron
/// defined by its four vertices.
fn _tetrahedron_mesh(v: [Vec3; 4]) -> CpuMesh {
    let face_colors = [
        Srgba::new(220, 60, 60, 255),
        Srgba::new(60, 180, 90, 255),
        Srgba::new(60, 100, 220, 255),
        Srgba::new(230, 200, 60, 255),
    ];

    let mut positions = Vec::with_capacity(12);
    let mut colors = Vec::with_capacity(12);
    for (face, color) in TET_FACES.iter().zip(face_colors.iter()) {
        for &i in face {
            positions.push(v[i]);
            colors.push(*color);
        }
    }

    CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    }
}

/// Split one triangle (assumed to have unit-length vertices) into four by
/// bisecting each edge and re-projecting the midpoints onto the unit sphere.
fn subdivide_triangle(t: [Vec3; 3]) -> [[Vec3; 3]; 4] {
    let [a, b, c] = t;
    let ab = ((a + b) * 0.5).normalize();
    let bc = ((b + c) * 0.5).normalize();
    let ca = ((c + a) * 0.5).normalize();
    [[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]]
}

/// A regular icosahedron's 20 faces, as unit-length vertex triples, wound
/// counter-clockwise when viewed from outside.
fn icosahedron_triangles() -> Vec<[Vec3; 3]> {
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let raw = [
        vec3(-1.0, t, 0.0),
        vec3(1.0, t, 0.0),
        vec3(-1.0, -t, 0.0),
        vec3(1.0, -t, 0.0),
        vec3(0.0, -1.0, t),
        vec3(0.0, 1.0, t),
        vec3(0.0, -1.0, -t),
        vec3(0.0, 1.0, -t),
        vec3(t, 0.0, -1.0),
        vec3(t, 0.0, 1.0),
        vec3(-t, 0.0, -1.0),
        vec3(-t, 0.0, 1.0),
    ];
    let verts: Vec<Vec3> = raw.iter().map(|v| v.normalize()).collect();
    let faces = [
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    faces
        .iter()
        .map(|f| [verts[f[0]], verts[f[1]], verts[f[2]]])
        .collect()
}

/// Triangulate a unit sphere by subdividing an icosahedron `subdivisions`
/// times, re-projecting new vertices onto the sphere at each step.
fn icosphere_triangles(subdivisions: u32) -> Vec<[Vec3; 3]> {
    let mut tris = icosahedron_triangles();
    for _ in 0..subdivisions {
        tris = tris.into_iter().flat_map(subdivide_triangle).collect();
    }
    tris
}

/// Map a unit normal to an RGB color, so neighboring tetrahedra on the
/// sphere's surface get visibly distinct but smoothly varying colors.
fn normal_to_color(n: Vec3) -> Srgba {
    let r = ((n.x * 0.5 + 0.5) * 255.0).round() as u8;
    let g = ((n.y * 0.5 + 0.5) * 255.0).round() as u8;
    let b = ((n.z * 0.5 + 0.5) * 255.0).round() as u8;
    Srgba::new(r, g, b, 255)
}

/// Build a `CpuMesh` approximating a solid sphere out of tetrahedra: each
/// triangle of a subdivided icosahedron is fanned into a tetrahedron with
/// the sphere's center, filling the ball from the inside out.
pub fn sphere_mesh(radius: f32, subdivisions: u32) -> CpuMesh {
    let triangles = icosphere_triangles(subdivisions);
    let mut positions = Vec::with_capacity(triangles.len() * 12);
    let mut colors = Vec::with_capacity(triangles.len() * 12);

    for [a, b, c] in triangles {
        let color = normal_to_color((a + b + c) / 3.0);
        let tet = [vec3(0.0, 0.0, 0.0), a * radius, b * radius, c * radius];
        for face in TET_FACES.iter() {
            for &i in face {
                positions.push(tet[i]);
                colors.push(color);
            }
        }
    }

    CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    }
}
