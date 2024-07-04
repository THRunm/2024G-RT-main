use num_traits::FloatConst;
use crate::AABB::Aabb;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;


pub struct HitRecord {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            u: 0.0,
            v: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: Box::new(crate::material::Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        }
    }
    pub(crate) fn set_face_normal(&mut self, ray: crate::ray::Ray, outward_normal: Vec3) {
        self.front_face = ray.direction * outward_normal < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval,rev:&mut HitRecord) -> bool;
    fn bounding_box(&self) -> Option<Aabb>;
}

pub struct Translate<Obj: Hittable> {
    pub(crate) offset: Vec3,
    pub(crate) obj: Obj,
    pub(crate) bbox: Aabb,
}

impl<Obj: Hittable> Translate<Obj> {
    pub fn new(obj: Obj, offset: Vec3) -> Self {
        let bbox = obj.bounding_box().unwrap() + offset;
        Self {
            offset,
            obj,
            bbox,
        }
    }
}

impl<Obj: Hittable> Hittable for Translate<Obj> {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool{
        let moved_ray = Ray::new_time(ray.origin - self.offset, ray.direction, ray.time);
        let hit = self.obj.hit(&moved_ray, ray_t,rec);
        if hit {
            rec.p += self.offset;
        }
        hit
    }
    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}

pub struct RotateY<Obj: Hittable> {
    pub(crate) obj: Obj,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) bbox: Aabb,
}

impl<Obj: Hittable> RotateY<Obj> {
    pub fn new(obj: Obj, angle: f64) -> Self {
        let radians = f64::PI() / 180.0 * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = obj.bounding_box().unwrap();
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.y.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        if tester[c] < min[c] {
                            min[c] = tester[c];
                        }
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }
                    }
                }
            }
        }
        let bbox = Aabb::set(min, max);
        Self {
            obj,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl<Obj: Hittable> Hittable for RotateY<Obj> {
    fn hit(&self, ray: &Ray, ray_t: Interval,rec:&mut HitRecord) ->bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;
        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;
        let r=Ray::new_time(origin,direction,ray.time);
        if !self.obj.bounding_box().unwrap().hit(&r,ray_t){
            return false;
        }
        let mut p:Vec3=rec.p;
        p.x=self.cos_theta*rec.p.x+self.sin_theta*rec.p.z;
        p.z=-self.sin_theta*rec.p.x+self.cos_theta*rec.p.z;
        let mut normal:Vec3=rec.normal;
        normal.x=self.cos_theta*rec.normal.x+self.sin_theta*rec.normal.z;
        normal.z=-self.sin_theta*rec.normal.x+self.cos_theta*rec.normal.z;
        rec.p=p;
        rec.normal=normal;
        true
    }
    fn bounding_box(&self) -> Option<Aabb> {
        return Some(self.bbox);
    }
}