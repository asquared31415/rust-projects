use crate::{objects::HitRecord, vec::{Vec3, Ray, Color}, util::*};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {color}
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&'a self, _ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = hit.normal() + random_unit_vector();
        Some(
            (self.color.clone(), Ray::new(hit.point(), &scatter_direction))
        )
    }
}

pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 {fuzz} else {1.0};
        Self {color, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray.direction().normalize(), hit.normal());
        let scattered = Ray::new(hit.point(), &(&reflected + random_sphere_point(self.fuzz)));
        let attenuation = self.color.clone();
        if scattered.direction().dot(hit.normal()) > 0.0 {
            Some(
                (attenuation, scattered)
            )
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Self {
        Self {refraction_idx}
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let etai_etat = if hit.is_outside {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };
        let uv = &ray.direction().normalize();
        // Derived from trig definition of dot product and trig pythagorean identity
        let cos_theta = (-uv).dot(hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let reflect_prob = schlick(cos_theta, etai_etat);
        if etai_etat * sin_theta > 1.0 || rand::random::<f64>() < reflect_prob {
            let reflected = reflect(uv, hit.normal());
            Some((Vec3(1.0, 1.0, 1.0), Ray::new(hit.point(), &reflected)))
        } else {
            let refracted = refract(uv, hit.normal(), etai_etat);
            Some((Vec3(1.0, 1.0, 1.0), Ray::new(hit.point(), &refracted)))
        }
    }
}