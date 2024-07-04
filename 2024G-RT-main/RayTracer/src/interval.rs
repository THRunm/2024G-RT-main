use std::ops::{Add, AddAssign, Neg};
#[derive(Clone, Copy)]
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
    pub fn expand(&self, delta: f64) -> Interval {
        Interval { min: self.min - delta/2.0, max: self.max + delta/2.0 }
    }
    pub fn intersect(lhs:&Interval,rhs: &Interval) -> Interval {
        Interval { min: lhs.min.min(rhs.min), max: lhs.max.max(rhs.max) }
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

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Interval {
        Interval { min: self.min + rhs, max: self.max + rhs }
    }
}