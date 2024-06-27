use std::ops::Neg;

pub struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}
impl Interval {
    pub fn new() -> Interval {
        Interval { min:-f64::INFINITY, max:f64::INFINITY }
    }
    pub fn set(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self,x:f64)->bool
    {
        x>self.min && x<self.max
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
    pub fn empty() -> Interval {
        Interval { min: f64::INFINITY, max: -f64::INFINITY }
    }
    pub fn universe() -> Interval {
        Interval { min: -f64::INFINITY, max: f64::INFINITY }
    }
    pub fn clamp(&self,x:f64)->f64{
        if x<self.min{
            self.min
        }
        else if x>self.max{
            self.max
        }
        else{
            x
        }
    }
}

