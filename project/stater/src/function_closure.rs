/* Function */

fn function_1() {
    let x = 3 + 5;
    println!("3 + 5 = {}", x);
}

fn function_2(name:String) {
    println!("Hello, {}", name);
}

fn sum_of_two_numbers(x:i32, y:i32) {
    println!("{} + {} = {}", x, y, x+y);
}

pub fn main() {
    println!("-- Function & Closure --");
    function_1();
    function_2("Felix Kiprotich".to_string());
    sum_of_two_numbers(7, 8);

//     Closure
    let var_minus_1 = |x:i32| x-1;
    println!("{}", var_minus_1(5));

    let plus_two = |x| {
        let mut result:i32 = x;
        result += 1;
        result += 2;
        result
    };

    println!("{}", plus_two(5));
}
