use rand::{RngExt, rngs::ThreadRng};
use three_d::{Srgba, Vector3};

pub fn random_color(rng: &mut ThreadRng) -> Srgba {
    Srgba {
        r: rng.random_range(0..255),
        g: rng.random_range(0..255),
        b: rng.random_range(0..255),
        a: u8::MAX,
    }
}

pub fn remap_point_yz_axis<Precision>(point: Vector3<Precision>) -> Vector3<Precision> {
    Vector3 {
        x: point.x,
        y: point.z,
        z: point.y,
    }
}
