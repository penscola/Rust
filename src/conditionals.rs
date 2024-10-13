pub fn main2() {
    let x = true;
    if x {
        println!("Yeah, we're true");
    } else {
        println!("Boo, we are false")
    }

    let y = 5;
    if y == 4 {
        println!("We have a 4");
    } else if y == 3 {
        println!("We have a 3");
    }
    else {
        println!("We have something else other than 3 or 4");
    }
}