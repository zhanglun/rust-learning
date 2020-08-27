fn main () {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_truple(rect1)
    );

    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect2)
    );

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let sq = Rectangle::square(3) ;

    println!("sq is {:?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_truple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}