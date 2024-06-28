use image::RgbImage;
use crate::interval::Interval;
use crate::vec3::Vec3;
pub fn linear_to_gamma(value: f64) -> f64 {
    if value >0.0
    {
        value.sqrt()
    }
    else
    {
        0.0
    }
}
/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: u32, j: u32) {
    // let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    // *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
    let intersity=Interval::set(0.0,0.99);
    let mut r:f64 = pixel_color.x ;
    let mut g:f64  = pixel_color.y ;
    let mut b:f64  = pixel_color.z ;
    r=linear_to_gamma(r);
    g=linear_to_gamma(g);
    b=linear_to_gamma(b);
    let rb = (256.0 *intersity.clamp(r)) as u8;
    let gb = (256.0 *intersity.clamp(g)) as u8;
    let bb = (256.0 *intersity.clamp(b)) as u8;
    img.put_pixel(i.try_into().unwrap(), j.try_into().unwrap(), image::Rgb([rb, gb , bb ]));
}
