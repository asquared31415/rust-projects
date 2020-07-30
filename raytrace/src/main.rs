extern crate image;

use std::{sync::Arc, time::Instant};
use image::{math::utils::clamp, RgbImage};
use rayon::prelude::*;

mod camera;
mod materials;
mod objects;
mod util;
mod vec;

use camera::*;
use materials::*;
use objects::*;
use util::*;
use vec::*;
use rand::Rng;

const BG_COLOR_TOP: Color = Vec3(0.3, 0.5, 1.0);
const BG_COLOR_BOTTOM: Color = Vec3(1.0, 1.0, 1.0);
const MAX_DEPTH: i32 = 10;

fn ray_color(ray: &Ray, hittables: &Hittables, depth: i32) -> Color {
    if depth <= 0 {
        Vec3(0.0, 0.0, 0.0)
    } else {
        // A min of some small value helps to abvoid floating point errors causing fake hits
        match hittables.hit(ray, 0.0001, f64::INFINITY) {
            Some(hit) => {
                if let Some((attenuation, ray)) = hit.material().scatter(ray, &hit) {
                    attenuation * ray_color(&ray, hittables, depth - 1)
                } else {
                    Vec3(0.0, 0.0, 0.0)
                }
            },
            None => {
                let t = 0.5 * (ray.direction().normalize().1 + 1.0);
                let ret = lerp(BG_COLOR_BOTTOM, BG_COLOR_TOP, t);
                ret
            }
        }
    }
}

fn to_color(color: &Color) -> image::Rgb<u8> {
    let r = (256.0 * clamp((color.0 / SAMPLES_PER_PIXEL as f64).sqrt(), 0.0, 0.999)).floor() as u8;
    let g = (256.0 * clamp((color.1 / SAMPLES_PER_PIXEL as f64).sqrt(), 0.0, 0.999)).floor() as u8;
    let b = (256.0 * clamp((color.2 / SAMPLES_PER_PIXEL as f64).sqrt(), 0.0, 0.999)).floor() as u8;
    image::Rgb([r, g, b])
}

const SAMPLES_PER_PIXEL: i32 = 100;

const IMAGE_WIDTH: u32 = 1024;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const FOV_DEG: f64 = 20.0;
const APETURE: f64 = 0.1;
const ORIGIN: Vec3 = Vec3(13.0, 2.0, 3.0);
const TARGET: Vec3 = Vec3(0.0, 0.0, 0.0);

fn main() {
    let mut hittables = Hittables::new();
    // ground
    hittables.push(Box::new(Sphere::new(Vec3(0.0, -1000.0, -1.0), 1000.0, Arc::new(Lambertian::new(Vec3(0.3, 0.8, 0.2))))));
    
    hittables.push(Box::new(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    hittables.push(Box::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))))));
    hittables.push(Box::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)))));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(a as f64 + 1.9 * rand::random::<f64>(), 0.2, b as f64 + 1.9 * rand::random::<f64>());
            
            if (&center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: f64 = rand::random();

                if material < 0.8 {
                    hittables.push(Box::new(Sphere::new(center, 0.2, Arc::new(Lambertian::new(Color::random())))));
                } else if material < 0.95 {
                    let mut rng = rand::thread_rng();
                    hittables.push(Box::new(Sphere::new(center, 0.2, Arc::new(Metal::new(Color::rand_range(0.5, 1.0), rng.gen_range(0.0, 0.5))))));
                } else {
                    hittables.push(Box::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    let camera = Camera::new(ORIGIN, TARGET, Vec3(0.0, 1.0, 0.0), FOV_DEG, APETURE, 10.0);

    let start = Instant::now();
    println!("Starting raytracing...");
    let image_data: Vec<(u32, u32, Color)> = (0..IMAGE_HEIGHT).rev().collect::<Vec<u32>>().into_par_iter().map(|row| {
        (0..IMAGE_WIDTH).collect::<Vec<u32>>().into_par_iter().map(|col| {
            let color: Color = (0..SAMPLES_PER_PIXEL).collect::<Vec<i32>>().into_par_iter().map(|_| {
                let u = (col as f64 + rand::random::<f64>()) / (IMAGE_WIDTH) as f64;
                let v = (row as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT) as f64;
                ray_color(&camera.get_ray(u, v), &hittables, MAX_DEPTH)
            }).sum();
            (col, IMAGE_HEIGHT - row - 1, color)
        }).collect::<Vec<(u32, u32, Color)>>()
    }).flatten().collect();
    println!("Raytracing completed in {} seconds", start.elapsed().as_secs_f32());

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, color) in image_data.iter() {
        img.put_pixel(*x, *y, to_color(color));
    }
    img.save("./image.png").unwrap();
}
