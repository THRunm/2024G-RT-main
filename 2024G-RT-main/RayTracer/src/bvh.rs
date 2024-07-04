use std::sync::Arc;
use crate::AABB::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &[Arc<dyn Hittable>], start: usize, end: usize) -> BvhNode {
        let mut bbox = Aabb::new(Interval::empty(), Interval::empty(), Interval::empty());
        for i in start..end {
            let temp_bbox = objects[i].bounding_box().unwrap();
            bbox = Aabb::surrounding_box(bbox, temp_bbox);
        }
        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            2 => BvhNode::box_z_compare,
            _ => BvhNode::box_x_compare,
        };
        let object_span = end - start;
        if object_span == 1 {
            return BvhNode {
                left: Arc::clone(&objects[start]),
                right: Arc::clone(&objects[start]),
                bbox: objects[start].bounding_box().unwrap(),
            };
        } else if object_span == 2 {
            return BvhNode {
                left: Arc::clone(&objects[start]),
                right: Arc::clone(&objects[start + 1]),
                bbox: Aabb::surrounding_box(objects[start].bounding_box().unwrap(), objects[start + 1].bounding_box().unwrap()),
            };
        } else {
            let mut sorted_objects = objects.to_vec();
            sorted_objects[start..end].sort_by(|a, b| if comparator(a, b) { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater });
            let mid = start + object_span / 2;
            let left = BvhNode::new(&sorted_objects, start, mid);
            let right = BvhNode::new(&sorted_objects, mid, end);
            let bbox= Aabb::surrounding_box(left.bbox, right.bbox);
            return BvhNode {
                left: Arc::new(left),
                right: Arc::new(right),
                bbox,
            };
        }
    }    pub fn set(lise: HittableList) -> BvhNode {
        let objects=lise.objects;
        let size=objects.len();
        BvhNode::new(&objects, 0, size)
    }
    pub fn box_compare(a:&Arc<dyn Hittable>,b:&Arc<dyn Hittable>,axis:usize)->bool{
        let box_a=a.bounding_box().unwrap();
        let box_b=b.bounding_box().unwrap();
        return match axis{
            0=>box_a.x.min<box_b.x.min,
            1=>box_a.y.min<box_b.y.min,
            2=>box_a.z.min<box_b.z.min,
            _=>false,
        }
    }
    pub fn box_x_compare(a:&Arc<dyn Hittable>,b:&Arc<dyn Hittable>)->bool{
        BvhNode::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a:&Arc<dyn Hittable>,b:&Arc<dyn Hittable>)->bool{
        BvhNode::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a:&Arc<dyn Hittable>,b:&Arc<dyn Hittable>)->bool{
        BvhNode::box_compare(a, b, 2)
    }

}
impl Hittable for BvhNode {
    fn hit(&self, ray: &crate::ray::Ray, ray_t:Interval,rec:&mut HitRecord) -> bool{
        if !self.bbox.hit(ray,ray_t) {
            return false;
        }
        let left_hit=self.left.hit(ray,ray_t,rec);
        let right_hit=self.right.hit(ray,Interval::set(ray_t.min(),if !left_hit{ray_t.max()}else{rec.t}),rec);
        return left_hit||right_hit;

    }
    fn bounding_box(&self) -> Option<Aabb> {
        return Option::from(self.bbox);
    }

}