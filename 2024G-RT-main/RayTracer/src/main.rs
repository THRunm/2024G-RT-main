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
mod image_;
mod perlin;


use std::sync::Arc;
use vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::texture::Texture;


const AUTHOR: &str = "name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}


fn random_world()-> HittableList {
    let mut world = HittableList::new();
    let checker = Texture::Checker(Box::new(texture::CheckerTexture::color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,material::Lambertian::set_texture(checker))));
    for a in -11..11{
        for b in -11..11 {
            let choose_mat = camera::random();
            let center = Vec3::new(a as f64 + 0.9 * camera::random(), 0.2, b as f64 + 0.9 * camera::random());
            if(center-Vec3::new(4.0,0.2,0.0)).length()>0.9
            {
                if choose_mat<0.8 {
                    let albedo = Vec3::elemul(Vec3::random(),Vec3::random());
                    let material = material::Lambertian::new(albedo);
                    let center2 = center+Vec3::new(0.0,camera::random()*0.5,0.0);
                    world.add(Arc::new(sphere::Sphere::set(center,center2,0.2,material)));
                }
                else if choose_mat<0.95 {
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
fn bouncing_spheres(path: &str) {
    let mut world = random_world();
    let world_=bvh::BvhNode::set(world);
    world= HittableList::set(Arc::new(world_));
    let vfov=20.0;
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.1;
    let focus_dist=10.0;
    let camera = camera::Camera::new(1200, 16.0/9.0,500,   vfov,lookfrom,lookat,vup,defocus_angle,focus_dist);
    let quality = 100;

    camera.render(world, path, quality);}
fn checkered_spheres(path: &str) {
    let mut world = HittableList::new();
    let checker = Texture::Checker(Box::new(texture::CheckerTexture::color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-10.0,0.0),10.0,material::Lambertian::set_texture(checker))));
    let checker = Texture::Checker(Box::new(texture::CheckerTexture::color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,10.0,0.0),10.0,material::Lambertian::set_texture(checker))));
    let vfov=20.0;
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.0;
    let focus_dist=10.0;
    let camera = camera::Camera::new(1200, 16.0/9.0,500,   vfov,lookfrom,lookat,vup,defocus_angle,focus_dist);
    let quality = 100;

    camera.render(world, path, quality);
}
fn earth(path: &str){
    let mut world = HittableList::new();
    let earth_texture = Texture::ImageTex(texture::ImageTexture::new("input/earthmap.png"));
    let earth_surface = material::Lambertian::set_texture(earth_texture);
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,0.0,0.0),2.0,earth_surface)));
    let vfov=20.0;
    let lookfrom = Vec3::new(0.0,0.0,12.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.0;
    let focus_dist=10.0;
    let camera = camera::Camera::new(800, 16.0/9.0,100,   vfov,lookfrom,lookat,vup,defocus_angle,focus_dist);
    let quality = 100;

    camera.render(world, path, quality);
}
fn perlin_spheres(path: &str) {
    let mut world = HittableList::new();
    let perlin_texture = Texture::Noise(texture::NoiseTexture::new(2.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,material::Lambertian::set_texture(perlin_texture))));
    let perlin_texture = Texture::Noise(texture::NoiseTexture::new(2.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,material::Lambertian::set_texture(perlin_texture))))  ;
    let vfov=20.0;
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.0;
    let focus_dist=10.0;
    let camera = camera::Camera::new(400, 16.0/9.0,100,   vfov,lookfrom,lookat,vup,defocus_angle,focus_dist);
    let quality = 100;
    camera.render(world, path, quality);
}
fn main() {
    let path = "output1/test9.png";
    let mode=4;
    match mode {
        1 => bouncing_spheres(path),
        2 => checkered_spheres(path),
        3 => earth(path),
        4 => perlin_spheres(path),
        _ => bouncing_spheres(path),
    }



}
