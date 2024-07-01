use crate::AABB::aabb;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::material::Material;
#[derive( Copy, Clone)]
pub struct HitRecord<'a>{
    pub t:f64,
    pub u:f64,
    pub v:f64,
    pub p:Vec3,
    pub normal:Vec3,
    pub front_face:bool,
    pub material:&'a dyn Material,
}

impl<'a> HitRecord<'a>{
    pub(crate) fn set_face_normal(&mut self, ray:crate::ray::Ray, outward_normal:Vec3) {
        self.front_face = ray.direction * outward_normal < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hittable:{
    fn hit(&self,ray:&crate::ray::Ray,ray_t:Interval)->Option<HitRecord>;
    fn bounding_box(&self)->Option<aabb>;
}