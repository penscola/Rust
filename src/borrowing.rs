
// without Borrowing
/*
Right now we used the variable and returned it back so that the original owner
get its data back
Here we are not getting the reference but instead the original data on the heap
 */
fn print_mul1(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[0] * v[1]);
    return v;
}

/*
With borrowing explicit references
Here we have the reference, so now more than one variable is pointing to the same data on the heap
Two variables reference the same data which is vr and v(main)
 */
fn print_mul2(vr: &Vec<i32>) {
    println!("{}", (*vr)[0] * (*vr)[1]);
}

// this is how you should do it
fn print_mul3(v: &Vec<i32>) {
    println!("{}", v[0] * v[1]);
    // same asin print_mul2
}

fn count_occurrences(v2: &Vec<i32>, val:i32) -> usize {
    v2.into_iter().filter(|&&x| x == val).count()
}

pub fn main() {
    println!("-- Borrowing--");

    let mut v = Vec::new(); //creating the resource
    for i in 100..1000 {
        v.push(i);
    }
    // at this point, v is using not less than 3600 bytes of memory

    // transfer ownership to print_mul() and get it back after they're done
    v = print_mul1(v);
    
    // now we again own and control v
    println!("(1) We still have v: {}, {}, ...", v[0], v[1]);
    
    //Take a reference to v(borrow it) and pass this reference to print_mul2
    print_mul2(&v);
    // v is still completely ours
    println!("(2) We still have v: {}, {}, ...", v[0], v[1]);
    
    //some as in above example
    print_mul3(&v);
    println!("(3) We still have v: {}, {}, ...", v[0], v[1]);
    
    println!("\nAnother Example");
    let v2 = vec![2,9,3,1,3,2,5,5,2];
    // borrowing v2 for iteration and item is the reference
    for &item in &v2 {
        //the first borrow is still active
        //we borrow it the second time here!
        let res = count_occurrences(&v, item);
        println!("{} -> {}", item, res);
    }
}