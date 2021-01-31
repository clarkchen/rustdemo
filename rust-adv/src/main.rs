// use shapes::Shape;
// use lights::TrafficLight;
mod shapes;
mod sum_list;
mod lights;

use shapes::{Shape, Circle, Rectangle};
use lights::{TrafficLight};
use sum_list::{sum_list};

fn print_area<T: Shape>(shape: T) {
    println!("This {} has an area of {}", shape.shape_name(), shape.area());
}

fn main() {
    // traffice lights
    let y = TrafficLight::Yellow;
    let r = TrafficLight::Red;
    let g = TrafficLight::Green;
    format!("yellow time is {y}, green time is {g}, red time is {r}", y=y.time(),r=r.time(), g=g.time());

    //sum list
    println!("normal sum is {}", sum_list(&[1,2,3,5]).unwrap());
    println!("abnormla result is None {}", (sum_list::sum_list(&[1, u32::MAX]) == None));

    //shape
    let circle = Circle{radius: 50.0,..Default::default() };
    print_area(circle);
    let rectangle = Rectangle{height:10.0, width:50.0, ..Default::default() };
    print_area(rectangle);

    println!("Please Run Cargo Test")

}

