use crate::vec3::Vec3;
use crate::interval::Interval;
#[derive(Debug, Copy, Clone)]
pub struct HitRecord{
    pub t:f64,
    pub p:Vec3,
    pub normal:Vec3,
    pub front_face:bool,
}

impl HitRecord{
    pub(crate) fn set_face_normal(&mut self, ray:crate::ray::Ray, outward_normal:Vec3) {
        self.front_face = ray.direction * outward_normal < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}
pub trait Hittable{
    fn hit(&self,ray:crate::ray::Ray,ray_t:Interval)->Option<HitRecord>;
}