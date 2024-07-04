use crate::AABB::Aabb;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use num_traits::float::FloatConst;

pub struct Sphere<Mat: Material+ Sync + Send> {
    pub center1: Vec3,
    pub radius: f64,
    pub material: Mat,
    pub is_moving: bool,
    pub center_vec: Vec3,
    pub bbox: Aabb,
}

impl<Mat: Material + Clone + Sync + Send+ 'static> Sphere<Mat> {
    pub fn new(center: Vec3, radius: f64, material: Mat) -> Self {
        let radius = if radius > 0.0 { radius } else { 0.0 };
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center1: center,
            radius,
            material,
            is_moving: false,
            center_vec: Vec3::new(0.0, 0.0, 0.0),
            bbox: Aabb::set(center - rvec, center + rvec),
        }
    }

    pub fn set(center1: Vec3, center2: Vec3, radius: f64, material: Mat) -> Self {
        let radius = if radius > 0.0 { radius } else { 0.0 };
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::set(center1 - rvec, center1 + rvec);
        let box2 = Aabb::set(center2 - rvec, center2 + rvec);
        let bbox = Aabb::surrounding_box(box1, box2);
        Sphere {
            center1,
            radius,
            material,
            is_moving: true,
            center_vec: center2 - center1,
            bbox,
        }
    }

    pub fn sphere_center(&self, time: f64) -> Vec3 {
        if self.is_moving {
            self.center1 + self.center_vec * time
        } else {
            self.center1
        }
    }

    pub fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + f64::PI();
        let u = phi / (2.0 * f64::PI());
        let v = theta / f64::PI();
        (u, v)
    }
}

impl<Mat: Material + Clone+ Sync + Send + 'static> Hittable for Sphere<Mat> {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = self.sphere_center(ray.time);
        let oc = ray.origin - center;
        let a = ray.direction.squared_length();
        let half_b = oc * ray.direction;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - center) / self.radius;
        let (u, v) = Sphere::<Mat>::get_sphere_uv((p - center) / self.radius);

        rec.t = t;
        rec.p = p;
        rec.u = u;
        rec.v = v;
        rec.material = Box::new(self.material.clone());
        rec.normal = normal;
        rec.front_face = true;
        rec.set_face_normal(*ray, normal);

        true
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
