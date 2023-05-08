use std::default;

use super::area::Area;

pub struct  Rectangle {
    pub width:f64,
    pub x:f64,
    pub y:f64,
    pub height:f64
 }
 
 //Traits-similar to ts interface
 impl Area for Rectangle {
    fn area(&self)->f64{
        return self.width * self.height;
    }
}

//Create struct with defautl value.
impl Default for Rectangle {
    fn default()->Self{
        Rectangle{
            height:10.0,
            width:10.0,
            x:0.0,
            y:0.0
        }
    }
}