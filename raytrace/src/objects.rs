use crate::{Ray, Vec3, materials::Material, vec::Point};
use std::sync::Arc;

pub struct HitRecord {
    point: Vec3,
    pub t: f64,
    normal: Vec3,
    pub is_outside: bool,
    material: Arc<dyn Material + Send + Sync>
}

impl HitRecord {
    pub fn point<'a>(&'a self) -> &'a Point {
        &self.point
    }

    pub fn normal<'a>(&'a self) -> &'a Vec3 {
        &self.normal
    }

    pub fn material<'a>(&'a self) -> &'a Arc<dyn Material + Send + Sync> {
        &self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

pub struct Hittables {
    items: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }

    pub fn push(&mut self, item: Box<dyn Hittable + Send + Sync>) {
        self.items.push(item);
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = tmax;

        for item in self.items.iter() {
            if let Some(hit) = item.hit(ray, tmin, closest) {
                closest = hit.t;
                record = Some(hit);
            }
        }

        record
    }

    pub fn pop(&mut self) -> Option<Box<dyn Hittable + Send + Sync>> {
        self.items.pop()
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if tmax < tmin { return None }

        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let disc = half_b.powi(2) - a*c;

        if disc < 0.0 {
            None
        } else {
            let root = disc.sqrt();
            let t = (-half_b - root) / a;
            if tmin < t && t < tmax {
                let point= ray.at(t);
                let outward_normal = (&point - &self.center) / self.radius;
                let is_outside = ray.direction().dot(&outward_normal) < 0.0;
                Some(HitRecord {
                    point: point.clone(),
                    t,
                    normal: if is_outside {outward_normal} else {-outward_normal},
                    is_outside,
                    material: Arc::clone(&self.material)
                })
            } else {
                let t = (-half_b + root) / a;
                if tmin < t && t < tmax {
                    let point= ray.at(t);
                    let outward_normal = (&point - &self.center) / self.radius;
                    let is_outside = ray.direction().dot(&outward_normal) < 0.0;
                    Some(HitRecord {
                        point: point.clone(),
                        t,
                        normal: if is_outside {outward_normal} else {-outward_normal},
                        is_outside,
                        material: Arc::clone(&self.material)
                    })
                } else {
                    None
                }
            }

        }
    }
}

pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    normal: Vec3,
    material: Arc<dyn Material + Send + Sync>,
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        assert_ne!(p1, p2, "Points on a triangle must be unique");
        assert_ne!(p2, p3, "Points on a triangle must be unique");
        assert_ne!(p3, p1, "Points on a triangle must be unique");
        let normal = (&p2 - &p1).cross(&(&p3 - &p1)).normalize();
        assert_ne!(normal.length_squared(), 0.0, "Points on a triangle must not be colinear");
        Self {
            p1,
            p2,
            p3,
            normal,
            material
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction());
        if denom.abs() > 0.0000001 {
            let d = self.normal.dot(&self.p1);
            let t = (d - self.normal.dot(&ray.origin())) / denom;
            if tmin < t && t < tmax {
                let point = ray.at(t);

                let a = ((&self.p2 - &self.p1).cross(&(&point - &self.p1))).dot(&self.normal);
                let b = ((&self.p3 - &self.p2).cross(&(&point - &self.p2))).dot(&self.normal);
                let c =((&self.p1 - &self.p3).cross(&(&point - &self.p3))).dot(&self.normal);
                if a >= 0.0 && b >= 0.0 && c >= 0.0 {
                    return Some(HitRecord {
                        point,
                        t,
                        normal: (&self.normal).clone(),
                        is_outside: denom < 0.0,
                        material: Arc::clone(&self.material)
                    })
                }
            }
        }
        
        None
    }
}

pub struct Square {
    t1: Triangle,
    t2: Triangle,
}

impl Square {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, p4: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        // Side lengths
        assert!(((&p1 - &p2).length() - (&p2 - &p3).length()).abs() < 0.00000001, "Sides of a square must be equal");
        assert!(((&p2 - &p3).length() - (&p3 - &p4).length()).abs() < 0.00000001, "Sides of a square must be equal");
        assert!(((&p3 - &p4).length() - (&p4 - &p1).length()).abs() < 0.00000001, "Sides of a square must be equal");
        assert!(((&p4 - &p1).length() - (&p1 - &p2).length()).abs() < 0.00000001, "Sides of a square must be equal");

        // Angles
        assert!((&p1 - &p2).dot(&(&p2 - &p3)).abs() < 0.00000001, "Squares must have right angles");
        assert!((&p2 - &p3).dot(&(&p3 - &p4)).abs() < 0.00000001, "Squares must have right angles");
        assert!((&p3 - &p4).dot(&(&p4 - &p1)).abs() < 0.00000001, "Squares must have right angles");
        assert!((&p4 - &p1).dot(&(&p1 - &p2)).abs() < 0.00000001, "Squares must have right angles");

        let t1 = Triangle::new(p1.clone(), p2.clone(), p3.clone(), Arc::clone(&material));
        let t2 = Triangle::new(p1.clone(), p3.clone(), p4.clone(), Arc::clone(&material));
        Self {
            t1,
            t2
        }
    }
}

impl Hittable for Square {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.t1.hit(ray, tmin, tmax).or_else(|| self.t2.hit(ray, tmin, tmax))
    }
}