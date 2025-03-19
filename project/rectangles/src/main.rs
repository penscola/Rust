use crate::area::area_rect::area_rectangle;

mod area;

fn main() {
    let rect1 = (30, 50);

    println!("the area of the Rectangle is {} square pixels", area_rectangle(rect1));
}
