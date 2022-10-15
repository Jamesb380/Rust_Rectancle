
#![allow(non_snake_case)]
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,

}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * self.width + 2 * self.height
    }
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(side:u32) -> Rectangle{
        Rectangle { width: side, height: side }
    }

}


fn main() {

    let sqr_pants = Rectangle{width: 30, height: 50};
   // let mr_cabs = Rectangle {width:40, ..sqr_pants};
    let patrick = Rectangle{width:60, height:80};
    println!("The are of the rectangle is {}. The {:?}", sqr_pants.area(), sqr_pants);
    println!("The perimeter is {}", sqr_pants.perimeter());
    println!();
    println!("Does patrick hold sqr_pants? {}",patrick.can_hold(&sqr_pants));
    println!("This is a square {:?}", Rectangle::square(5)); //For Associated functions
    



}

