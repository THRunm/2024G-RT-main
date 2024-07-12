use std::sync::Arc;
use crate::AABB::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Triangle<Mat: Material + Sync + Send> {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub mat: Mat,
    pub normal: Vec3,
    pub bbox: Aabb,
}

impl<Mat: Material + Sync + Send + Clone + 'static> Triangle<Mat> {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, mat: Mat) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = Vec3::cross(edge1, edge2).unit();
        let bbox = Aabb::surrounding_box(
            Aabb::set(v0, v1),
            Aabb::set(v0, v2),
        );
        Triangle {
            v0,
            v1,
            v2,
            mat,
            normal,
            bbox,
        }
    }
}

impl<Mat: Material + Sync + Send + Clone + 'static> Hittable for Triangle<Mat> {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = Vec3::cross(ray.direction, edge2);
        let a =edge1* h;

        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s* h;

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = Vec3::cross(s, edge1);
        let v = f * ray.direction* q;

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f *edge2* q;
        if !ray_t.contains(t) {
            return false;
        }

        rec.t = t;
        rec.p = ray.at(t);
        rec.u = u;
        rec.v = v;
        rec.material = Box::new(self.mat.clone());
        rec.normal = if ray.direction*self.normal < 0.0 { self.normal } else { -self.normal };
        rec.front_face = ray.direction* rec.normal < 0.0;
        true
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
