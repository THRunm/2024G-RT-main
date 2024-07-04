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
mod quad;


use std::sync::Arc;
use vec3::Vec3;
use crate::hittable::{RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{DiffuseLight, Lambertian};
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
fn quads(path:&str){
    let mut world = HittableList::new();
    let left_red =Lambertian::new(Vec3::new(1.0, 0.2, 0.2));
    let back_green =Lambertian::new(Vec3::new(0.2, 1.0, 0.2));
    let right_blue =Lambertian::new(Vec3::new(0.2, 0.2, 1.0));
    let upper_orange =Lambertian::new(Vec3::new(1.0, 0.5, 0.0));
    let lower_teal =Lambertian::new(Vec3::new(0.2, 0.8, 0.8));

    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(-3.0,-2.0,5.0),Vec3::new(0.0,0.0,-4.0),Vec3::new(0.0,4.0,0.0),left_red)));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(-2.0,-2.0,0.0),Vec3::new(4.0,0.0,0.0),Vec3::new(0.0,4.0,0.0),back_green)));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(3.0,-2.0,1.0),Vec3::new(0.0,0.0,4.0),Vec3::new(0.0,4.0,0.0),right_blue)));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(-2.0,3.0,1.0),Vec3::new(4.0,0.0,0.0),Vec3::new(0.0,0.0,4.0),upper_orange)));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(-2.0,-3.0,5.0),Vec3::new(4.0,0.0,0.0),Vec3::new(0.0,0.0,-4.0),lower_teal)));
    let vfov=80.0;
    let lookfrom = Vec3::new(0.0,0.0,9.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.0;
    let focus_dist=10.0;
    let camera = camera::Camera::new(400, 16.0/9.0,100,   vfov,lookfrom,lookat,vup,defocus_angle,focus_dist);
    let quality = 100;
    camera.render(world, path, quality);
}
fn simple_light(path: &str) {
    let mut world = HittableList::new();
    let pertext = Texture::Noise(texture::NoiseTexture::new(4.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,material::Lambertian::set_texture(pertext))));
    let pertext = Texture::Noise(texture::NoiseTexture::new(4.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,material::Lambertian::set_texture(pertext))));
    let difflight = material::DiffuseLight::set_color(Vec3::new(4.0,4.0,4.0));
    world.add(Arc::new(quad::quad::<DiffuseLight>::new(Vec3::new(3.0,1.0,-2.0),Vec3::new(2.0,0.0,0.0),Vec3::new(0.0,2.0,0.0),difflight)));
    let difflight = material::DiffuseLight::set_color(Vec3::new(4.0,4.0,4.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,7.0,0.0),2.0,difflight)));
    let vfov=20.0;
    let lookfrom = Vec3::new(26.0,3.0,6.0);
    let lookat = Vec3::new(0.0,2.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.0;
    let focus_dist=10.0;
    let mut camera = camera::Camera::new(400, 16.0/9.0, 100, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist);
    let quality = 100;
    camera.set_background(Vec3::new(0.0,0.0,0.0));
    camera.render(world, path, quality);
}
fn cornell_box(path:&str){
    let mut world = HittableList::new();


    let red = Lambertian::new(Vec3::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Vec3::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::set_color(Vec3::new(15.0, 15.0, 15.0));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0,00.0), Vec3::new(0.0, 0.0,555.0), green)));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(0.0, 0.0, 0.0),Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0),  red)));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white)));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0,0.0), white)));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));
    world.add(Arc::new(quad::quad::<Lambertian>::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0,-555.0),white)));
    world.add(Arc::new(quad::quad::<DiffuseLight>::new(Vec3::new(343.0, 554.8, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light)));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));

    let  box1=quad::quad::bx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,330.0,165.0),white);
    let box1=RotateY::new(box1,15.0);
   let box1=Translate::new(box1,Vec3::new(265.0,0.0,295.0));
    world.add(Arc::new(box1));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));

    let  box2=quad::quad::bx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,165.0,165.0),white);
    let box2=RotateY::new(box2,-18.0);
    let box2=Translate::new(box2,Vec3::new(130.0,0.0,65.0));
    world.add(Arc::new(box2));
    let vfov=40.0;
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);

    let defocus_angle=0.0;
    let focus_dist=10.0;
    let mut camera = camera::Camera::new(600, 1.0, 10, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist);
    let quality = 100;
    camera.set_background(Vec3::new(0.0,0.0,0.0));
    camera.render(world, path, quality);
}
fn main() {
    let path = "output1/test17.png";
    let mode=7;
    match mode {
        1 => bouncing_spheres(path),
        2 => checkered_spheres(path),
        3 => earth(path),
        4 => perlin_spheres(path),
        5 => quads(path),
        6 => simple_light(path),
        7 => cornell_box(path),
        _ => bouncing_spheres(path),
    }



}
