use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;

pub struct Sphere<mat:Material>{
    pub center:Vec3,
    pub radius:f64,
    pub material:mat,
}
impl<mat:Material> Sphere<mat>{
    pub fn new(center:Vec3, radius:f64, material: mat) ->Self{
        let radius=match radius>0.0{
            true=>radius,
            false=>0.0,
        };
        Sphere{
            center,
            radius,
            material,
        }
    }
}
impl<mat:Material> Hittable for Sphere<mat>{
    fn hit(&self, ray: &Ray, ray_t:Interval) -> Option<HitRecord>{
        let oc=self.center-ray.origin;
        let a=ray.direction.squared_length();
        let h=oc*ray.direction;
        let c=oc.squared_length()-self.radius*self.radius;
        let discriminant=h*h-a*c;
        if discriminant<0.0{
            return None;
        }
        let sqrtd=f64::sqrt(discriminant);
        let root = (h-sqrtd)/a;
        if(!ray_t.surrounds(root)){
            let root = (h+sqrtd)/a;
            if(!ray_t.surrounds(root)){
                return None;
            }
        }
        let t=root;
        let p=ray.at(t);
        let normal=(p-self.center)/self.radius;
        let material=&self.material;
        let mut rec=HitRecord{
            t,
            p,
            normal,
            front_face:true,
            material,
        };
        rec.set_face_normal(*ray,normal);
        Some(rec)
    }
}