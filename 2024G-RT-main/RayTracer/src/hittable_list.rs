use std::sync::Arc;
use crate::AABB::Aabb;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;

pub struct HittableList {
    pub objects:Vec<Arc<dyn Hittable>>,
    pub bbox:Option<Aabb>,
}

impl HittableList {
    pub fn new()-> HittableList {
        HittableList {
            objects:Vec::new(),
            bbox:None,
        }
    }
    pub fn set(objects:Arc<dyn Hittable>)-> HittableList {
        let mut list= HittableList::new();
        list.add(objects);
        list
    }

    pub fn add(&mut self,object:Arc<dyn Hittable>){
        self.objects.push(object.clone());
        if self.bbox.is_none(){
            self.bbox= Option::from(object.bounding_box().unwrap());
    }
        else {
            self.bbox=Option::from(Aabb::surrounding_box(self.bbox.unwrap(), object.bounding_box().unwrap()));
        }
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool{
        let mut rec_temp=HitRecord::new();
        let mut hit_anything=false;
        let mut closest_so_far=ray_t.max;
        for object in self.objects.iter(){
            let mut temp_rec=HitRecord::new();
            if object.hit(ray,Interval::set(ray_t.min(),closest_so_far),&mut temp_rec){
                hit_anything=true;
                closest_so_far=temp_rec.t;
                rec_temp=temp_rec;
            }
        }
        rec.t=rec_temp.t;
        rec.u=rec_temp.u;
        rec.v=rec_temp.v;
        rec.p=rec_temp.p;
        rec.normal=rec_temp.normal;
        rec.front_face=rec_temp.front_face;
        rec.material=rec_temp.material;
        return hit_anything;
    }
    fn bounding_box(&self) -> Option<Aabb> {
        return self.bbox;
    }

}