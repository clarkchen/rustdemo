use super::*;

#[test]
pub fn test_area_circle() {
    let circle = Circle{radius: 2.0,..Default::default() };
    let result =  12.5664;
    // print_area(circle);
    println!("circle Area: {}", circle.area());
    // assert
    assert_eq!(circle.area(), result);

}

#[test]
pub fn test_rectangle_area(){
    let rectangle = Rectangle{height:10.0, width:50.0, ..Default::default() };
    let result = 10.0*50.0;
    println!("rectangle Area: {}", rectangle.area());
    // print_area(rectangle);

    assert_eq!(rectangle.area(), result)
}