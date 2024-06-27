use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Debug, Copy, Clone)]
pub struct HitRecord{
    pub t:f64,
    pub p:Vec3,
    pub normal:Vec3,
}