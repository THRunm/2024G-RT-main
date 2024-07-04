use std::sync::Arc;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::vec3::Vec3;

pub struct ConstantMedium <Mat: Material+ Sync + Send>{
    pub(crate) boundary: Arc<dyn Hittable+ Sync + Send>,
    pub(crate) phase_function: Mat,
    pub(crate) neg_inv_density: f64,
}
impl<Mat: Material+ Sync + Send> ConstantMedium<Mat>{
    pub fn new(boundary: Arc<dyn Hittable+ Sync + Send>, density: f64, phase_function: Mat) -> Self {
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        }
    }

}

impl <Mat: Material + Clone+ Sync + Send + 'static>Hittable for ConstantMedium<Mat> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut crate::hittable::HitRecord) -> bool {
        let enableDebug = false;
        let debugging = enableDebug && crate::camera::random() < 0.00001;

        let mut rec1 = crate::hittable::HitRecord::new();
        let mut rec2 = crate::hittable::HitRecord::new();

        if !self.boundary.hit(r, Interval::universe(), &mut rec1)
        { return false; }

        if !self.boundary.hit(r, Interval::set(rec1.t+0.0001, f64::INFINITY), &mut rec2)
        { return false; }


        if rec1.t < ray_t.min { rec1.t = ray_t.min; }
        if rec2.t > ray_t.max{ rec2.t = ray_t.max; }

        if rec1.t >= rec2.t
        { return false; }

        if rec1.t < 0.0
        { rec1.t = 0.0; }

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density *crate::camera::random().log(10.0);

        if hit_distance > distance_inside_boundary
        { return false; }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);


        rec.normal = Vec3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.material =Box::new(self.phase_function.clone());

        return true;
    }
    fn bounding_box(&self) -> Option<crate::AABB::Aabb> {
        self.boundary.bounding_box()
    }
}
