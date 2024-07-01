use crate::AABB::aabb;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;

pub struct Sphere<mat:Material>{
    pub center1:Vec3,
    pub radius:f64,
    pub material:mat,
    pub is_moving:bool,
    pub center_vec:Vec3,
    pub bbox:aabb,

}
impl<mat:Material> Sphere<mat>{
    pub fn new(center:Vec3, radius:f64, material: mat) ->Self{
        let radius=match radius>0.0{
            true=>radius,
            false=>0.0,
        };
        let rvec=Vec3::new(radius,radius,radius);
        Sphere{
            center1:center,
            radius,
            material,
            is_moving:false,
            center_vec:Vec3::new(0.0,0.0,0.0),
            bbox:aabb::set(center-rvec,center+rvec),
        }
    }
    pub fn set(center1:Vec3,center2:Vec3,radius:f64,material:mat)->Self{
        let radius=match radius>0.0{
            true=>radius,
            false=>0.0,
        };
        let rvec=Vec3::new(radius,radius,radius);
        let box1=aabb::set(center1-rvec,center1+rvec);
        let box2=aabb::set(center2-rvec,center2+rvec);
        let bbox=aabb::surrounding_box(box1,box2);
        Sphere{
            center1,
            radius,
            material,
            is_moving:true,
            center_vec:center2-center1,
            bbox,
        }
    }
    pub fn sphere_center(&self,time:f64)->Vec3{
        if self.is_moving{
            return self.center1+self.center_vec*time;
        }
        self.center1
    }
}
impl<mat:Material> Hittable for Sphere<mat>{
    fn hit(&self, ray: &Ray, ray_t:Interval) -> Option<HitRecord>{
        let center=self.sphere_center(ray.time);
        let oc=center-ray.origin;
        let a=ray.direction.squared_length();
        let h=oc*ray.direction;
        let c=oc.squared_length()-self.radius*self.radius;
        let discriminant=h*h-a*c;
        if discriminant<0.0{
            return None;
        }
        let sqrtd=f64::sqrt(discriminant);
        let root = (h-sqrtd)/a;
        if root==0.0{
            return None;
        }
        if(!ray_t.surrounds(root)){
            let root = (h+sqrtd)/a;
            if(!ray_t.surrounds(root)){
                return None;
            }
        }
        let t=root;
        let p=ray.at(t);
        let normal=(p-center)/self.radius;
        let material=&self.material;
        let mut rec=HitRecord{
            t,
            p,
            u:0.0,
            v:0.0,
            normal,
            front_face:true,
            material,
        };
        rec.set_face_normal(*ray,normal);
        Some(rec)
    }
    fn bounding_box(&self) -> Option<aabb> {
        Some(self.bbox)
    }
}