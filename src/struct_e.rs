struct Student {
    age:i32,
    mark:i32,
}

#[derive(Debug)]
enum Language {
    Java,
    Scala,
}

fn checked_division(dividend:i32, divisor:i32) -> Option<i32> {
    if divisor == 0{
        None
    } else {
        Some(dividend/dividend)
    }
}

fn try_division(dividend:i32, divisor:i32) {
    match checked_division(dividend, divisor) {
        None => println!("{}/{} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{}/{} = {}", dividend, divisor, quotient)
        }
    }
}

pub fn main() {
    let student:Student = Student { age:23, mark: 17 };
    println!("Student information: age {}, mark: {}", student.age, student.mark);

    let Student { age:my_age, mark:my_mark } = student;
    println!("Information gathered: {} - {}", my_age, my_mark);

    let first_languages = Language::Java;
    println!("{:?}", first_languages);

    let second_langauge = Language::Scala;
    println!("{:?}", second_langauge);

    // Option<T>
    let number = Some(7);
    let letter: Option<i32> = Some(5);

    if let Some(i) = number {
        println!("Matched {:?}", i);
    } else {
        println!("Didn;t matched a number.");
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn;t matched a number.");
    }

    // call Option<T> functions
    try_division(4, 3);
    try_division(4, 0);

    // Arrays
    let my_array = [1,2,3];
    println!("Here are elements of my array: {:?}", my_array);
    let mut my_mutable_array = [1,2,3];
    println!("Here are elements of my mutable array: {:?}", my_mutable_array);
    println!("Element one = {}", my_array[2]);

    // looping through arrays
    let mut index = 0;
    for i in my_array.iter() {
        println!("@ index = {}, i = {}", index, i);
        index = index + 1;
    }

    // creating fixed arrays
    let array_two = [4;5];
    println!("{:?}", array_two);
    for i in 0..array_two.len() {
        println!("{}", array_two[i]);
    }
}