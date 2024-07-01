use crate::AABB::aabb;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;

pub struct Hittable_List{
    pub objects:Vec<Box<dyn Hittable>>,
    pub bbox:Option<aabb>,
}

impl Hittable_List{
    pub fn new()->Hittable_List{
        Hittable_List{
            objects:Vec::new(),
            bbox:None,
        }
    }
    pub fn add(&mut self,object:Box<dyn Hittable>){
        self.objects.push(object.clone());
        if self.bbox.is_none(){
            self.bbox= Option::from(object.bounding_box().unwrap());
    }
        else {
            self.bbox=Option::from(aabb::surrounding_box(self.bbox.unwrap(),object.bounding_box().unwrap()));
        }
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }
}

impl Hittable for Hittable_List{
    fn hit(&self, ray: &Ray, ray_t:Interval) -> Option<HitRecord> {
        let mut rec:Option<HitRecord>=None;
        let mut closest_so_far=ray_t.max();
        for object in self.objects.iter(){
            if let Some(temp_rec)=object.hit(ray,Interval::set(ray_t.min(),closest_so_far)){
                closest_so_far=temp_rec.t;
                rec=Some(temp_rec);
            }
        }
        rec
    }
    fn bounding_box(self) -> Option<aabb> {
        return self.bbox;
    }

}