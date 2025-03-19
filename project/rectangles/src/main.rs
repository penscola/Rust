use crate::area::area_rect::area_rectangle;

mod area;

struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the Rectangle is {} square pixels", area_rectangle(&rect1));
}
