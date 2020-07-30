use crate::{vec::{Ray, Vec3}, ASPECT_RATIO, Point, util::*};

#[derive(Debug)]
pub struct Camera {
    origin: Point,
    target: Point,
    up: Vec3,
    lower_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
    fov: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    focus_dist: f64
}

impl Camera {
    pub fn new(origin: Point, target: Point, up: Vec3, fov: f64, apeture: f64, focus_dist: f64) -> Self {
        assert_ne!(origin, target, "Must not face the origin point");
        assert!(fov.abs() < 90.0, "Field of view must be less than 90 degrees");

        let up = up.normalize();

        let h = fov.to_radians().tan();
        let viewport_height = 2.0 * h;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let w = (&origin - &target).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left = &origin - &horizontal/2.0 - &vertical/2.0 - focus_dist * &w;
        let lens_radius = apeture/2.0;
        
        Self {
            origin: origin.clone(),
            lower_left,
            horizontal,
            vertical,
            target,
            up,
            fov,
            u,
            v,
            w,
            lens_radius,
            focus_dist
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_disk_vec(1.0);
        let offset = rd.0 * &self.u + rd.1 * &self.v;
        Ray::new(&(&self.origin + &offset), &(&self.lower_left + u*&self.horizontal + v*&self.vertical - &self.origin - offset))
    }

    pub fn set_facing(&mut self, target: Point) {
        assert_ne!(self.origin, target, "Must not face camera's origin");

        let h = self.fov.to_radians().tan();
        let viewport_height = 2.0 * h;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let w = (&self.origin - &target).normalize();
        let u = self.up.cross(&w);
        let v = w.cross(&u);
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left = &self.origin - &horizontal/2.0 - &vertical/2.0 - self.focus_dist * w;

        self.lower_left = lower_left;
        self.horizontal = horizontal;
        self.vertical = vertical;
        self.target = target;
    }

    pub fn set_origin(&mut self, origin: Point) {
        assert_ne!(self.target, origin, "Must not face camera's origin");

        let h = self.fov.to_radians().tan();
        let viewport_height = 2.0 * h;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let w = (&origin - &self.target).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);
        let horizontal = self.focus_dist * viewport_width * &u;
        let vertical = self.focus_dist * viewport_height * &v;
        let lower_left = &self.origin - &horizontal/2.0 - &vertical/2.0 - self.focus_dist * &w;

        self.lower_left = lower_left;
        self.horizontal = horizontal;
        self.vertical = vertical;
        self.origin = origin;
        self.w = w;
        self.u = u;
        self.v = v;
    }
}