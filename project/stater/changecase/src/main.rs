use crate::to_uppercase::uppercase::change_to_uppercase;

mod to_uppercase;

fn main() {
    let lowercase = change_to_uppercase("felix");
    println!("{}", lowercase);
}
