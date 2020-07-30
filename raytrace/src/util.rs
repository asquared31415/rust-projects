use crate::Vec3;
use rand::Rng;

// TAU IS UNSTABLE OR SOMETHING
// Note: check back in a few days it might be less unstable
// except actually it apparently misses the release cycle by a couple days
const TAU: f64 = 6.28318530717958647692528676655900577f64;

pub fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
    (1.0 - t) * a + t * b
}

pub fn sphere_to_cartesian(sphere: Vec3) -> Vec3 {
    Vec3(
        sphere.0 * sphere.1.sin() * sphere.2.cos(),
        sphere.0 * sphere.1.sin() * sphere.2.sin(),
        sphere.0 * sphere.1.cos()
    )
}

pub fn random_sphere_point(radius: f64) -> Vec3 {
    if radius.abs() <= 0.0000001 {
        Vec3(0.0, 0.0, 0.0)        
    } else {
        let mut rng = rand::thread_rng();
        sphere_to_cartesian(
            Vec3(
            rng.gen_range(0.0, radius),
            rng.gen_range(0.0, TAU),
            rng.gen_range(0.0, TAU/2.0)
            )
        )
    }
}

pub fn random_disk_vec(radius: f64) -> Vec3 {
    if radius.abs() <= 0.0000001 {
        Vec3(0.0, 0.0, 0.0)
    } else {
        let mut rng = rand::thread_rng();
        // polar coordinates in 2d act like spherical in 3d
        // setting the spherical theta component to Tau/4 brings the z component
        // to zero by disallowing any rotation within the XZ plane
        sphere_to_cartesian(Vec3(
            rng.gen_range(0.0, radius),
            TAU/4.0,
            rng.gen_range(0.0, TAU)
            )
        )
    }
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0, TAU);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z.powi(2)).sqrt();
    Vec3(r*a.cos(), r*a.sin(), z)
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vec3, normal: &Vec3, etai_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(&normal);
    let parallel = etai_etat * (uv + cos_theta * normal);
    let perp = -(1.0 - parallel.length_squared()).sqrt() * normal;
    parallel + perp
}

pub fn schlick(cos: f64, idx: f64) -> f64 {
    let r0 = ((1.0-idx) / (1.0+idx)).powi(2);
    r0 + (1.0-r0)*(1.0-cos).powi(5)
}