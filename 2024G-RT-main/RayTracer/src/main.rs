mod color;
mod vec3;
mod ray;
mod sphere;

use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use vec3::Vec3;
use ray::Ray;


const AUTHOR: &str = "name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn hit_sphere(center:Vec3, radius:f64, ray: Ray) ->f64{
    let oc=center-ray.origin;
    let a=ray.direction.squared_length();
    let b=-2.0*oc*ray.direction;
    let c=oc.squared_length()-radius*radius;
    let discriminant=b*b-4.0*a*c;
    if discriminant<0.0{
         -1.0}
    else{
        (-b-f64::sqrt(discriminant))/(2.0*a)
    }

}

fn ray_color(r: ray::Ray) -> vec3::Vec3 {
    let t=hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5,r);
    if(t>0.0){
        let N=(r.at(t)-Vec3::new(0.0,0.0,-1.0)).unit();
        return Vec3::new(N.x+1.0,N.y+1.0,N.z+1.0)*0.5;
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
}
fn main() {
    let path = "output/test3.jpg";

    let aspect_ratio=16.0/9.0;
    let width = 400;
    let height = match (width as f64 / aspect_ratio) as u32>1  {
        true => (width as f64 / aspect_ratio) as u32,
        false => 1,
    };

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let camera_cemter=vec3::Vec3::new(0.0,0.0,0.0);

    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical=Vec3::new(0.0,-viewport_height,0.0);

    let pixel_delta_x=horizontal/f64::from(width);
    let pixel_delta_y=vertical/f64::from(height);

    let lower_left_corner=camera_cemter-horizontal/2.0-vertical/2.0-Vec3::new(0.0,0.0,focal_length);
    let pixel100_loc=lower_left_corner+(pixel_delta_x+pixel_delta_y)*0.5;

    let quality = 60;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(width, height);

    // 以下是write color和process bar的示例代码
    let pixel_color = [255u8; 3];
    for i in 0..width {
        for j in 0..height {
            let pixel_center=pixel100_loc+(i as f64*pixel_delta_x+j as f64*pixel_delta_y);
            let ray_direction=pixel_center-camera_cemter;
            let r=Ray::new(camera_cemter,ray_direction);
            let pixel_color_ = ray_color(r);
            let pixel_color = [
                (pixel_color_.x*259.99 ) as u8,
                (pixel_color_.y*259.99  ) as u8,
                (pixel_color_.z *259.99 ) as u8,
            ];
            write_color(pixel_color, &mut img, i, j);
            bar.inc(1);
        }
    }
    bar.finish();

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
