use std::mem;
use std;

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
    let mut stack = 10;
    let heap = Box::new(stack);
    println!("Stack={}, Heap={}", stack, heap);
    println!("Size of stack {}, size of heap {}", mem::size_of_val(&stack), mem::size_of_val(&*heap));

    // scope and shadowing
    foo_function();
    stack = 21;
    println!("c={}", c);
    {
        let scoped_var = 4;
        println!("scoped_var = {}, stack = {}", scoped_var, stack);
    }

    let scoped_var = 15;
    println!("Redecleared scoped_var={} and stack={}", scoped_var, stack);

    // Operators
    /*
    Addition
    Multiplication
    Division 
    Modulo
    */
    let number = 2;
    let add_number = number + 5;
    println!("Addition={}", add_number);

    let multiply = number * 5;
    println!("Multiplication={}", multiply);

    let division = 8/2;
    println!("Division={}", division);

    let modulo = 5%2;
    println!("Modulo={}", modulo);

    let mut another_number = 7;
    another_number -= 2;
    println!("Operator={}", another_number);
    another_number += 5;
    println!("Operator={}", another_number);
    another_number *= 5;
    println!("Operator={}", another_number);
    another_number /= 5;
    println!("Operator={}", another_number);

    let cubed = i32::pow(number, 3);
    println!("Number raised to the power of 3 = {}", cubed);

    let cubed_times_pi = 8.0 * std::f64::consts::PI;
    println!("cubed * PI = {}", cubed_times_pi);

    let mut bool_result = 5 < 6;
    println!("bool_result = {}", bool_result);

    bool_result = 5 == 6;
    println!("bool_result = {}", bool_result)
}