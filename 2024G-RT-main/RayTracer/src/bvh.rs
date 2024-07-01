use crate::AABB::aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::Hittable_List;
use crate::interval::Interval;

pub struct BVH_Node{
    pub left:Box<dyn Hittable>,
    pub right:Box<dyn Hittable>,
    pub bbox:aabb,
}

impl BVH_Node{
    pub fn new(objects:&Vec<Box<dyn Hittable>>,start:usize,end:usize)->BVH_Node{
        let mut bbox=aabb::new(Interval::empty(),Interval::empty(),Interval::empty());
        for i in start..end{
            let temp_bbox=objects[i].bounding_box().unwrap();
            bbox=aabb::surrounding_box(bbox,temp_bbox);
        }
        let axis=bbox.longest_axis();
        let comparator=match axis{
            0=>BVH_Node::box_x_compare,
            1=>BVH_Node::box_y_compare,
            2=>BVH_Node::box_z_compare,
            _=>BVH_Node::box_x_compare,
        };
        let object_span=end-start;
        if object_span==1{
            return BVH_Node{
                left: objects[start].clone(),
                right:objects[start].clone(),
                bbox:objects[start].bounding_box().unwrap(),
            }
        }
        else if object_span==2 {
            return BVH_Node {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bbox: aabb::surrounding_box(objects[start].bounding_box().unwrap(), objects[start + 1].bounding_box().unwrap()),

            }
        }
        else {
            let mut objects=objects;
            objects.sort_by(|a,b|if comparator(a,b){std::cmp::Ordering::Less}else{std::cmp::Ordering::Greater});
            let mid=start+object_span/2;
            let left=BVH_Node::new(objects.clone(),start,mid);
            let right=BVH_Node::new(objects.clone(),mid,end);
            let left_box=left.bbox;
            let right_box=right.bbox;
            return BVH_Node{
                left:Box::from(left),
                right:Box::from(right),
                bbox:aabb::surrounding_box(left_box,right_box),
    }
    }
    }
    pub fn set(lise:Hittable_List)->BVH_Node{
        let mut objects=lise.objects;
        let size=objects.len();
        BVH_Node::new(&objects,0,size)
    }
    pub fn box_compare(a:&Box<dyn Hittable>,b:&Box<dyn Hittable>,axis:usize)->bool{
        let box_a=a.bounding_box().unwrap();
        let box_b=b.bounding_box().unwrap();
        return match axis{
            0=>box_a.x.min<box_b.x.min,
            1=>box_a.y.min<box_b.y.min,
            2=>box_a.z.min<box_b.z.min,
            _=>false,
        }
    }
    pub fn box_x_compare(a:&Box<dyn Hittable>,b:&Box<dyn Hittable>)->bool{
        BVH_Node::box_compare(a,b,0)
    }
    pub fn box_y_compare(a:&Box<dyn Hittable>,b:&Box<dyn Hittable>)->bool{
        BVH_Node::box_compare(a,b,1)
    }
    pub fn box_z_compare(a:&Box<dyn Hittable>,b:&Box<dyn Hittable>)->bool{
        BVH_Node::box_compare(a,b,2)
    }

}
impl Hittable for BVH_Node{
    fn hit(&self, ray: &crate::ray::Ray, ray_t:Interval) -> Option<HitRecord> {
        if(!self.bbox.hit(ray,ray_t)){
            return None;
        }
        let left_hit=self.left.hit(ray,ray_t);
        let right_hit=self.right.hit(ray,Interval::set(ray_t.min(),if left_hit.is_none(){ray_t.max()}else{left_hit.unwrap().t}));
        return match (left_hit,right_hit){
            (None,None)=>None,
            (Some(left),None)=>Some(left),
            (None,Some(right))=>Some(right),
            (Some(left),Some(right))=>if left.t<right.t{Some(left)}else{Some(right)},
        }
    }
    fn bounding_box(self) -> Option<aabb> {
        return Option::from(self.bbox);
    }

}