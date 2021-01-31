
pub trait Shape {
    fn area(&self) -> f64;
    // fn name: String;
    fn shape_name(&self) -> String;
}



const PI: f64 = 3.1416;


pub struct Circle{
    pub name: String,
    pub(crate) radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn shape_name(&self)->String{
        self.name.clone()
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 2.0,
            name: String::from("circle"),
        }
    }
}

pub struct Rectangle {
    pub height: f64,
    pub width: f64,
    pub name: String

}

impl Shape for Rectangle {
    fn area(&self) -> f64{
        self.height * self.width
    }
    fn shape_name(&self)->String{
        self.name.clone()
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        Self {
            height: 10.0,
            width: 5.0,
            name: String::from("Rectangle"),
        }
    }
}





// https://dev.to/ahmednoor/implementing-polymorphism-in-rust-4b3f
// https://doc.rust-lang.org/reference/items/traits.html
// https://github.com/tinydjp/RustDemos/blob/main/src/area.rs

#[cfg(test)]
mod tests;

