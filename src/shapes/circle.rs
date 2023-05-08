use core::fmt;
use std::{f64::consts::PI};

use super::area::Area;

#[derive(Debug)]
pub struct  Circle {
    pub radious:f64,
    pub x:f64,
    pub y:f64,
 }

impl Area for Circle{
    fn area(&self)->f64{
        return self.radious * self.radious * PI;
    }
}

impl Default for Circle {
    fn default()->Self{
        Circle{
            radious:2.0,
            x:0.0,
            y:0.0
        }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f:&mut fmt::Formatter)-> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}