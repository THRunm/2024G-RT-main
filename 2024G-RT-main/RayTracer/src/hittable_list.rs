use std::sync::Arc;
use crate::AABB::Aabb;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
    pub bbox: Option<Aabb>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: None,
        }
    }

    pub fn set(object: Arc<dyn Hittable + Sync + Send>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object.clone());
        let obj_bbox = object.bounding_box().unwrap();
        self.bbox = match &self.bbox {
            Some(current_bbox) => Some(Aabb::surrounding_box(current_bbox.clone(), obj_bbox)),
            None => Some(obj_bbox),
        };
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = None;
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
        if hit_anything
        {
            rec.t = rec_temp.t;
            rec.u = rec_temp.u;
            rec.v = rec_temp.v;
            rec.p = rec_temp.p;
            rec.normal = rec_temp.normal;
            rec.front_face = rec_temp.front_face;
            rec.material = rec_temp.material;
        }
        return hit_anything;
    }
    fn bounding_box(&self) -> Option<Aabb> {
        return self.bbox;
    }

}