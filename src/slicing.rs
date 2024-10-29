pub fn main(){
    println!("--- Slicing ---");

    let vec = vec![1,2,3];
    let int_slice = &vec[0..3];
    let str_slice: &[&str] = &["one", "two", "three"];

    println!("vec={:?}", vec);
    println!("int_slice={:?}", int_slice);
    println!("str_slice={:?}", str_slice);
    
    assert_eq!(str_slice.len(), 3);
    assert!(!str_slice.is_empty());

    // String
    // let my_string = "Hello there!";
    let mut s = "Hello".to_string();
    println!("s={:?}", s);
    s.push_str(", world.");
    println!("String with append: {}", s);
    // println!("There first letter of s is {}", s[0]);

    // concatenation
    let hello = "Hello ".to_string();
    let world = "world.".to_string();
    let hello_world = hello + &world;
    println!("{}", hello_world);

    // Multiple lines
    let multi_string = "Hey
    Brett";
    println!("{}",multi_string);
}