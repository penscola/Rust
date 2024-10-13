pub fn main2() {
    let x = false;
    if x {
        println!("Yeah, we're true");
    } else {
        println!("Boo, we are false")
    }

    let y = 4;
    if y == 4 {
        println!("We have a 4");
    } else if y == 3 {
        println!("We have a 3");
    }
    else {
        println!("We have something else other than 3 or 4");
    }

    if (x==false) || (y==5) {
        println!("Inside multi conditional");
    } else {
        println!("multi conditional not met")
    }

    // loops
    let mut x = 5;
    let mut keep_looping = true;
    println!("Beginning of while loop");
    while keep_looping {
        x += x-3;
        println!("{}", x);
        if x % 5 == 0 {
            keep_looping = false;
        }
    }

    // For Loop
    println!("Beginning of For loop");
    for x in 0..20 {
        println!("{}", x);
    }

    // Match statment
    println!("Beginning of Match statments");
    let x = "Apple";
    match x {
        "Apple" => println!("Great brand!"),
        "Linux" => println!("Open source!"),
        _ => println!("Some other OS system"),
    }

    let area_code = 206;
    let area = match area_code {
        206 => "Seattle",
        318 => "Louisana",
        200..300 => "Washington state",
        _ => "Invalid",
    };
    println!("The area for {} is {}", area_code, area)
}