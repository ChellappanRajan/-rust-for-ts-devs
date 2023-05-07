use std::{f64::consts::PI};
pub struct  Rectangle {
    pub width:f64,
    pub x:f64,
    pub y:f64,
    pub height:f64
 }
 
pub struct  Circle {
    pub radious:f64,
    pub x:f64,
    pub y:f64,
 }

 //Traits-similar to ts interface
pub trait Area {
      fn  area(&self)->f64;
}

 impl Area for Rectangle {
    fn area(&self)->f64{
        return self.width * self.height;
    }
}

impl Area for Circle{
    fn area(&self)->f64{
        return self.radious * self.radious * PI;
    }
}
