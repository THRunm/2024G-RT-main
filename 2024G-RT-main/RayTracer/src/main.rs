mod color;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;


use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use vec3::Vec3;
use ray::Ray;
use crate::hittable::Hittable;
use crate::hittable_list::Hittable_List;
use crate::interval::Interval;


const AUTHOR: &str = "name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

// fn hit_sphere(center:Vec3, radius:f64, ray: Ray) ->f64{
//     let oc=center-ray.origin;
//     let a=ray.direction.squared_length();
//     let b=oc*ray.direction;
//     let c=oc.squared_length()-radius*radius;
//     let discriminant=b*b-a*c;
//     if discriminant<0.0{
//          -1.0}
//     else{
//         (b-f64::sqrt(discriminant))/a
//     }
//
// }


fn main() {
    let path = "output/test5.jpg";



    let mut world =Hittable_List::new();

    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5)));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0)));

    let camera = camera::Camera::new(400, 16.0/9.0,100);

    let quality = 100;

    camera.render(world, path, quality);
}
