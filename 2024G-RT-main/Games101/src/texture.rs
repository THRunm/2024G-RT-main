#![allow(warnings)]
use nalgebra::{Vector3};

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    }
    fn vecn_to_vector3(vec: &VecN<u8, 3>) -> Vector3<f64> {
        Vector3::new(vec[0] as f64, vec[1] as f64, vec[2] as f64)
    }
    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        // 在此实现双线性插值函数, 并替换掉get_color
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let u_min = u_img.floor() as i32;
        let u_max = u_img.ceil().min(self.width as f64 - 1.0) as i32;
        let v_min = v_img.floor() as i32;
        let v_max = v_img.ceil().min(self.height as f64 - 1.0) as i32;

        let c1 = Texture::vecn_to_vector3(self.img_data.at_2d(v_min, u_min).unwrap());
        let c2 = Texture::vecn_to_vector3(self.img_data.at_2d(v_min, u_max).unwrap());
        let c3 = Texture::vecn_to_vector3(self.img_data.at_2d(v_max, u_min).unwrap());
        let c4 = Texture::vecn_to_vector3(self.img_data.at_2d(v_max, u_max).unwrap());

        let u_ratio = u_img - u_min as f64;
        let v_ratio = v_img - v_min as f64;

        let a = c1 * (1.0 - u_ratio) + c2 * u_ratio;
        let b = c3 * (1.0 - u_ratio) + c4 * u_ratio;
        let color = b * (1.0 - v_ratio) + a * v_ratio;

        Vector3::new(color[2], color[1], color[0])
    }
}