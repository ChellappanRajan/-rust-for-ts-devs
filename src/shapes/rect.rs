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
