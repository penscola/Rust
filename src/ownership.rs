fn no_ownership_change(data:i32) {
    println!("Hello: {}", data);
}

// no ownership change
// pub fn main() {
//     println!("-- No Ownership change --");
//
//     let data = 5;
//     let data2 = data;
//     println!("Data: {}", data);
//     println!("Data2: {}", data2);
//     no_ownership_change(data);
// }

// Ownership change
/*pub fn main() {
    println!("-- Ownership Change --");

    let v = vec![1,2,3];
    println!("Data: {}", v[0]);
    let v2 = v; /* Ownership movement*/
    println!("Data:{}", v2[0]);
    //println!("Data:{}", v[0]);
}*/

// last ownership example
fn print_sum(v:Vec<i32>) {
    println!("{}", v[0] + v[1]);
//     v is dropped and deallocated here
}

pub fn main() {
    println!("-- Ownership Change --");

    let mut v = Vec::new(); // creating the resource
    for i in 100..1000 {
        v.push(i);
    }

//     at this point, v is using
//     no less than 3600 bytes of memory, as one element = 4 bytes
    print_sum(v);
//     we no longer own nor have any control over v
//     it would be a compile-time error to try and access v here
    println!("We're done");
}
