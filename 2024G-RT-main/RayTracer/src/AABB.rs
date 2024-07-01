use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive( Copy, Clone)]
pub(crate) struct aabb{
    pub(crate) x:Interval,
    pub(crate) y:Interval,
    pub(crate) z:Interval,
}

impl aabb {
    pub fn new(x:Interval,y:Interval,z:Interval)->aabb{
        aabb{
            x,
            y,
            z,
        }
    }
    pub fn set(a:Vec3,b:Vec3)->aabb{
        let x=if a.x>b.x{Interval::set(b.x,a.x)}else{Interval::set(a.x,b.x)};
        let y=if a.y>b.y{Interval::set(b.y,a.y)}else{Interval::set(a.y,b.y)};
        let z=if a.z>b.z{Interval::set(b.z,a.z)}else{Interval::set(a.z,b.z)};
        aabb{
            x,
            y,
            z,
        }
    }
    pub fn surrounding_box(box0:aabb,box1:aabb)->aabb{
        aabb{
            x:Interval::intersect(&box0.x,&box1.x),
            y:Interval::intersect(&box0.y,&box1.y),
            z:Interval::intersect(&box0.z,&box1.z),
        }
    }
    pub fn axis_interval(self,n:i32)->Interval{
        match n{
            0=>self.x,
            1=>self.y,
            2=>self.z,
            _=>Interval::new(),
        }
    }
    pub fn hit(&self,r:&Ray,ray_t:Interval)->bool{
        let ray_origin=r.origin;
        let ray_direction=r.direction;
        let mut ray_t=ray_t;
        for i in 0..3{
            let ax=self.axis_interval(i);
            let adinv=1.0/ray_direction[i.try_into().unwrap()];
            let mut t0=(ax.min-ray_origin[i.try_into().unwrap()])*adinv;
            let mut t1=(ax.max-ray_origin[i.try_into().unwrap()])*adinv;
            if (t0<t1){
                if(t0>ray_t.min){
                    ray_t.min=t0;
                }
                if(t1<ray_t.max){
                    ray_t.max=t1;
                }
            }
            else{
                if(t1>ray_t.min){
                    ray_t.min=t1;
                }
                if(t0<ray_t.max){
                    ray_t.max=t0;
                }
            }
            if ray_t.max<=ray_t.min{
                return false;
            }
        }
    true
    }
    pub fn longest_axis(&self)->i32{
        if self.x.size()>self.y.size(){
            if self.x.size()>self.z.size(){
                0
            }else{
                2
            }
        }else{
            if self.y.size()>self.z.size(){
                1
            }else{
                2
            }
        }
    }

}

