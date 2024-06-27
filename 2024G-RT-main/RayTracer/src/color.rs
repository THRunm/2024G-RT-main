use image::RgbImage;
use crate::interval::Interval;
use crate::vec3::Vec3;
/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: u32, j: u32) {
    // let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    // *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
    let intersity=Interval::set(0.0,0.99);
    let r:f64 = pixel_color.x ;
    let g:f64  = pixel_color.y ;
    let b:f64  = pixel_color.z ;
    let rb = (256.0 *intersity.clamp(r)) as u8;
    let gb = (256.0 *intersity.clamp(g)) as u8;
    let bb = (256.0 *intersity.clamp(b)) as u8;
    img.put_pixel(i.try_into().unwrap(), j.try_into().unwrap(), image::Rgb([rb, gb , bb ]));
}
