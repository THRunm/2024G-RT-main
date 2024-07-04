use std::fs::File;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rayon::prelude::ParallelBridge;
use crate::hittable_list::HittableList;
use crate::{AUTHOR, is_ci, ray, vec3};
use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::vec3::Vec3;
use crate::ray::Ray;
use rayon::iter::ParallelIterator;


pub struct Camera{
    pub(crate) image_width:u32,
    pub(crate) aspect_ratio:f64,
    pub(crate) sample_per_pixel:u32,
    pub(crate) max_depth:i32,

    pub(crate) vfov:f64,

    pub(crate) lookfrom:Vec3,
    pub(crate) lookat:Vec3,
    pub(crate) vup:Vec3,

    pub(crate)  defocus_angle:f64,
    pub(crate) focus_dist:f64,

    pub(crate) pixel_samples_scale:f64,
    pub(crate) image_height:u32,
    pub(crate) center:Vec3,
    pub(crate) pixel100_loc:Vec3,
    pub(crate) pixel_delta_x:Vec3,
    pub(crate) pixel_delta_y:Vec3,
    pub(crate) u:Vec3,
    pub(crate) v:Vec3,
    pub(crate) w:Vec3,
    pub(crate) defocus_disk_x:Vec3,
    pub(crate) defocus_disk_y:Vec3,

    pub(crate) background:Vec3,
}
pub(crate) fn random() ->f64{
    let mut rng = StdRng::from_entropy();
    rng.gen_range(0.0..1.0)
}
impl Camera{
    pub fn new(image_width:u32, aspect_ratio:f64,sample_per_pixel:u32,            vfov:f64,
               lookfrom:Vec3,
               lookat:Vec3,
               vup:Vec3,defocus_angle:f64,
               focus_dist:f64,  ) ->Camera{
        let image_height = match (image_width as f64 / aspect_ratio) as u32>1  {
            true => (image_width as f64 / aspect_ratio) as u32,
            false => 1,
        };

        let center=lookfrom;
        let w=(lookfrom-lookat).unit();
        let u=Vec3::cross(vup,w).unit();
        let v=Vec3::cross(w,u);
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h*focus_dist;
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = u * viewport_width;
        let vertical = -v * viewport_height;
        let pixel_delta_x=horizontal/f64::from(image_width);
        let pixel_delta_y=vertical/f64::from(image_height);
        let lower_left_corner=center-(focus_dist*w)-(horizontal/2.0)-(vertical/2.0);
        let pixel100_loc=lower_left_corner+(pixel_delta_x+pixel_delta_y)*0.5;
        let pixel_samples_scale=1.0/f64::from(sample_per_pixel);
        let max_depth:i32=50;

        let defocus_radius=focus_dist*(defocus_angle.to_radians()/2.0).tan();
        let defocus_disk_x=u*defocus_radius;
        let defocus_disk_y=v*defocus_radius;
        let background=Vec3::new(0.7,0.8,1.0);
        Camera{
            image_width,
            aspect_ratio,
            sample_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            pixel_samples_scale,
            image_height,
            center,
            pixel100_loc,
            pixel_delta_x,
            pixel_delta_y,
            u,
            v,
            w,
            defocus_disk_x,
            defocus_disk_y,
            background,
        }
    }
    pub fn ray_color(&self,r: &Ray, depth:i32, world:&HittableList) -> vec3::Vec3 {
        if depth <= 0 {
            return vec3::Vec3::new(0.0, 0.0, 0.0);
        }
        let mut hit_record=HitRecord::new();
        if  world.hit(r, Interval::set(0.001, f64::INFINITY), &mut hit_record) {
           let color_from_emission=hit_record.material.emitted(hit_record.u,hit_record.v,&hit_record.p);
            if let Some((scattered, attenuation)) = hit_record.material.scatter(r, &hit_record) {
                return color_from_emission+Vec3::elemul( attenuation , self.ray_color(&scattered, depth - 1, world));
            }
            return color_from_emission;
        }
        return self.background
    }
    pub fn render(&self, world: HittableList, path:&str, quality:u8){
        let bar: ProgressBar = if is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
            let mut pixel_color = Vec3::new(1110.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let u = x as f64 + random();
                let v = y as f64 + random();
                let ray = self.get_ray(u, v);
                pixel_color += self.ray_color(&ray, self.max_depth, &world);
            }
            pixel_color *= self.pixel_samples_scale;
            let r = (255.99 * pixel_color.x.min(1.0).sqrt()) as u8;
            let g = (255.99 * pixel_color.y.min(1.0).sqrt()) as u8;
            let b = (255.99 * pixel_color.z.min(1.0).sqrt()) as u8;
            *pixel = image::Rgb([r, g, b]);
            bar.inc(1);
        });
        bar.finish();
        // 保存图像到文件
        let output_image = image::DynamicImage::ImageRgb8(img);
        let mut output_file = File::create(path).expect("Failed to create file");
        output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality))
            .expect("Failed to write image");
    }
    pub fn set_background(&mut self,background:Vec3){
        self.background=background;
    }
    pub fn sample_squre()->Vec3{
        Vec3::new(random()-0.5,random()-0.5,0.0)
    }
    pub fn defocus_disk_sample(&self)->Vec3{
        let p=Vec3::random_in_unit_disk();
        self.center+self.defocus_disk_x*p.x+self.defocus_disk_y*p.y
    }
    pub fn get_ray(&self,u:f64,v:f64)->ray::Ray{
        let offset=Self::sample_squre();
        let pixel_sample=self.pixel100_loc+((u+f64::from(offset.x))*self.pixel_delta_x)+((v+f64::from(offset.y))*self.pixel_delta_y);
        let ray_origin=if self.defocus_angle<=0.0{
            self.center}
        else {
            self.defocus_disk_sample()
        };
        let ray_direction=pixel_sample-self.center;
        let ray_time=random();
        Ray::new_time(ray_origin,ray_direction,ray_time)
        }
}