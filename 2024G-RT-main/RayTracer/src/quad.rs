use std::sync::Arc;
use crate::AABB::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct quad <Mat: Material+ Sync + Send> {
    pub(crate) q: Vec3,
    pub(crate) u: Vec3,
    pub(crate) v: Vec3,
    pub(crate) w: Vec3,
    pub(crate) mat: Mat ,
    pub(crate) bbox: Aabb,
    pub(crate) d: f64,
    pub(crate) normal: Vec3,
}

impl<Mat: Material + Clone + Sync + Send+ 'static>   quad<Mat> {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Mat) -> Self {
        let n = Vec3::cross(u, v);
        let normal = n.unit();
        let d = q * normal;
        let w = n / (n * n);
        let bbox1 = Aabb::set(q, q + u + v);
        let bbox2 = Aabb::set(q + u, q + v);
        let bbox = Aabb::surrounding_box(bbox1, bbox2);
        quad {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            d,
            normal,
        }
    }

    pub fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::set(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }
        Some((a, b))
    }
    pub fn bx(a:Vec3,b:Vec3,mat:Mat)->  HittableList{
        let mut sides=HittableList::new();
        let min=Vec3::new(a.x.min(b.x),a.y.min(b.y),a.z.min(b.z));
        let max=Vec3::new(a.x.max(b.x),a.y.max(b.y),a.z.max(b.z));
        let dx=Vec3::new(max.x-min.x,0.0,0.0);
        let dy=Vec3::new(0.0,max.y-min.y,0.0);
        let dz=Vec3::new(0.0,0.0,max.z-min.z);
        sides.add(Arc::new(quad::new(Vec3::new(min.x,min.y,max.z),dx,dy,mat.clone())));
        sides.add(Arc::new(quad::new(Vec3::new(max.x,min.y,max.z),-dz,dy,mat.clone())));
        sides.add(Arc::new(quad::new(Vec3::new(min.x,min.y,min.z),dz,dy,mat.clone())));
        sides.add(Arc::new(quad::new(Vec3::new(max.x,min.y,min.z),-dx,dy,mat.clone())));
        sides.add(Arc::new(quad::new(Vec3::new(min.x,max.y,max.z),dx,-dz,mat.clone())));
        sides.add(Arc::new(quad::new(Vec3::new(min.x,min.y,min.z),dx,dz,mat.clone())));
        sides
    }
}

impl<Mat: Material + Clone + Sync + Send+ 'static>Hittable for quad<Mat> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal * r.direction;
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - self.normal * r.origin) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let p = r.at(t);
        let planar_hitpt_vector = p - self.q;
        let alpha = self.w * Vec3::cross(planar_hitpt_vector, self.v);
        let beta = self.w * Vec3::cross(self.u, planar_hitpt_vector);
        let (a, b) = match quad::<Mat>::is_interior(alpha, beta) {
            Some((a, b)) => (a, b),
            None => return false,
        };
        rec.t = t;
        rec.p = p;
        rec.u = a;
        rec.v = b;
        rec.material =  Box::new(self.mat.clone());
        rec.normal = self.normal;
        rec.front_face = true;
        rec.set_face_normal(*r, self.normal);
        true
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
