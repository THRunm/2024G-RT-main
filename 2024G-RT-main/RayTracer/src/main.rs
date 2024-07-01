mod color;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;
mod AABB;
mod bvh;
mod texture;



use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use std::sync::Arc;
use vec3::Vec3;
use ray::Ray;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::Hittable_List;
use crate::interval::Interval;
use crate::texture::Texture;


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
fn random_world()->Hittable_List{
    let mut world =Hittable_List::new();
    let checker = Texture::Checker(Box::new(texture::CheckerTexture::color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,material::Lambertian::set_texture(checker))));
    for a in -11..11{
        for b in -11..11 {
            let choose_mat = camera::random();
            let center = Vec3::new(a as f64 + 0.9 * camera::random(), 0.2, b as f64 + 0.9 * camera::random());
            if(center-Vec3::new(4.0,0.2,0.0)).length()>0.9
            {
                if(choose_mat<0.8){
                    let albedo = Vec3::elemul(Vec3::random(),Vec3::random());
                    let material = material::Lambertian::new(albedo);
                    let center2 = center+Vec3::new(0.0,camera::random()*0.5,0.0);
                    world.add(Arc::new(sphere::Sphere::set(center,center2,0.2,material)));
                }
                else if(choose_mat<0.95){
                    let albedo = Vec3::random()*0.5+Vec3::new(0.5,0.5,0.5);
                    let fuzz = camera::random()*0.5;
                    let material = material::Metal::new(albedo,fuzz);
                    world.add(Arc::new(sphere::Sphere::new(center,0.2,material)));
                }
                else{
                    let material = material::Dielectric::new(1.5);
                    world.add(Arc::new(sphere::Sphere::new(center,0.2,material)));
                }
            }

        }
        }
    let material1 = material::Dielectric::new(1.5);
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,material1)));
    let material2 = material::Lambertian::new(Vec3::new(0.4,0.2,0.1));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0,material2)));
    let material3 = material::Metal::new(Vec3::new(0.7,0.6,0.5),0.0);
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(4.0,1.0,0.0),1.0,material3)));
    world


}

fn main() {
    let path = "output1/test3.png";



    let mut world = random_world();
    let world_=bvh::BVH_Node::set(world);
    world=Hittable_List::set(Arc::new(world_));
    let vfov=20.0;
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.1;
    let focus_dist=10.0;
    let camera = camera::Camera::new(400, 16.0/9.0,50,   vfov,lookfrom,lookat,vup,defocus_angle,focus_dist);
    let quality = 100;

    camera.render(world, path, quality);
}
