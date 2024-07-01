use crate::vec3::Vec3;
#[derive(Debug, Copy, Clone)]
pub(crate) struct Ray{
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) time: f64,
}
impl Ray{
    pub fn new(origin: Vec3, direction: Vec3)->Ray{
        Ray{
            origin,
            direction,
            time: 0.0,
        }
    }
    pub fn new_time(origin: Vec3, direction: Vec3, time: f64)->Ray{
        Ray{
            origin,
            direction,
            time,
        }
    }
    pub fn at(self,x:f64)->Vec3{
        self.origin+self.direction*x
    }

}