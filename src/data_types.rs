use std::mem;

fn foo_function() {
    let y = 5;
    println!("foo_function() is being called. Result is {}", y)
}

pub fn data_types() {
    // data types
    let bool_variable = true;
    println!("My bool variable value is {}", bool_variable);

    let bool_two:bool = false;
    println!("My bool variable value is {}", bool_two);
    println!("Size of bool is {} bytes", mem::size_of_val(&bool_two));

    let c: char = 'c';
    println!("Size of char is {} bytes", mem::size_of_val(&c));
    let number: i32 = 42;
    println!("Size of i32 is {} bytes", mem::size_of_val(&number));
    let double_number: f64 = 1.0;
    println!("Size of f64 is {} bytes", mem::size_of_val(&double_number));
    let u_number: u32 = 8000;
    println!("Size of u32 is {} bytes", mem::size_of_val(&u_number));
    let my_string: &str  = "Hello, World!";
    println!("Size of string is {} bytes", mem::size_of_val(&my_string));

    println!("c={}, number={}, double_number={}, Unsigned number={}, my_string={}", 
    c, number, double_number, u_number, my_string);

    // stack and heap
    foo_function();
    let stack = 10;
    let heap = Box::new(stack);
    println!("Stack={}, Heap={}", stack, heap);
    println!("Size of stack {}, size of heap {}", mem::size_of_val(&stack), mem::size_of_val(&*heap))
}