struct Student {
    age:i32,
    mark:i32,
}

#[derive(Debug)]
enum Language {
    Java,
    Scala,
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
    let letter: Option<i32> = None;

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
}